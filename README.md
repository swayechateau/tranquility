# 🧘 Tranquility

> *Peace of mind through dev environment automation.*

**Tranquility** is a robust, cross-platform CLI tool that automates the setup and teardown of development environments. With deep OS-awareness, custom script execution, VPS provisioning, font installation, and strict schema validation, Tranquility helps you spin up dev-ready systems in minutes.

---

## ✨ Features

### 🧩 Application Management

* JSON/YAML/XML-based install definitions
* OS + distro awareness (Debian, Fedora, macOS, etc.)
* Supports apt, brew, winget, pacman, nix, and more
* CLI-based shell install logic with fallback to package manager
* Dependency management and interactive or auto mode

### 🖼 Nerd Fonts

* Install from 60+ popular Nerd Fonts
* Interactive and bulk install modes
* Auto downloads, unzips, installs, and refreshes font cache

### 🧠 Smart System Detection

* OS, architecture, distro, CPU info
* Detects default and available package managers
* Can auto-install missing ones interactively

### ☁️ VPS Provisioning

* Define remote VPSes with SSH config
* Run post-connect provisioning scripts
* JSON-based config management and CLI editor

### 📜 Script Execution Engine

* Unified shell abstraction for Unix and Windows
* Runs inline or file-based scripts with sudo support
* Dry-run mode for safe previews

### 🧪 Schema Validation

* Validates applications.json/yaml/xml files
* Enforces CLI+PM fallback logic
* Detailed error messages with index references
* Example schema in `schema.md`

---

## ⚙️ Installation

```bash
git clone https://github.com/swayechateau/tranquility.git
cd tranquility
./build
./run
```

---

## 🚀 CLI Commands

### Install Applications

```bash
tranquility install --all
```

* `--all`: installs all matched apps
* `--server`: server-safe apps only
* `--dry-run`: preview without changes

### Uninstall Applications

```bash
tranquility uninstall --all
```

Reverses install steps if uninstall logic is defined.

### Fonts

```bash
tranquility fonts
tranquility fonts --all
```

Interactive or full bulk Nerd Font install.

### VPS

```bash
tranquility vps --list
tranquility vps --add
tranquility vps --connect
tranquility vps --delete
```

---

## 🧬 Application Format (JSON Example)

```json
{
  "applications": [
    {
      "name": "Neovim",
      "categories": ["Editors", "Development"],
      "supported_systems": ["Linux", "MacOS"],
      "versions": [
        {
          "name": "Stable",
          "check_command": "nvim",
          "dependencies": ["curl"],
          "install_methods": [
            {
              "os": "Ubuntu",
              "package_manager": "Apt",
              "package_name": "neovim"
            },
            {
              "os": "macOS",
              "steps": {
                "install": ["brew install neovim"]
              }
            }
          ]
        }
      ]
    }
  ]
}
```

---

## ✅ Validation Logic

| `steps` | `package_manager` | `package_name` | ✅ Valid? | Notes                  |
| ------- | ----------------- | -------------- | -------- | ---------------------- |
| ❌       | ❌                 | N/A            | ❌        | No way to install      |
| ✅       | ❌                 | N/A            | ✅        | CLI only               |
| ❌       | ✅                 | ✅              | ✅        | PM only                |
| ✅       | ✅                 | ✅/❌            | ✅        | CLI override or hybrid |

Run:

```bash
tranquility validate --file ./applications.yaml
```

---

## ☁️ VPS Example

```json
{
  "name": "dev-vm",
  "username": "ubuntu",
  "host": "10.0.0.5",
  "private_key": "~/.ssh/id_rsa",
  "post_connect_script": "echo Connected && uptime"
}
```

---

## 🧰 Developer Notes

* Built with Rust, Clap, Schemars, JSONSchema, Figlet, and Dialoguer
* Color-coded CLI output with `print_info!`, `print_error!`, etc.
* Config lives in `~/.config/tranquility/config.json`

---

## 🤝 Contributing

Pull requests welcome.
Open an issue or fork the repo to get started.
