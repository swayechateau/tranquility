
```bash
tranquility logs [OPTIONS]
```

| Option                | Description                              |
| --------------------- | ---------------------------------------- |
| `--json-only`         | Show only JSON-formatted lines           |
| `--path`              | Print only the log file path             |
| `--tail <N>`          | Show the last N log lines (default: 50)  |
| `--date <YYYY-MM-DD>` | Filter logs by date                      |
| `--level <level>`     | Filter logs by level (info, warn, error) |


* [ ] Add `--strict` mode (warnings = errors)