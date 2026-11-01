# Command reference

The binary is `tquil`. Run `tquil <command> --help` for full options.

## Global flags

These work on every command.

| Flag | Description |
|------|-------------|
| `-v`, `-q` | Raise / lower verbosity (repeatable) |
| `--json` | Output the envelope as JSON |
| `--format <auto\|text\|markdown\|json\|jsonl\|plain>` | Payload format |
| `-p`, `--plain` | Same as `--format plain` |
| `--dry-run` | Simulate actions without making changes |
| `--color <auto\|always\|never>` | Color policy |
| `-C`, `--cwd <DIR>` | Run against another directory |
| `--ci` | Non-interactive, assume yes |
| `--non-interactive` | Non-interactive, assume no |
| `-y`, `--yes` | Accept defaults |
| `--force` | Force actions that would normally be prevented |
| `--config <FILE>` | Use a config file from a non-default location |

## Commands

| Command | Description |
|---------|-------------|
| `tquil info` | Show system information and status |
| `tquil doctor` | Diagnose the local environment |
| `tquil doctor fix` | Attempt safe local repairs (try `--dry-run` first) |
| `tquil logs` | View logs (`--tail N`, `--date YYYY-MM-DD`, `--level`, `--json-only`, `--path`) |

### Lists

`tquil list` has `apps`, `fonts`, `vps` and `categories` subcommands.

### Applications

```sh
tquil app list
tquil app categories
tquil app install [--all] [--server] [--dry-run]
tquil app uninstall
```

### Fonts

```sh
tquil font list
tquil font install --name FiraCode,Hack    # or --all
tquil font update
tquil font uninstall
```

### Config

`TARGET` is `config` (default), `applications` or `vps`.

```sh
tquil config validate --file path.yaml [TARGET]
tquil config override --file path.yaml [TARGET]
tquil config reset [TARGET]
```

### VPS profiles

```sh
tquil vps schema                       # example profile
tquil vps add NAME --host example.com
tquil vps list
tquil vps connect [--id ID]            # omit --id to pick interactively
tquil vps update --id ID --name NEW
tquil vps script [--id ID] --inline 'uptime'
tquil vps copy --id ID --source SRC --destination DEST
tquil vps delete [--id ID]
```
