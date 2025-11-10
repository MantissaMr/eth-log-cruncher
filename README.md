# Geth Log Cruncher

A command-line tool for converting unstructured Ethereum Geth logs into machine-readable JSONL. It parses raw log files into structured data suitable for analysis, database ingestion, or processing with other CLI tools like jq.

## Installation

tk

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
Because the tool outputs JSONL to `stdout`, you can filter on the fly. Example: show only `ERROR`-level logs:

```bash
geth-log-cruncher /path/to/your/geth.log | jq 'select(.level == "ERROR")'
```

### Example pipeline
Write parsed output to a file, then count ERRORs:

```bash
geth-log-cruncher /var/log/geth.log > /tmp/geth.jsonl
jq -r '.level' /tmp/geth.jsonl | grep -c ERROR
```

## Output format
Typical fields:

* `timestamp` — ISO 8601 timestamp (reconstructed using `--year` when needed).  
* `level` — log level when present (e.g., `INFO`, `WARN`, `ERROR`).  
* `target` — the module or subsystem that emitted the log (when available).  
* `message` — the raw log message text.  
* `details` — an object of parsed KV pairs extracted from the message (flexible and sparse).

## Examples
A sample parsed line might look like:

```json
{
  "timestamp": "2023-07-01T12:34:56Z",
  "level": "ERROR",
  "target": "eth/downloader",
  "message": "failed to download block",
  "details": {
    "block": "0xabc123",
    "peer": "12D3K..."
  }
}
```

## Contributing
PRs and issues welcome. If you add parsers for more Geth subsystems or improve performance, please open an issue first to discuss the approach. 

## License
This project is licensed under the **MIT License**.
```