# ☁️ Tranquility VPS Commands

> Manage, connect to, and provision virtual private servers with ease.

---

## 📚 Table of Contents

* [⚡ Quickstart](#-quickstart)
* [📘 VPS Command Overview](#-vps-command-overview)
* [➕ add](#-add)
* [📄 list](#-list)
* [🔌 connect](#-connect)
* [🔄 update](#-update)
* [❌ delete](#-delete)
* [📜 script](#-script)
* [🧹 Global Options](#-global-options)
* [🧪 Example Use Cases](#-example-use-cases)
* [📁 VPS Config Format](#-vps-config-format)

  * [🔧 JSON Example](#-json-example)
  * [🔧 YAML Example](#-yaml-example)

---

## ⚡ Quickstart

```bash
# Add a server
tranquility vps add --name my-vps --host 192.168.1.2 --user root

# Connect to it
tranquility vps connect --id my-vps

# Run a post-provisioning script
tranquility vps script --id my-vps
```

---

## 📘 VPS Command Overview

### Commands

| Command   | Description                             |
| --------- | --------------------------------------- |
| `add`     | Add a new VPS entry to the config       |
| `list`    | List configured VPS entries             |
| `connect` | SSH into a VPS from the config          |
| `update`  | Update an existing VPS entry            |
| `delete`  | Delete a VPS entry                      |
| `script`  | Run a command or script on a remote VPS |

### Global Flags

| Flag        | Description                                     |
| ----------- | ----------------------------------------------- |
| `--delete`  | Delete the entire VPS config file               |
| `--dry-run` | Simulate the command without performing actions |

---

## ➕ `add`

Add a new VPS to your configuration.

```bash
tranquility vps add --name <name> --host <ip/domain> --user <username>
```

**Example:**

```bash
tranquility vps add --name my-vps --host 192.168.1.2 --user root
```

---

## 📄 `list`

List all configured VPS entries, optionally filtering by user.

```bash
tranquility vps list
tranquility vps list --user <username>
```

**Examples:**

```bash
tranquility vps list
tranquility vps list --user root
```

---

## 🔌 `connect`

SSH into a VPS defined in the config.

```bash
tranquility vps connect
tranquility vps connect --id <vps-id>
```

**Examples:**

```bash
tranquility vps connect
tranquility vps connect --id my-vps
```

---

## 🔄 `update`

Update an existing VPS entry. You can use it interactively or supply options directly.

```bash
tranquility vps update
tranquility vps update --id <vps-id>
tranquility vps update --id <vps-id> --host <host> --user <username>
```

**Examples:**

```bash
tranquility vps update
tranquility vps update --id my-vps
tranquility vps update --id my-vps --host 192.168.1.2 --user root
```

---

## ❌ `delete`

Delete a VPS entry by ID or interactively select one.

```bash
tranquility vps delete
tranquility vps delete --id <vps-id>
```

**Examples:**

```bash
tranquility vps delete
tranquility vps delete --id my-vps
```

---

## 📜 `script`

Run a script or command on a configured VPS. This command adapts based on which flags you provide.

### ✅ Valid Usages

```bash
# Use post_connect_script from the config, selected interactively
tranquility vps script

# Use post_connect_script from a specific VPS
tranquility vps script --id <vps-id>

# Run an inline command on a specific VPS
tranquility vps script --id <vps-id> --inline "<command>"

# Run a script file on a specific VPS
tranquility vps script --id <vps-id> --file "<path-to-script.sh>"

# Run an inline command on a VPS selected interactively
tranquility vps script --inline "<command>"

# Run a script file on a VPS selected interactively
tranquility vps script --file "<path-to-script.sh>"
```

---

### 🧠 Behavior Logic

| Scenario                             | Behavior                                                                |
| ------------------------------------ | ----------------------------------------------------------------------- |
| No `--id`, no `--inline` or `--file` | Prompt to select a VPS, then run its `post_connect_script` if available |
| `--id` only                          | Run that VPS’s `post_connect_script` if available                       |
| `--id` + `--inline`                  | Run the provided inline command on the specified VPS                    |
| `--id` + `--file`                    | Run the specified file on the specified VPS                             |
| `--inline` only                      | Prompt to select a VPS, then run the inline command                     |
| `--file` only                        | Prompt to select a VPS, then run the script file                        |

---

### 🧪 Examples

```bash
# Run a post-provisioning script from config
tranquility vps script --id prod

# Override with an inline command
tranquility vps script --id prod --inline "echo hello && uptime"

# Run a setup script from a file
tranquility vps script --file ./scripts/setup.sh

# Select a VPS and run a one-liner
tranquility vps script --inline "uptime"
```

---

## 🧹 Global Options

### `--delete`

Delete the entire VPS config file (`vps.yaml`, `vps.yml`, or `vps.json`):

```bash
tranquility vps --delete
```

---

### `--dry-run`

Simulate any `vps` command to preview changes without applying them:

```bash
tranquility vps delete --id my-vps --dry-run
```

---

## 🧪 Example Use Cases

```bash
# Connect to a VPS
tranquility vps connect --id prod-1

# Run post-provision script (from config)
tranquility vps script --id prod-1

# Run a custom inline command
tranquility vps script --id dev --inline "uptime"

# Run a setup script from a file
tranquility vps script --id dev --file ./provision.sh

# Update an entry non-interactively
tranquility vps update --id dev-box --host 10.0.0.2 --user dev
```

---

## 📁 VPS Config Format

VPS configurations are stored in either **YAML** (`vps.yaml`, `vps.yml`) or **JSON** (`vps.json`). YAML is used by default.

Each VPS entry must include at least a `name` and `host`.

### Supported Fields

| Field                 | Required | Default         | Description                            |
| --------------------- | -------- | --------------- | -------------------------------------- |
| `id`                  | No       | auto-generated  | Unique identifier for CLI usage        |
| `name`                | Yes      | —               | Display name                           |
| `host`                | Yes      | —               | IP address or domain                   |
| `user`                | No       | Current user    | SSH login username                     |
| `port`                | No       | 22              | SSH port                               |
| `private_key`         | No       | `~/.ssh/id_rsa` | Path to SSH private key                |
| `post_connect_script` | No       | —               | Command/script to run after connection |

> 📂 Config is read from the current directory. File load order is: `vps.yaml`, then `vps.yml`, then `vps.json`.

---

### 🔧 JSON Example

```json
{
  "vps": [
    {
      "id": "dev-vm",
      "name": "Dev VM",
      "host": "10.0.0.5"
    },
    {
      "name": "staging",
      "user": "root",
      "host": "staging.example.com",
      "private_key": "~/.ssh/staging_id_rsa",
      "post_connect_script": "./scripts/bootstrap.sh"
    },
    {
      "name": "prod",
      "user": "deploy",
      "host": "prod.example.com",
      "port": "2222",
      "private_key": "~/.ssh/staging_id_rsa",
      "post_connect_script": "echo Connected && uptime"
    }
  ]
}
```

---

### 🔧 YAML Example

```yaml
vps:
  - id: dev-vm
    name: Dev VM
    host: 10.0.0.5

  - name: staging
    user: root
    host: staging.example.com
    private_key: ~/.ssh/staging_id_rsa
    post_connect_script: ./scripts/bootstrap.sh

  - name: prod
    user: deploy
    host: prod.example.com
    port: 2222
    private_key: ~/.ssh/staging_id_rsa
    post_connect_script: |
      echo Starting provisioning...
      ./scripts/deploy.sh
```

---

> 📝 **Tip:** Use `--dry-run` on any command to preview the result without making changes.
