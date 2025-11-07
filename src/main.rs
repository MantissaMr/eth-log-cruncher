
// IMPORTS
use std::path::Path;
use std::process;
use std::fs::File;
use std::io::{self, BufRead};
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use chrono::{DateTime, Datelike, Local, NaiveDateTime};
use std::collections::HashMap;
use serde::{Serialize};
use indicatif::{ProgressBar, ProgressStyle};

//DATA STRUCTURES 
#[derive(Debug, Serialize)]
struct LogEntry {
    level: String,
    timestamp: DateTime<Local>,
    message: String,
    details: HashMap<String, String>,
    // more fields to be added as needed 
}

//GLOBAL VARIABLES
lazy_static! {
    // Regex pattern to match log lines
    static ref LOG_REGEX: Regex = Regex::new(
        r"^(?P<level>INFO|WARN|ERROR|DEBUG|TRACE)\s*\[(?P<timestamp>.+?)\]\s+(?P<message>.*)"
    ).unwrap();

    // Specialized Regex to parse key-value pairs
    static ref KV_REGEX: Regex = Regex::new(r#"(?P<key>\w+)=(?P<value>"[^"]*"|\S+)"#).unwrap();
}

// CLI DEFINITIONS
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The path to the log file to be processed
    log_file_path: String,

    /// Optional year for log timestamps (default: current year)
    #[arg(long)]
    year: Option<i32>,
}


// PARSING LOGIC
fn parse_line(line: &str, year: i32) -> Option<LogEntry> {
    
    // Core parsing logic 
    LOG_REGEX.captures(line).and_then(|caps| {
            /*
        Extract the raw tmestamp string from the regex capture, prepend the year,
        and use chrono to parse the timestamp into a DateTime<Local> object.
                            */
        let raw_timestamp_str = &caps["timestamp"];
        let with_year = format!("{}-{}", year, raw_timestamp_str);
        let naive_dt = NaiveDateTime::parse_from_str(&with_year, "%Y-%m-%d|%H:%M:%S%.f").ok()?;
        let local_dt = naive_dt.and_local_timezone(Local).single()?;

        // Extract other fields under message
        let message = caps["message"].to_string();
        let mut details = HashMap::new();
        for kv_caps in KV_REGEX.captures_iter(&message) {
            let key = kv_caps["key"].to_string(); 
            let mut value = kv_caps["value"].to_string();

            if value.starts_with('"') && value.ends_with('"') {
                value = value.trim_matches('"').to_string();
            }

            details.insert(key, value);
        }
        

        Some(LogEntry{
            level: caps["level"].to_string(),
            timestamp: local_dt,
            message: caps["message"].to_string(),
            details,

        })
    })
}

// ENTRY POINT
fn main() {
    // Get the user's instructions
    let cli_args = Cli::parse();

    // Pass those instructions to the main `run` function and handle any top-level errors
    if let Err(e) = run(cli_args){
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}

// WORKFLOW LOGIC
fn run(args: Cli) -> Result<(), Box<dyn std::error::Error>> {
    // eprintln!("Processing log file at path: {}", args.log_file_path);
 
    let path = Path::new(&args.log_file_path);
    
    // Validation processes
    if !path.exists() {
        return Err(format!("Error: File not found at path '{}'", args.log_file_path).into());
    }

    if !path.is_file() {
        return Err(format!("Error: The path '{}' is a directory, not a file", args.log_file_path).into());
    }

        // Determine the year to use for log entries
    let year = args.year.unwrap_or_else(|| Local::now().year());
    eprintln!("Using year: {}", year);

   //  eprint!("Pre-scanning file to count lines...");
    let total_lines = io::BufReader::new(File::open(path)?).lines().count();

        // progress bar setup
    let pb = ProgressBar::new(total_lines as u64);
    pb.set_style(ProgressStyle:: default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) - {msg}")?
        .progress_chars("#>-"));
    pb.set_message("Scanning log file...");
    
        // Open the file for reading
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    
    let mut valid_line_count = 0;

    // Main processing loop
    for line_result in pb.wrap_iter(reader.lines()) {
        let line = line_result?;

        if let Some(log_entry) = parse_line(&line, year){
            valid_line_count += 1;

            let json_string = serde_json::to_string(&log_entry)?;
            println!("{}", json_string); 

        }
    }

    pb.finish_with_message("Scan complete!");
    // Summary output
    eprint!("\n");
    eprintln!("Run Summary");
    eprintln!("---------------------");
    eprintln!("Total Lines Processed: {}", total_lines);
    eprintln!("Valid Log Entries Found: {}", valid_line_count);
    eprintln!("---------------------");

    Ok(())
}