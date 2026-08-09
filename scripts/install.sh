#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=libs/common.sh
. "$SCRIPT_DIR/libs/common.sh"

PROG_NAME="tquil"
SOURCE_BIN="./target/release/tranquility"

# ── 0. Choose global install dir ─────────────────────────────────────────────
os_distro="$(detect_os)"
os="${os_distro%%|*}"

if [[ "$os" == "macos" && "$(uname -m)" == "arm64" && -d "/opt/homebrew/bin" ]]; then
    GLOBAL_DIR="/opt/homebrew/bin"
else
    GLOBAL_DIR="/usr/local/bin"
fi

LOCAL_DIR="$HOME/.local/bin"

# ── 1. Pull & build ──────────────────────────────────────────────────────────
info "Pulling latest code and building…"

if has_cmd git; then
    git pull --ff-only || warn "Git pull failed (continuing)…"
fi

if [[ -x "$SCRIPT_DIR/build" ]]; then
    "$SCRIPT_DIR/build"
else
    pkg_mgr="$(detect_pkg_mgr "$os")"
    install_rustup "$pkg_mgr"
    cargo build --release
fi

if [[ ! -x "$SOURCE_BIN" ]]; then
    err "Build failed: binary not found at $SOURCE_BIN."
    exit 1
fi

# ── 2. Ask whether to install ────────────────────────────────────────────────
printf "\nInstall %s? [y/N] " "$PROG_NAME"
read -r REPLY
REPLY=$(printf '%s' "$REPLY" | tr '[:upper:]' '[:lower:]')

if [[ "$REPLY" != "y" && "$REPLY" != "yes" ]]; then
    info "Skipping installation."
    exit 0
fi

# ── 3. Decide install scope ──────────────────────────────────────────────────
if [[ $EUID -eq 0 ]]; then
    INSTALL_DIR="$GLOBAL_DIR"
    USE_SUDO=""
else
    while true; do
        printf "Install for all users or just you?\n"
        printf "[g]lobal / [l]ocal  (default: local): "
        read -r SCOPE
        SCOPE=$(printf '%s' "$SCOPE" | tr '[:upper:]' '[:lower:]')

        case "$SCOPE" in
            ""|"l"|"local")
                INSTALL_DIR="$LOCAL_DIR"
                USE_SUDO=""
                break
                ;;
            "g"|"global")
                INSTALL_DIR="$GLOBAL_DIR"
                USE_SUDO="$(need_sudo)"
                break
                ;;
            *)
                warn "Please answer 'g' or 'l'."
                ;;
        esac
    done
fi

# ── 4. Ensure target directory exists ────────────────────────────────────────
if [[ "$INSTALL_DIR" == "$LOCAL_DIR" && ! -d "$LOCAL_DIR" ]]; then
    mkdir -p "$LOCAL_DIR"
fi

# ── 5. Install binary ────────────────────────────────────────────────────────
info "Copying binary to $INSTALL_DIR/$PROG_NAME"

${USE_SUDO:+$USE_SUDO} cp "$SOURCE_BIN" "$INSTALL_DIR/$PROG_NAME"
${USE_SUDO:+$USE_SUDO} chmod +x "$INSTALL_DIR/$PROG_NAME"

ok "Installation complete."
printf "Run '%s --help' to get started.\n" "$PROG_NAME"

if [[ "$INSTALL_DIR" == "$LOCAL_DIR" ]]; then
    info "Make sure $LOCAL_DIR is in your PATH:"
    printf "  export PATH=\"%s:\$PATH\"\n" "$LOCAL_DIR"
fi
