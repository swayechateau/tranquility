# Examples

Sample `config`, `applications` and `vps` files in TOML (the default), YAML and JSON.
Each one is checked with `tquil config validate`:

```bash
tquil config validate config --file example/config.toml
tquil config validate applications --file example/applications.toml
tquil config validate vps --file example/vps.toml
```

Update the paths in `config.*` to match your machine. VPS files in TOML must use
the `[[vps]]` table form because TOML has no top-level arrays.
