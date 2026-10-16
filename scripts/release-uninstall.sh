#!/usr/bin/env bash
# uninstall-tquil.sh – Remove the “tquil” binary from Linux or macOS.

set -euo pipefail

PROG_NAME="tquil"

# Colors (printf-friendly)
COLOR_OK="\033[32m"
COLOR_ERR="\033[31m"
COLOR_RST="\033[0m"

# ── 1. Detect platform ───────────────────────────────────────────────────────
case "$(uname -s)" in
    Linux*)  PLATFORM=linux ;;
    Darwin*) PLATFORM=mac ;;
    *)
        printf "${COLOR_ERR}Unsupported OS: %s${COLOR_RST}\n" "$(uname -s)"
        exit 1
        ;;
esac

# ── 2. Assemble likely install paths ─────────────────────────────────────────
CANDIDATE_PATHS=(
    "/usr/local/bin/$PROG_NAME"
    "/usr/bin/$PROG_NAME"
    "$HOME/.local/bin/$PROG_NAME"
)

# Homebrew prefixes (Intel & ARM)
if [[ "$PLATFORM" == "mac" ]]; then
    [[ -d "/opt/homebrew/bin" ]]  && CANDIDATE_PATHS+=("/opt/homebrew/bin/$PROG_NAME")
    [[ -d "/usr/local/bin"    ]]  && CANDIDATE_PATHS+=("/usr/local/bin/$PROG_NAME")
fi

# ── 3. Find the first existing binary ────────────────────────────────────────
FOUND=""

for p in "${CANDIDATE_PATHS[@]}"; do
    if [[ -x "$p" ]]; then
        FOUND="$p"
        break
    fi
done

if [[ -z "$FOUND" ]]; then
    printf "${COLOR_ERR}No %s installation found.${COLOR_RST}\n" "$PROG_NAME"
    exit 0
fi

printf "Found ${COLOR_OK}%s${COLOR_RST} at: %s\n" "$PROG_NAME" "$FOUND"

# ── 4. Confirm uninstall ─────────────────────────────────────────────────────
printf "Uninstall it? [y/N] "
read -r REPLY
REPLY=$(printf '%s' "$REPLY" | tr '[:upper:]' '[:lower:]')

# ── 5. Remove it ─────────────────────────────────────────────────────────────
if [[ "$REPLY" == "y" || "$REPLY" == "yes" ]]; then

    # user-local install (no sudo required)
    if [[ "$FOUND" == "$HOME/"* ]]; then
        rm -v "$FOUND"
    else
        # system install — sudo required
        if ! command -v sudo >/dev/null 2>&1; then
            printf "${COLOR_ERR}sudo not installed; cannot remove system binary.${COLOR_RST}\n"
            exit 1
        fi
        sudo rm -v "$FOUND"
    fi

    printf "${COLOR_OK}%s has been uninstalled.${COLOR_RST}\n" "$PROG_NAME"
else
    printf "Aborting uninstallation.\n"
fi
