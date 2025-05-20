// src/applications.rs

use crate::categories::Category;
use crate::print::{print_error, print_info};
use crate::system::{OsSupport, SystemInfo, SystemSupport};
use colored::Colorize;
use os_info::Type as OSType;
use tabled::settings::{Style};
use tabled::{Table, Tabled};
struct Application {
    name: &'static str,
    display_name: &'static str,
    supported: &'static [SystemSupport],
    categories: &'static [Category],
    is_server_app: bool,
}

#[derive(Tabled)]
struct DisplayApp<'a> {
    Name: &'a str,
    Categories: String,
    Server: bool,
}
static APPS: &[Application] = &[
    Application {
        name: "nerd-fonts",
        display_name: "Nerd Fonts",
        categories: &[Category::Fonts],
        supported: &[SystemSupport::Cross],
        is_server_app: true,
    },
    Application {
        name: "nginx",
        display_name: "Nginx",
        categories: &[Category::Servers],
        supported: &[SystemSupport::Linux],
        is_server_app: true,
    },
    Application {
        name: "docker",
        display_name: "Docker",
        categories: &[Category::Development, Category::Containerization],
        supported: &[SystemSupport::Cross],
        is_server_app: false,
    },
    Application {
        name: "podman",
        display_name: "Podman",
        categories: &[Category::Development, Category::Containerization],
        supported: &[SystemSupport::Cross],
        is_server_app: true,
    },
    Application {
        name: "kubernetes",
        display_name: "Kubernetes",
        categories: &[Category::Development, Category::Containerization],
        supported: &[SystemSupport::Cross],
        is_server_app: true,
    },
    Application {
        name: "vagrant",
        display_name: "Vagrant",
        categories: &[Category::Development, Category::Containerization],
        supported: &[SystemSupport::Cross],
        is_server_app: false,
    },
];

pub fn list_supported_application_for_current_os(
    server_only: bool,
    category_filter: Vec<Category>,
) {
    let system = SystemInfo::new();
    let os_flag = match system.os_type() {
        OSType::Linux => OsSupport::LINUX,
        OSType::Windows => OsSupport::WINDOWS,
        OSType::Macos => OsSupport::MACOS,
        _ => {
            print_error(format!("Unsupported OS: {:?}", system.os_type()));
            return;
        }
    };

    if !category_filter.is_empty() {
        let joined = category_filter
            .iter()
            .map(|c| format!("{:?}", c))
            .collect::<Vec<_>>()
            .join(", ");
        print_info(format!(
            "{} applications supported on {:?} in categories: {}",
            if server_only { "Server" } else { "All" },
            system.os_type(),
            joined
        ));
    } else {
        print_info(format!(
            "{} applications supported on {:?}",
            if server_only { "Server" } else { "All" },
            system.os_type()
        ));
    }

    let mut rows = vec![];

    for app in APPS.iter() {
        let is_supported = app.supported.iter().any(|s| s.flags().contains(os_flag));
        let is_server = app.is_server_app;

        let matches_category = if category_filter.is_empty() {
            true
        } else {
            app.categories
                .iter()
                .any(|cat| category_filter.contains(cat))
        };

        if is_supported && (!server_only || is_server) && matches_category {
            rows.push(DisplayApp {
                Name: app.display_name,
                Categories: app
                    .categories
                    .iter()
                    .map(|c| format!("{:?}", c))
                    .collect::<Vec<_>>()
                    .join(", "),
                Server: is_server,
            });
        }
    }

    let mut table = Table::new(rows);
    table
        .with(Style::modern_rounded());

    println!("{}", table);
}
