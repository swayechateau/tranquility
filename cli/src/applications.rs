// src/applications.rs

use crate::categories::{Category, DevelopmentCategory}; 
use crate::system::SystemSupport;
struct Application {
    name: &'static str,
    supported: &'static [SystemSupport],
    categories: &'static [Category],
    is_server_app: bool,
}

static APPS: &[Application] = &[
    Application { 
        name: "nerd-fonts", 
        categories: &[Category::Fonts],  
        supported: &[SystemSupport::Cross], 
        is_server_app: true
    },
    Application { 
        name: "nginx", 
        categories: &[Category::Servers],  
        supported: &[SystemSupport::Linux], 
        is_server_app: true 
    },
    Application { 
        name: "docker", 
        categories: &[Category::Development(DevelopmentCategory::Containerization)], 
        supported: &[SystemSupport::Cross], 
        is_server_app: false 
    },
    Application { 
        name: "podman",
        categories: &[Category::Development(DevelopmentCategory::Containerization)],
        supported: &[SystemSupport::Cross],
        is_server_app: true
    },
    Application { 
        name: "kubernetes",
        categories: &[Category::Development(DevelopmentCategory::Containerization)],
        supported: &[SystemSupport::Cross],
        is_server_app: true
    },
    Application { 
        name: "vagrant",
        categories: &[Category::Development(DevelopmentCategory::Containerization)],
        supported: &[SystemSupport::Cross],
        is_server_app: false
    },

];