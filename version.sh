#!/bin/bash

CARGO_TOML="Cargo.toml"

# Function to get version from Cargo.toml
get_version() {
    grep -m 1 '^version\s*=' "$CARGO_TOML" | sed -E 's/version\s*=\s*"([^"]+)"/\1/'
}

# Function to set version in Cargo.toml
set_version() {
    local new_version="$1"
    if [[ ! "$new_version" =~ ^v?[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo "Invalid version format. Use vX.Y.Z or X.Y.Z"
        exit 1
    fi
    # Remove leading 'v' if present
    new_version="${new_version#v}"
    # Update version in Cargo.toml
    sed -i.bak -E "0,/^version\s*=\s*\"[^\"]+\"/s//version = \"$new_version\"/" "$CARGO_TOML"
    echo "Version updated to $new_version"
}

if [[ "$1" == "--set" && -n "$2" ]]; then
    set_version "$2"
else
    version=$(get_version)
    if [[ -n "$version" ]]; then
        echo "$version"
    else
        echo "Version not found in $CARGO_TOML"
        exit 1
    fi
fi