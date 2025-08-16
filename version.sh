#!/bin/bash
set -euo pipefail

CARGO_TOML="${CARGO_TOML:-Cargo.toml}"

usage() {
  echo "Usage:"
  echo "  $0                # print version"
  echo "  $0 --set X.Y.Z    # set version"
  exit 1
}

# Prefer [workspace.package], then [package]
get_version() {
  awk '
    BEGIN{ sec=""; }
    /^[[:space:]]*\[[^]]+\][[:space:]]*$/ {
      if ($0 ~ /^[[:space:]]*\[workspace\.package\][[:space:]]*$/) sec="ws";
      else if ($0 ~ /^[[:space:]]*\[package\][[:space:]]*$/) sec="pkg";
      else sec="";
    }
    sec=="ws" && /^[[:space:]]*version[[:space:]]*=/ {
      if (match($0, /"[^"]+"/)) { v=substr($0,RSTART+1,RLENGTH-2); print v; exit 0 }
    }
    END{}' "$CARGO_TOML" || true

  awk '
    BEGIN{ sec=""; }
    /^[[:space:]]*\[[^]]+\][[:space:]]*$/ {
      if ($0 ~ /^[[:space:]]*\[workspace\.package\][[:space:]]*$/) sec="ws";
      else if ($0 ~ /^[[:space:]]*\[package\][[:space:]]*$/) sec="pkg";
      else sec="";
    }
    sec=="pkg" && /^[[:space:]]*version[[:space:]]*=/ {
      if (match($0, /"[^"]+"/)) { v=substr($0,RSTART+1,RLENGTH-2); print v; exit 0 }
    }
    END{}' "$CARGO_TOML" || true
}

set_version() {
  local new_version="$1"
  if [[ ! "$new_version" =~ ^v?[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Invalid version format. Use vX.Y.Z or X.Y.Z" >&2; exit 1
  fi
  new_version="${new_version#v}"

  local backup="${CARGO_TOML}.bak"
  [[ -f "$backup" ]] || cp "$CARGO_TOML" "$backup"

  # Try workspace.package first
  local tmp
  tmp="$(mktemp)"
  if awk -v new="$new_version" '
    BEGIN{ sec=""; replaced=0 }
    /^[[:space:]]*\[[^]]+\][[:space:]]*$/ {
      if ($0 ~ /^[[:space:]]*\[workspace\.package\][[:space:]]*$/) sec="ws";
      else if ($0 ~ /^[[:space:]]*\[package\][[:space:]]*$/) sec="pkg";
      else sec="";
      print; next
    }
    sec=="ws" && !replaced && /^[[:space:]]*version[[:space:]]*=/ {
      # Preserve leading whitespace and trailing suffix after the closing quote
      # indent = leading spaces/tabs
      match($0, /^[ \t]*/); indent=substr($0,1,RLENGTH);
      # find first and second quote
      p1 = index($0, "\"");
      if (p1 > 0) {
        rest = substr($0, p1+1);
        p2 = index(rest, "\"");
      } else { p2=0 }
      if (p1>0 && p2>0) {
        suffix = substr($0, p1+p2+1);
        print indent "version = \"" new "\"" suffix;
        replaced=1; next
      }
    }
    { print }
    END{ if (!replaced) exit 2 }
  ' "$CARGO_TOML" > "$tmp"; then
    mv "$tmp" "$CARGO_TOML"
    echo "Version updated in [workspace.package] to $new_version"
    return 0
  else
    local rc=$?
    rm -f "$tmp"
    if [[ $rc -ne 2 ]]; then
      echo "Failed updating [workspace.package] (awk exit $rc)" >&2; exit 1
    fi
  fi

  # Fallback: package
  tmp="$(mktemp)"
  if awk -v new="$new_version" '
    BEGIN{ sec=""; replaced=0 }
    /^[[:space:]]*\[[^]]+\][[:space:]]*$/ {
      if ($0 ~ /^[[:space:]]*\[workspace\.package\][[:space:]]*$/) sec="ws";
      else if ($0 ~ /^[[:space:]]*\[package\][[:space:]]*$/) sec="pkg";
      else sec="";
      print; next
    }
    sec=="pkg" && !replaced && /^[[:space:]]*version[[:space:]]*=/ {
      match($0, /^[ \t]*/); indent=substr($0,1,RLENGTH);
      p1 = index($0, "\"");
      if (p1 > 0) {
        rest = substr($0, p1+1);
        p2 = index(rest, "\"");
      } else { p2=0 }
      if (p1>0 && p2>0) {
        suffix = substr($0, p1+p2+1);
        print indent "version = \"" new "\"" suffix;
        replaced=1; next
      }
    }
    { print }
    END{ if (!replaced) exit 2 }
  ' "$CARGO_TOML" > "$tmp"; then
    mv "$tmp" "$CARGO_TOML"
    echo "Version updated in [package] to $new_version"
  else
    local rc=$?
    rm -f "$tmp"
    if [[ $rc -eq 2 ]]; then
      echo "No editable version found under [workspace.package] or [package] in $CARGO_TOML" >&2
      exit 1
    else
      echo "Failed updating [package] (awk exit $rc)" >&2
      exit 1
    fi
  fi
}

case "${1-}" in
  --set)
    [[ -n "${2-}" ]] || usage
    set_version "$2"
    ;;
  "" )
    v="$(get_version || true)"
    if [[ -n "${v-}" ]]; then
      echo "$v"
    else
      echo "Version not found under [workspace.package] or [package]" >&2
      exit 1
    fi
    ;;
  * )
    usage
    ;;
esac
