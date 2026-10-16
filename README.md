# Tranquility (Development Environment Setup)



## 📦 Version Management

This project includes a custom Cargo subcommand, **`cargo version`**, located in:

```sh
.cargo/bin/cargo-version
```

It is a project-specific version manager that keeps **Cargo.toml** and the **VERSION** file in sync, and provides semantic version bumping.

---

### ✨ What It Does

The `cargo version` command:

#### 🔍 Reads the current version

* Preferentially reads from `[workspace.package]`
* Falls back to `[package]`
* Ensures the root `VERSION` file is always up-to-date
* Auto-corrects if the files ever diverge

#### ✏️ Sets a new version

```sh
cargo version --set 1.2.3
```

Updates:

* `Cargo.toml`
* `VERSION`
* Preserves formatting, indentation, and comments

#### ⬆️ Semantic version bumping

```sh
cargo version --bump patch
cargo version --bump minor
cargo version --bump major
```

This automatically increments the version (per semver rules) and updates all version sources.

#### 🛠 Automatically creates a `VERSION` file

If the file does not exist, it is created with the normalized version.

---

### 🚀 Installation / Setup

To enable the custom `cargo version` command, ensure the script is executable and in the project-local Cargo bin path.

#### 1. Ensure the script exists

```sh
.cargo/bin/cargo-version
```

#### 2. Make it executable

```sh
chmod +x .cargo/bin/cargo-version
```

#### 3. Add `.cargo/bin` to your PATH (recommended)

Add to your shell rc file:

```sh
export PATH="$PWD/.cargo/bin:$PATH"
```

Or for a single session:

```sh
PATH="$PWD/.cargo/bin:$PATH"
```

Now you can run:

```sh
cargo version
```

just like any built-in Cargo command.

---

### 🧪 Usage Examples

#### Print current version

```sh
cargo version
```

#### Set a specific version

```sh
cargo version --set 2.0.0
```

Accepts both `1.2.3` and `v1.2.3`.

#### Bump patch

```sh
cargo version --bump patch
```

#### Bump minor

```sh
cargo version --bump minor
```

#### Bump major

```sh
cargo version --bump major
```

---

### 🔒 Guarantees

* The version in `Cargo.toml` is treated as the **source of truth**.
* The `VERSION` file will never drift — it is synced automatically.
* Formatting inside `Cargo.toml` is preserved.
* Safe, strict script execution (`set -euo pipefail`).

---

### License

Tranquility is licensed under the GNU General Public License, version 2 only (SPDX: `GPL-2.0-only`). The full text is in [`LICENSE`](LICENSE). It is not licensed under "any later version".
