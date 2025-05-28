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

## 🚀 Usage

```bash
tranquility [OPTIONS] <COMMAND>
```

Use `--help` after any command for more information.

---

## 🧭 Command Overview

| Command  | Description                                |
| -------- | ------------------------------------------ |
| `doctor` | Diagnose and optionally fix CLI issues     |
| `config` | Manage and validate configuration files    |
| `logs`   | View CLI logs                              |
| `font`   | Install, uninstall, update, and list fonts |
| `app`    | Install and manage applications            |
| `vps`    | Manage VPS hosts                           |
| `list`   | List apps, fonts, VPS, and categories      |

---

## 🩺 `doctor`

Check and fix common CLI issues:

[📘 Full Doctor Command Reference →](./docs/command-doctor.md)

---

## ⚙️ `config`

Manage configuration files for Tranquility. (supports xml, json or yaml [yaml is default])

[📘 Full Config Command Reference →](./docs/command-config.md)

---

## 📜 `logs`

View CLI logs.

[📘 Full Logs Command Reference →](./docs/command-logs.md)

---

## 🖋️ `font`

Manage Nerd Fonts.

[📘 Full Font Command Reference →](./docs/command-font.md)

---

## 🧩 `app`

Manage your application setup.

[📘 Full App Command Reference →](./docs/command-app.md)

---

## 🌐 `vps`

Manage virtual private servers.

[📘 Full VPS Command Reference →](./docs/command-vps.md)

---

## 📦 `list`

List supported resources:

[📘 Full List Command Reference →](./docs/command-list.md)

---

## 🐛 Debugging

Use the global `--debug` flag to enable verbose logging:

```bash
tranquility --debug apps install
```

---

## 🧪 Example Workflows

* Install all fonts and apps in dry-run mode:

  ```bash
  tranquility font install --all --dry-run
  tranquility app install --all --dry-run
  ```

* Override the applications config file:

  ```bash
  tranquility config override applications --file ./apps.json
  ```

* Validate a VPS config:

  ```bash
  tranquility config validate vps --file ./vps.json
  ```

* Reset the full config to defaults:

  ```bash
  tranquility config reset
  ```

---

## ✅ Validation Logic

| `steps`  | `package_manager`  | `package_name` | ✅ Valid? | Notes                  |
| -------- | ------------------ | -------------- | --------- | ---------------------- |
| ❌       | ❌                 | N/A            | ❌        | No way to install      |
| ✅       | ❌                 | N/A            | ✅        | CLI only               |
| ❌       | ✅                 | ✅             | ✅        | PM only                |
| ✅       | ✅                 | ✅/❌          | ✅        | CLI override or hybrid |

Run:

```bash
tranquility config validate --file ./applications.yaml
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
