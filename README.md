# Geth Log Cruncher

A command-line tool for converting unstructured Ethereum Geth logs into machine-readable JSONL. It parses raw log files into structured data suitable for analysis, database ingestion, or processing with other CLI tools like `jq`.

## Installation

Install directly from crates.io using Cargo:

```bash
cargo install geth-log-cruncher
```

## Usage

### Basic example

```bash
geth-log-cruncher /path/to/your/geth.log > parsed_logs.jsonl
```

### Specifying a year

For archived log files where the timestamp year isn't present or isn't the current year, use `--year`:

```bash
geth-log-cruncher /path/to/archive/geth-2023.log --year 2023 > parsed_2023_logs.jsonl
```

### Filtering with jq

Example: show only `DEBUG`-level logs:

```bash
geth-log-cruncher /path/to/your/geth.log | jq 'select(.level == "DEBUG")'
```

### Example pipeline

Write parsed output to a file, then count INFO logs:

```bash
geth-log-cruncher /var/log/geth.log > /tmp/geth.jsonl
jq '.level' /tmp/geth.jsonl | grep -c '"INFO"'
```

## Output format

Typical fields:

* `timestamp` — ISO 8601 timestamp (reconstructed using `--year` when needed).  
* `level` — log level when present (e.g., `INFO`, `WARN`, `DEBUG`).  
* `message` — the raw log message text.  
* `details` — an object of parsed KV pairs extracted from the message (flexible and sparse).


A sample parsed line might look like:

```json
{
  "timestamp": "2023-07-01T12:34:56Z",
  "level": "DEBUG",
  "message": "failed to download block",
  "details": {
    "block": "0xabc123",
    "peer": "12D3K..."
  }
}
```

## Contributing

PRs and issues are welcome. If you add parsers for more Geth subsystems or improve performance, please open an issue first to discuss the approach.

## License

This project is licensed under the **MIT License**.