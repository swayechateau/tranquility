
## 🖋️ `fonts`

Install, uninstall, update, and list Nerd Fonts.

### Install Fonts

| Option        | Description                            |
| ------------- | -------------------------------------- |
| `--all`       | List all fonts (installed + available) |
| `--name`      | Font name(s) to install/uninstall      |

```bash
tranquility fonts install
tranquility fonts install --all
tranquility fonts install --name FiraCode JetBrainsMono
```

### Uninstall Fonts

| Option        | Description                            |
| ------------- | -------------------------------------- |
| `--all`       | Uninstall all fonts                    |
| `--name`      | Font name(s) to install/uninstall      |

```bash
tranquility fonts uninstall
tranquility fonts uninstall --all
tranquility fonts uninstall --name FiraCode
```

### Update Fonts

| Option        | Description                            |
| ------------- | -------------------------------------- |
| `--name`      | Font name(s) to install/uninstall      |

```bash
tranquility fonts update # updates all
tranquility fonts update --name FiraCode --name FiraCodeMono # updates only the named fonts
```

### List Fonts

| Option        | Description                            |
| ------------- | -------------------------------------- |
| `--all`       | List all fonts (installed + available) |
| `--installed` | List only installed fonts              |

```bash
tranquility fonts list
tranquility fonts list --installed
tranquility fonts list --all
```


## Future improvements

* [ ] Add `font search <pattern>`
* [ ] Add `font export` (optional)