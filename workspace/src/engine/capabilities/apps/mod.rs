use dialoguer::Confirm;
use os_info::Type as OSType;

use crate::engine::error::{Error, ErrorCode};
use crate::engine::models::application::{Application, ApplicationList, InstallMethod};
use crate::engine::models::category::Category;
use crate::engine::models::system::{OsSupport, SystemInfo};

// ---------------------------------------------------------------------------
// Application loading
// ---------------------------------------------------------------------------

/// Parse an applications file's content, picking the format from its extension
/// (json, yaml, yml, xml — the same formats `config`/`vps` files support).
fn parse_apps_str(path: &std::path::Path, data: &str) -> Result<ApplicationList, String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_lowercase();

    match ext.as_str() {
        "json" => serde_json::from_str(data).map_err(|e| e.to_string()),
        "yaml" | "yml" => serde_yaml::from_str(data).map_err(|e| e.to_string()),
        "xml" => quick_xml::de::from_str(data).map_err(|e| e.to_string()),
        other => Err(format!("unsupported file extension: .{other}")),
    }
}

/// Load applications from an optional file, falling back to built-in
/// defaults. By default the file's entries are added to the built-ins; with
/// `replace: true` the file's entries are used *instead of* the built-ins —
/// for a user or company catalog meant to be the complete, authoritative
/// list rather than an extension of it.
pub fn load_apps(apps_file: Option<&std::path::Path>, replace: bool) -> ApplicationList {
    let mut apps = if replace { Vec::new() } else { default_apps() };

    if let Some(path) = apps_file
        && path.exists()
    {
        match std::fs::read_to_string(path) {
            Ok(data) => match parse_apps_str(path, &data) {
                Ok(user_apps) => {
                    tracing::info!(
                        "Loaded {} applications from {}",
                        user_apps.applications.len(),
                        path.display()
                    );
                    apps.extend(user_apps.applications);
                }
                Err(e) => eprintln!("❌ Failed to parse applications file: {e}"),
            },
            Err(e) => eprintln!("❌ Failed to read applications file: {e}"),
        }
    } else if replace {
        eprintln!("⚠️  --replace given but no applications file is configured — catalog is empty.");
    }

    ApplicationList { applications: apps }
}

/// Validate an applications file. Returns the number of applications.
pub fn validate_apps_file(path: &std::path::Path) -> Result<usize, Error> {
    let data = std::fs::read_to_string(path).map_err(|e| {
        ErrorCode::IoFailure
            .error()
            .with_context("path", path.display().to_string())
            .with_context("error", e.to_string())
    })?;
    let list = parse_apps_str(path, &data).map_err(|e| {
        ErrorCode::ConfigInvalid
            .error()
            .with_context("file", path.display().to_string())
            .with_context("error", e)
    })?;
    Ok(list.applications.len())
}

