# 📦 `applications.*` Schema Documentation

This document describes the **supported structure**, **valid rules**, and **common pitfalls** for application install definitions used by your CLI.

---

## 📐 Supported Formats

Your CLI accepts:

* `.json`
* `.yaml` / `.yml`
* `.xml`

All formats are validated using a consistent schema and logic.

---

## 📄 Top-Level Structure

```jsonc
{
  "applications": [
    {
      "id": "string", // Optional: unique identifier, if not set, one is auto generated
      "name": "string",                 // Name of the application
      "categories":["string"], // Optional: if blank it is assigned misc by default, can be an array of supported categorys
      "server_compatible": true | false, // Optional: default is false
      "supported_systems": "strings[]", // Optional: if not present get supported systesm from versions::install_methods::os
      "versions": [
        {
          "name": "string",             // Version label
          "check_command": "string",    // Optional: CLI to test if installed
          "dependencies": ["string"],   // Optional
          "install_methods": [
            {
              "os": "string | string[]",        // Required: OS name or distro(s)
              "package_manager": "string",      // Optional
              "package_name": "string",         // Required in most cases
              "is_cask": true | false,          // Optional, macOS only
              "steps": {
                "preinstall_steps": ["string"],      // Optional
                "install": ["string"],               // Optional
                "postinstall_steps": ["string"],     // Optional
                "uninstall": ["string"],             // Optional
                "postuninstall_steps": ["string"]    // Optional
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

## ✅ Valid `os` Values

| Value       | Meaning                              |
| ----------- | ------------------------------------ |
| `"Linux"`   | All Linux distributions              |
| `"Ubuntu"`  | Ubuntu only                          |
| `"macOS"`   | macOS                                |
| `"Windows"` | Windows                              |
| `"Debian"`  | Debian only                          |
| `"Fedora"`  | Fedora only                          |
| etc.        | You can specify any distro as string |

* Can be a single string or an array of strings.

---

## ✅ Install Logic Rules

| Condition                                    | ✅ Valid? | Notes                                 |
| -------------------------------------------- | -------- | ------------------------------------- |
| `package_manager` **only**                   | ✅        | Requires `package_name`               |
| `steps.install` **only**                     | ✅        | Full CLI install                      |
| `steps.uninstall` **only**                   | ✅        | CLI uninstall, PM install             |
| `steps.install` + `steps.uninstall` **only** | ✅        | Full CLI override — PM optional       |
| `steps` + `package_manager`                  | ✅/❌      | Depends on fallback logic (see below) |
| No `steps` and no `package_manager`          | ❌        | Invalid — nothing to install with     |

---

## ⚠️ Special Rules

### Rule 1: At least one method must exist

Each install method **must have either**:

* `steps` **OR**
* `package_manager`

Otherwise, it's invalid.

---

### Rule 2: `package_name` is required when fallback to PM is possible

If `package_manager` is defined:

* ✅ `package_name` is **required** if:

  * `steps` is missing, or
  * `steps.install` is missing, or
  * `steps.uninstall` is missing

* ✅ `package_name` is **not required** only if:

  * Both `steps.install` **and** `steps.uninstall` are present

---

### Rule 3: CLI overrides package manager

If `steps.install` is defined, CLI is used to install.
If `steps.uninstall` is defined, CLI is used to uninstall.

---

## ✅ Valid Examples

### Full Package Manager

```json
{
  "os": "Ubuntu",
  "package_manager": "apt",
  "package_name": "myapp"
}
```

---

### Full CLI Override

```json
{
  "os": "Linux",
  "steps": {
    "install": ["./install.sh"],
    "uninstall": ["./uninstall.sh"]
  }
}
```

---

### Hybrid CLI/PM

```json
{
  "os": "Debian",
  "package_manager": "apt",
  "package_name": "myapp",
  "steps": {
    "uninstall": ["custom uninstall script"]
  }
}
```

---

## ❌ Invalid Examples

### ❌ Missing both steps and package manager

```json
{
  "os": "Ubuntu"
}
```

**Why?** No method to install or uninstall.

---

### ❌ Missing package\_name when fallback would use it

```json
{
  "os": "Ubuntu",
  "package_manager": "apt",
  "steps": {
    "install": ["./install.sh"]
  }
}
```

**Why?** `steps.uninstall` is missing → fallback to PM uninstall requires `package_name`.

---

## 🔄 Format Conversions

Your CLI supports loading the same schema from:

* `applications.json`
* `applications.yaml` / `.yml`
* `applications.xml`

Make sure the structure and keys match even in XML.

---

## 🧪 Testing

Run validation via your CLI:

```sh
mycli validate --file path/to/applications.yaml
```

---

## 📎 Related Files

* `models.rs` — Structs for deserialization
* `schema/application.rs` — Validation logic
* `tests/fixtures/*.json|yaml|xml` — Valid/invalid examples
