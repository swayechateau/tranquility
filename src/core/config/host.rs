use anyhow::{Context, Result, anyhow};
use std::fs;
use std::io::{Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::{Path, PathBuf};
use std::fs::create_dir_all;

use crate::core::constants::{APP_NAME, DEFAULT_DOMAINS_FILE, END_MARK, START_MARK};

#[derive(Debug, serde::Deserialize)]
pub struct HostConfig {
    #[serde(default)]
    pub prefix: Option<String>,
    #[serde(default)]
    pub domains: Vec<DomainSpec>,
}

#[derive(Debug, serde::Deserialize)]
pub struct DomainSpec {
    pub root: String,
    pub services: Vec<String>,
}

impl HostConfig {
    pub fn full_domain_list(&self) -> Vec<String> {
        let mut domains: Vec<String> = Vec::new();

        for d in &self.domains {
            let root = d.root.trim();
            if root.is_empty() {
                continue;
            }
            domains.push(root.to_string());

            for svc in &d.services {
                let svc = svc.trim();
                if svc.is_empty() {
                    continue;
                }
                let full_domain = format!("{}.{}", svc, root);
                domains.push(full_domain);
            }
        }

        domains
    }

   pub fn build_lines(&self, ipv4: &str, write_ipv6: bool) -> Result<String> {
        // Validate IPv4 a bit (not exhaustive, just avoid garbage)
        if ipv4.trim().is_empty() {
            return Err(anyhow!("IPv4 cannot be empty"));
        }

        let mut domains: Vec<String> = Vec::new();

        for d in &self.domains {
            if d.root.trim().is_empty() {
                return Err(anyhow!("root cannot be empty in a domain entry"));
            }
            for svc in &d.services {
                let host = if svc == "(root)" {
                    format!("local.{}", d.root)
                } else {
                    format!("local.{}.{}", svc.trim_matches('.'), d.root)
                };
                domains.push(host);
            }
        }

        domains.sort();
        domains.dedup();

        let mut out = String::new();
        out.push_str(START_MARK);
        out.push('\n');
        out.push_str("# Managed by dev-hosts. Do not edit between these markers.\n");

        // Group into reasonably sized lines to keep hosts tidy
        const PER_LINE: usize = 5;

        if !domains.is_empty() {
            // IPv4 lines
            for chunk in domains.chunks(PER_LINE) {
                out.push_str(ipv4);
                out.push(' ');
                out.push_str(&chunk.join(" "));
                out.push('\n');
            }
            // IPv6 lines if requested
            if write_ipv6 {
                for chunk in domains.chunks(PER_LINE) {
                    out.push_str("::1 ");
                    out.push_str(&chunk.join(" "));
                    out.push('\n');
                }
            }
        } else {
            out.push_str("# (no domains)\n");
        }

        out.push_str(END_MARK);
        out.push('\n');
        Ok(out)
    }
}

pub fn load_host_config() -> Result<HostConfig> {
    let path = resolve_or_create_config_path()
        .context("could not resolve config path")?;
    let raw = read_to_string(&path)
        .with_context(|| format!("reading config: {}", path.display()))?;
    let cfg: HostConfig = toml::from_str(&raw)
        .context("parsing TOML")?;
    Ok(cfg)
}

fn resolve_or_create_config_path() -> Option<PathBuf> {
    // 1) XDG / platform config dir + "tranquility/domains.toml"
    if let Some(cfg_dir) = dirs::config_dir() {
        let app_dir = cfg_dir.join(APP_NAME);
        if !app_dir.exists() {
            // try to create ~/.config/tranquility (or equivalent)
            if let Err(e) = create_dir_all(&app_dir) {
                eprintln!("⚠️  Failed to create config directory {}: {}", app_dir.display(), e);
                return None;
            }
        }
        let candidate = app_dir.join(DEFAULT_DOMAINS_FILE);
        if candidate.is_file() {
            return Some(candidate);
        }
    }

    // 2) Fallback: ./domains.toml
    let candidate = std::env::current_dir().ok()?.join(DEFAULT_DOMAINS_FILE);
    if candidate.is_file() {
        return Some(candidate);
    }

    None
}

pub fn upsert_managed_block(existing: &str, managed: &str) -> String {
    // If block exists, replace it; otherwise append with a leading newline.
    if let Some((start_idx, end_idx)) = find_block_indices(existing) {
        let mut s = String::with_capacity(existing.len() - (end_idx - start_idx) + managed.len());
        s.push_str(&existing[..start_idx]);
        if !s.ends_with('\n') {
            s.push('\n');
        }
        s.push_str(managed);
        if !existing[end_idx..].starts_with('\n') {
            s.push('\n');
        }
        s.push_str(&existing[end_idx..]);
        s
    } else {
        let mut s = existing.to_owned();
        if !s.ends_with('\n') {
            s.push('\n');
        }
        s.push_str(managed);
        s
    }
}

pub fn strip_managed_block(existing: &str) -> String {
    if let Some((start_idx, end_idx)) = find_block_indices(existing) {
        let mut s = String::new();
        s.push_str(&existing[..start_idx]);
        if existing[end_idx..].starts_with('\n') {
            s.push_str(&existing[end_idx + 1..]);
        } else {
            s.push_str(&existing[end_idx..]);
        }
        s
    } else {
        existing.to_owned()
    }
}

pub fn find_block_indices(s: &str) -> Option<(usize, usize)> {
    let start = s.find(START_MARK)?;
    let rest = &s[start..];
    let end_rel = rest.find(END_MARK)?;
    let end = start + end_rel + END_MARK.len();
    Some((start, end))
}

pub fn read_to_string(p: &Path) -> Result<String> {
    let mut f = fs::File::open(p)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn write_string(p: &Path, data: &str) -> Result<()> {
    // best-effort atomic-ish write: write to temp then replace
    let tmp = p.with_extension("tmp");
    {
        let mut f = fs::File::create(&tmp)?;
        f.write_all(data.as_bytes())?;
        f.sync_all()?;
    }
    fs::rename(&tmp, p)?;
    Ok(())
}

pub fn backup_file(p: &Path, contents: &str) -> Result<()> {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let backup = p.with_extension(format!("bak.{}", ts));
    let mut f = fs::File::create(&backup)?;
    f.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn count_domains(block: &str) -> usize {
    block
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .filter(|l| l.contains(' '))
        .map(|l| l.split_whitespace().skip(1).count())
        .sum()
}