/// Filter applications by server compatibility, categories, and current OS.
pub fn filter_apps(
    server_only: bool,
    categories: Vec<Category>,
    apps_file: Option<&std::path::Path>,
    replace: bool,
) -> Vec<Application> {
    let all = load_apps(apps_file, replace);
    let system = SystemInfo::new();
    let os_flag = match system.os_type() {
        OSType::Linux => OsSupport::LINUX,
        OSType::Macos => OsSupport::MACOS,
        OSType::Windows => OsSupport::WINDOWS,
        _ => OsSupport::LINUX,
    };

    all.applications
        .into_iter()
        .filter(|app| {
            let os_match = app
                .supported_systems
                .iter()
                .any(|s| s.flags().contains(os_flag));
            let server_match = !server_only || app.server_compatible;
            let category_match =
                categories.is_empty() || app.categories.iter().any(|c| categories.contains(c));
            os_match && server_match && category_match
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Install
// ---------------------------------------------------------------------------

pub async fn install_apps_command(
    all: bool,
    server: bool,
    dry_run: bool,
    apps_file: Option<&std::path::Path>,
    replace: bool,
) -> Result<(), Error> {
    let apps = filter_apps(server, vec![], apps_file, replace);
    let system = SystemInfo::new();
    let current_os = system.os_type_raw();
    let mut failed = Vec::new();

    for app in apps {
        if app.is_installed() {
            println!("  ⏭  {}: already installed — skipping", app.name);
            continue;
        }

        if !all {
            let prompt = format!("Install {}?", app.name);
            let confirmed = tokio::task::spawn_blocking(move || {
                Confirm::new()
                    .with_prompt(prompt)
                    .default(true)
                    .interact()
                    .unwrap_or(false)
            })
            .await
            .map_err(|e| {
                Error::from_code(ErrorCode::ProcessFailure).with_context("prompt", e.to_string())
            })?;

            if !confirmed {
                println!("  ⏭  Skipping {}", app.name);
                continue;
            }
        }

        let method = find_install_method(&app, &current_os);
        match method {
            Some(m) => {
                println!("  🚀 Installing {}...", app.name);
                if m.install(dry_run) {
                    println!("  ✅ {}", app.name);
                } else {
                    println!("  ❌ Failed to install {}", app.name);
                    failed.push(app.name);
                }
            }
            None => println!("  ⚠️  No install method found for {} on this OS", app.name),
        }
    }

    if failed.is_empty() {
        Ok(())
    } else {
        Err(Error::from_code(ErrorCode::ProcessFailure).with_context("failed", failed.join(", ")))
    }
}

// ---------------------------------------------------------------------------
// Uninstall
// ---------------------------------------------------------------------------

pub async fn uninstall_apps_command(
    all: bool,
    server: bool,
    dry_run: bool,
    apps_file: Option<&std::path::Path>,
    replace: bool,
) -> Result<(), Error> {
    let apps = filter_apps(server, vec![], apps_file, replace);
    let system = SystemInfo::new();
    let current_os = system.os_type_raw();
    let mut failed = Vec::new();

    for app in apps {
        if !app.is_installed() {
            println!("  ⏭  {}: not installed — skipping", app.name);
            continue;
        }

        if !all {
            let prompt = format!("Uninstall {}?", app.name);
            let confirmed = tokio::task::spawn_blocking(move || {
                Confirm::new()
                    .with_prompt(prompt)
                    .default(false)
                    .interact()
                    .unwrap_or(false)
            })
            .await
            .map_err(|e| {
                Error::from_code(ErrorCode::ProcessFailure).with_context("prompt", e.to_string())
            })?;

            if !confirmed {
                println!("  ⏭  Skipping {}", app.name);
                continue;
            }
        }

        let method = find_install_method(&app, &current_os);
        match method {
            Some(m) => {
                println!("  🧹 Uninstalling {}...", app.name);
                if m.uninstall(dry_run) {
                    println!("  🗑️  {}", app.name);
                } else {
                    println!("  ❌ Failed to uninstall {}", app.name);
                    failed.push(app.name);
                }
            }
            None => println!(
                "  ⚠️  No uninstall method found for {} on this OS",
                app.name
            ),
        }
    }

    if failed.is_empty() {
        Ok(())
    } else {
        Err(Error::from_code(ErrorCode::ProcessFailure).with_context("failed", failed.join(", ")))
    }
}

// ---------------------------------------------------------------------------
// List / Categories
// ---------------------------------------------------------------------------

/// Row data for app listing — returned to callers for format-aware rendering.
pub struct AppListRow {
    pub name: String,
    pub categories: String,
    pub server: bool,
    pub installed: bool,
}

/// Return rows for app listing. Callers are responsible for rendering.
pub fn app_list_rows(
    server_only: bool,
    categories: Vec<Category>,
    apps_file: Option<&std::path::Path>,
    replace: bool,
) -> Vec<AppListRow> {
    filter_apps(server_only, categories, apps_file, replace)
        .into_iter()
        .map(|app| AppListRow {
            categories: app
                .categories
                .iter()
                .map(|c| format!("{:?}", c))
                .collect::<Vec<_>>()
                .join(", "),
            server: app.server_compatible,
            installed: app.is_installed(),
            name: app.name,
        })
        .collect()
}

/// Return category display names. Callers are responsible for rendering.
pub fn category_list_rows() -> Vec<String> {
    use strum::IntoEnumIterator;
    Category::iter().map(|c| c.display()).collect()
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn find_install_method<'a>(app: &'a Application, current_os: &OSType) -> Option<&'a InstallMethod> {
    for version in &app.versions {
        for method in &version.install_methods {
            if method.os.iter().any(|os| os.equals_ostype(current_os)) {
                return Some(method);
            }
        }
        // Fallback methods (no OS restriction means they apply anywhere)
        for method in &version.install_methods {
            if method.os.is_empty() || method.fallback {
                return Some(method);
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Default application definitions
// ---------------------------------------------------------------------------

fn default_apps() -> Vec<Application> {
    use crate::engine::models::application::{ApplicationVersion, InstallMethod};
    use crate::engine::models::package_manager::PackageManager;
    use crate::engine::models::system::SystemSupport;
    use os_info::Type as OSType;

    vec![
        Application::new(
            Some("fish-shell".to_string()),
            "Fish Shell".to_string(),
            true,
            vec![Category::Shells],
            vec![SystemSupport::MacLin],
            vec![ApplicationVersion {
                name: "Latest".to_string(),
                check_command: Some("fish".to_string()),
                dependencies: vec![],
                install_methods: vec![
                    InstallMethod {
                        fallback: false,
                        os: vec![OSType::Ubuntu.into(), OSType::Debian.into()],
                        package_manager: Some(PackageManager::Apt),
                        package_name: Some("fish".to_string()),
                        is_cask: None,
                        steps: None,
                    },
                    InstallMethod {
                        fallback: false,
                        os: vec![OSType::Fedora.into()],
                        package_manager: Some(PackageManager::Dnf),
                        package_name: Some("fish".to_string()),
                        is_cask: None,
                        steps: None,
                    },
                    InstallMethod {
                        fallback: false,
                        os: vec![OSType::SUSE.into()],
                        package_manager: Some(PackageManager::Zypper),
                        package_name: Some("fish".to_string()),
                        is_cask: None,
                        steps: None,
                    },
                    InstallMethod {
                        fallback: false,
                        os: vec![OSType::Arch.into(), OSType::Manjaro.into()],
                        package_manager: Some(PackageManager::Pacman),
                        package_name: Some("fish".to_string()),
                        is_cask: None,
                        steps: None,
                    },
                    InstallMethod {
                        fallback: false,
                        os: vec![OSType::Macos.into()],
                        package_manager: Some(PackageManager::Brew),
                        package_name: Some("fish".to_string()),
                        is_cask: None,
                        steps: None,
                    },
                ],
            }],
        ),
        Application::new(
            Some("alacritty".to_string()),
            "Alacritty".to_string(),
            false,
            vec![Category::TerminalEmulators],
            vec![SystemSupport::MacLin],
            vec![ApplicationVersion {
                name: "Latest".to_string(),
                check_command: Some("alacritty".to_string()),
                dependencies: vec![],
                install_methods: vec![
                    InstallMethod {
                        fallback: false,
                        os: vec![OSType::Ubuntu.into(), OSType::Debian.into()],
                        package_manager: Some(PackageManager::Apt),
                        package_name: Some("alacritty".to_string()),
                        is_cask: None,
                        steps: None,
                    },
                    InstallMethod {
                        fallback: false,
                        os: vec![OSType::Macos.into()],
                        package_manager: Some(PackageManager::Brew),
                        package_name: Some("alacritty".to_string()),
                        is_cask: Some(true),
                        steps: None,
                    },
                    InstallMethod {
                        fallback: false,
                        os: vec![OSType::Arch.into(), OSType::Manjaro.into()],
                        package_manager: Some(PackageManager::Pacman),
                        package_name: Some("alacritty".to_string()),
                        is_cask: None,
                        steps: None,
                    },
                ],
            }],
        ),
    ]
}
