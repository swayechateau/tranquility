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

## Interactive and non-interactive use

Prompts only appear when stdin and stderr are terminals and neither `--ci` nor `--non-interactive` is set.

- `font install` / `font uninstall` without `--name` or `--all` show a picker; non-interactively they fail with an error.
- `vps connect|script|update|delete|copy` without `--id` show a picker; non-interactively `--id` is required (a single profile is selected automatically).
- `vps delete` asks for confirmation; pass `--yes` or `--ci` to skip it. With `--non-interactive` and no `--yes` it fails.
- `vps add` never prompts: the user defaults to the current OS user and the port to 22.
- Errors are printed as `E<code>: message` with context on stderr, and the process exits non-zero.
- `--dry-run` is honored by `app`, `font`, `config reset|override` and every `vps` command that changes state.
- `--json`, `--format json|jsonl` and `-p` disable color. `-p` falls back to text for tables.
- `-C` and `--config` must point at an existing directory / file.

## Commands

| Command | Description |
|---------|-------------|
| `tquil info` | Show system information and status |
| `tquil doctor` | Diagnose the local environment |
| `tquil doctor fix` | Attempt safe local repairs (try `--dry-run` first) |
| `tquil logs` | View logs (`--tail N`, `--date YYYY-MM-DD`, `--level info\|warn\|error`, `--json-only`, `--path`) |

### Lists

`tquil list` has `apps`, `fonts`, `vps` and `categories` subcommands.

### Applications

```sh
tquil app list
tquil app categories
tquil app install [--all] [--server] [--dry-run]   # --all/--server work before or after the subcommand
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
tquil vps add NAME --host example.com [--user U] [--port P] [--private-key FILE]
tquil vps list
tquil vps connect [--id ID]            # omit --id to pick interactively
tquil vps update --id ID --name NEW
tquil vps script [--id ID] --inline 'uptime'
tquil vps copy --id ID --source SRC --destination DEST
tquil vps delete [--id ID]
```
