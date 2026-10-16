#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=libs/common.sh
. "$SCRIPT_DIR/libs/common.sh"

os_distro="$(detect_os)"
os="${os_distro%%|*}"
pkg_mgr="$(detect_pkg_mgr "$os")"

install_rustup "$pkg_mgr"

cargo build --release
