# 🧩 Tranquility Application Commands

> Install, manage, and validate system applications using OS-aware, declarative definitions.

This document provides a complete overview of all `tranquility app` commands, supported options, configuration format, and usage examples.

---

## 📚 Table of Contents

* [📘 App Command Overview](#-app-command-overview)
* [📦 `install`](#-install)
* [🧹 `uninstall`](#-uninstall)
* [📋 `list`](#-list)
* [♻️ `reset`](#-reset)
* [📝 `override`](#-override)
* [✅ `validate`](#-validate)
* [📁 Application Config Format](#-application-config-format)
  * [🔧 YAML Example](#-yaml-example)
  * [🔧 JSON Example](#-json-example)
  * [🧠 Validation Logic](#-validation-logic)
* [🗃 Supported Categories](#-supported-categories)
* [🧪 Example Workflows](#-example-workflows)

---

**Examples:**

```bash
tranquility --dry-run app install --all
tranquility --debug app uninstall --name nginx
```

---

## 📘 App Command Overview

### Commands

| Command     | Description                                          |
| ----------- | ---------------------------------------------------- |
| `install`   | Install applications from config                     |
| `uninstall` | Uninstall applications using defined uninstall steps |
| `list`      | List apps and filter by category or server type      |


### Global Flags

These flags apply to **any** `tranquility` command, including `app`.

| Flag        | Description                                 |
| ----------- | ------------------------------------------- |
| `--dry-run` | Simulate actions without making changes     |
| `--debug`   | Print verbose internal output for debugging |

```bash
tranquility --debug app install
tranquility --dry-run app uninstall
```

---

## 📦 `install`

Install apps defined in your config.

| Option        | Description                                      |
| ------------- | ------------------------------------------------ |
| `--category`  | Install apps by category/s                          |
| `--server`    | Install only server compaible apps                  |
| `--all`       | Install all apps                                    |

```bash
tranquility app install --all
tranquility app install --name <app1> <app2>
tranquility app install --category <category>
```

**Examples:**

```bash
tranquility app install
tranquility app install --all
tranquility app install --server
tranquility app install --category dev
tranquility app install --name node git
tranquility --dry-run app install --name docker
```

---

## 🧹 `uninstall`

Uninstall apps using their defined uninstall logic.

| Option        | Description                                      |
| ------------- | ------------------------------------------------ |
| `--category`  | Remove apps by category/s                        |
| `--server`    | Remove only server compaible apps                |
| `--all`       | Remove all apps                                  |

```bash
tranquility app uninstall
tranquility app uninstall --all
tranquility app uninstall --name <app1> <app2>
tranquility app uninstall --server --category dev
tranquility app uninstall --server --all
tranquility app uninstall --category dev
```

**Examples:**

```bash
tranquility app uninstall --name postgresql redis
tranquility app uninstall --server
```

---

## 📋 `list`

List defined applications and optionally filter them.

| Option        | Description                                      |
| ------------- | ------------------------------------------------ |
| `--category`  | Show apps by category/s                          |
| `--server`    | Show only server compaible apps                  |

```bash
tranquility app list
tranquility app list --category <category>
tranquility app list --server
```

**Examples:**

```bash
tranquility app list
tranquility app list --category editors
tranquility app list --server
```

---

## Config Commands

to see the full list 
[📘 Full Config Command Reference →](./command-config.md)

### ♻️ `To reset`

Reset the applications config file to its default version.

```bash
tranquility config reset applications
```

### 📝 `To override`

Replace the current applications config with another file.

```bash
tranquility config override applications --file ./apps.yaml
```


### ✅ `To validate`

Validate the format, schema, and install logic of the application config.

```bash
tranquility config validate applications --file ./apps.yaml
```

---

## 📁 Application Config Format

Tranquility supports **YAML**, **JSON**, or **XML** to define your application list.

---

### 🔧 YAML Example

```yaml
applications:
  - name: Neovim
    categories: [editors, dev]
    supported_systems: [Linux, macOS]
    versions:
      - name: Stable
        check_command: nvim
        dependencies: [curl]
        install_methods:
          - os: Ubuntu
            package_manager: Apt
            package_name: neovim
          - os: macOS
            steps:
              install:
                - brew install neovim
```

---

### 🔧 JSON Example

```json
{
  "applications": [
    {
      "name": "Neovim",
      "categories": ["editors", "dev"],
      "supported_systems": ["Linux", "macOS"],
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

### 🧠 Validation Logic

| `steps` | `package_manager` | `package_name` | ✅ Valid | Notes                       |
| ------- | ----------------- | -------------- | ------- | --------------------------- |
| ❌       | ❌                 | N/A            | ❌       | No way to install           |
| ✅       | ❌                 | N/A            | ✅       | Shell steps only            |
| ❌       | ✅                 | ✅              | ✅       | Package manager only        |
| ✅       | ✅                 | ✅ / ❌          | ✅       | Shell overrides PM fallback |

---

## 🗃 Supported Categories

Filter applications using `--category` in `install`, `list`, and `uninstall`.

Common tags and full list can be seen using the list command:
[📘 Full List Command Reference →](./command-list.md)

---

## 🧪 Example Workflows

```bash
# Preview installing selected tools
tranquility app install --name neovim htop

# Install all CLI tools
tranquility app install --category cli

# Validate a custom app config
tranquility config validate applications --file ./apps.yaml

# Override the config with a production file
tranquility config override applications --file ./prod-apps.json

# Uninstall all server-related applications
tranquility app uninstall --server
```

---

> 📝 **Tip:** Use `--dry-run` on any command to preview the result without making changes.