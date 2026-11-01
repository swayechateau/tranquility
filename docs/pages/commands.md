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
- `vps add` only prompts for the id (interactive, and only when `--id` is not given): the generated id is shown pre-filled so you can accept or edit it. The user defaults to the current OS user and the port to 22.
- Errors are printed as `E<code>: message` with context on stderr, and the process exits non-zero.
- `--dry-run` is honored by `app`, `font`, `config reset|override` and every `vps` command that changes state.
- `--json`, `--format json|jsonl` and `-p` disable color. `-p` falls back to text for tables.
- `vps connect`, `script` and `copy` add `-o BatchMode=yes` to ssh/scp when not interactive, so a password or host-key prompt fails instead of hanging.
- `vps connect` runs the profile's `post_connect_script` (set with `--post-connect-script` on `add`/`update`) and then leaves you in a login shell.
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

Each profile has a friendly **name** (e.g. `Temp Server`) and a unique one-word **id** (e.g. `tempserver`). The id is generated from the name when not given (`--id` on add), made unique with a number suffix if needed, and can be changed with `vps update --new-id` (alias `--rename-to`). Ids are lowercase letters, digits, `-` or `_`. `--id` on every other command selects a profile by id or name.

```sh
tquil vps schema                       # JSON Schema for the vps file, with an example
tquil vps add --host example.com [NAME] [--id ID] [--user U] [--port P] [--private-key FILE] [--post-connect-script CMD] [--jump-host SPEC]
tquil vps list
tquil vps connect [--id ID] [--timeout SECS]   # omit --id to pick interactively
tquil vps update                       # pick a profile, then edit each field (pre-filled)
tquil vps update --id ID --name NEW    # non-interactive: --id plus at least one change flag
tquil vps update --id ID --new-id NEW  # change the id
tquil vps script [--id ID] --inline 'uptime' [--timeout SECS]
tquil vps script --all --inline 'uptime'       # run on every profile; continues past failures
tquil vps copy --id ID --source SRC --destination DEST [--direction push|pull|remote] [--timeout SECS]
tquil vps copy --all --source SRC --destination DEST          # copy to/from every profile
tquil vps copy --id FROM --to-id TO --source SRC --destination DEST   # directly between two profiles (scp -3)
tquil vps ping [--id ID | --all] [--timeout SECS]   # check reachability only, never prompts
tquil vps export [--output FILE]       # dump all profiles (stdout, or a file)
tquil vps import --file FILE [--merge] # replace, or merge skipping duplicate ids
tquil vps delete [--id ID]
```

- **ProxyJump:** `--jump-host user@bastion[:port]` on `add`/`update` routes `connect`/`script`/`copy` through that host via ssh's `-J`.
- **Host keys:** every ssh/scp call uses `StrictHostKeyChecking=accept-new` — a host is trusted the first time, but a later change to its key is still rejected (unlike disabling the check entirely).
- **Timeouts:** `--timeout` becomes `ConnectTimeout` for `connect`, `script`, `copy` and `ping`; `ping` defaults to 5s. It bounds the connection attempt, not how long a remote command itself takes to finish.
- **SSH keys:** only key-based auth is supported non-interactively (`--private-key`, or the default identity/agent). A passphrase-protected key with no agent running will fail fast under `--ci`/`--non-interactive` rather than hang, since those add `BatchMode=yes`.
