# ⚙️ Tranquility Configuration Commands

> Reset, override, and validate Tranquility CLI configuration files with precise control.

This guide explains how to manage configuration files using the `tranquility config` command. Each subcommand supports specific config targets such as the main CLI config, application definitions, or VPS environments.

---

## 📚 Table of Contents

* [🔁 Reset Configuration](#-reset-configuration)
* [🔄 Override Configuration](#-override-configuration)
* [✅ Validate Configuration](#-validate-configuration)
* [📂 Supported Targets](#-supported-targets)
* [🧪 Example Workflows](#-example-workflows)

---

## 🔁 Reset Configuration

Reset a Tranquility configuration file to its built-in default.

| Target         | Description                   |
| -------------- | ----------------------------- |
| `config`       | Reset the main CLI config     |
| `applications` | Reset the applications config |
| `vps`          | Reset the VPS config          |


```bash
tranquility config reset [config|applications|vps]
```

> 🧠 **Note:** If no target is specified, `config` is assumed by default.

---

## 🔄 Override Configuration

Replace an existing configuration file with a custom file of your choice.


| Target         | Description                      |
| -------------- | -------------------------------- |
| `config`       | Override the main CLI config     |
| `applications` | Override the applications config |
| `vps`          | Override the VPS config          |


```bash
tranquility config override [config|applications|vps] --file <path>
```

> 📁 The specified file must follow the correct format for the target (YAML, JSON, or XML depending on type).

---

## ✅ Validate Configuration

Check a configuration file for structural integrity, schema correctness, and logical consistency.

| Target         | Description                      |
| -------------- | -------------------------------- |
| `config`       | Validate the main CLI config     |
| `applications` | Validate the applications config |
| `vps`          | Validate the VPS config          |

```bash
tranquility config validate [config|applications|vps] --file <path>
```

> ✅ Validation catches issues such as malformed YAML, missing fields, or invalid install logic.

---

## 📂 Supported Targets

Each `config` subcommand supports the following target types:

| Target         | Description                                  |
| -------------- | -------------------------------------------- |
| `config`       | The main Tranquility CLI config file         |
| `applications` | The declarative application list and logic   |
| `vps`          | Virtual server presets and provisioning data |

---

## 🗂 Configuration File Format

Tranquility supports `JSON`, `YAML`, and `XML` formats for all configuration files. The main `config` file defines paths to your application and VPS configs, as well as logging behavior.

### 📄 JSON Example (`config.json`)

```json
{
  "applications_file": "/home/chatsw/.config/tranquility/applications.json",
  "vps_file": "/home/chatsw/.config/tranquility/vps.json",
  "log_directory": "/home/chatsw/.config/tranquility/logs",
  "log_output": "primary"
}
```

### 🧾 YAML Example (`config.yaml`)

```yaml
applications_file: /home/chatsw/.config/tranquility/applications.yaml
vps_file: /home/chatsw/.config/tranquility/vps.yaml
log_directory: /home/chatsw/.config/tranquility/logs
log_output: primary
```

### 📜 XML Example (`config.xml`)

```xml
<config>
  <applications_file>/home/chatsw/.config/tranquility/applications.xml</applications_file>
  <vps_file>/home/chatsw/.config/tranquility/vps.xml</vps_file>
  <log_directory>/home/chatsw/.config/tranquility/logs</log_directory>
  <log_output>primary</log_output>
</config>
```

---

> 💡 **Tip:** Use `tranquility config validate config --file <path>` to confirm the structure and file paths are correct before using a custom config.

---

## 🧪 Example Workflows

```bash
# Reset the entire CLI to default settings
tranquility config reset

# Replace applications config with a custom YAML
tranquility config override applications --file ./apps.yaml

# Validate a VPS setup file before provisioning
tranquility config validate vps --file ./server-defs.json
```

---

> 💡 **Tip:** Use validation before overriding configs in production workflows to avoid unexpected behavior.

### Future Improvements

* [ ] Add config lock with `fs2::FileExt::lock_exclusive()`