// src/installer.rs

pub fn install_apps(all: bool, server: bool) {
    // if server and not supported server print a warning unsupported server variable proceed at own risk
    // if server and all set - install all server app
    // if not server but all set - install all apps 
    // if not all or server, loop through categories and ask user which catergory do they wish to install apps from
    // add them to the list and proceed with the install
}

pub fn uninstall_apps(all:bool, server:bool) {
    // if server and not supported server print a warning unsupported server variable proceed at own risk
    // if server and all set - uninstall all server apps
    // if not server but all set - uninstall all apps 
}


/// Run a shell command under `sh -c` (Unix) or via PowerShell (Windows).
/// Exits on error.
pub fn run_shell_command(command: &str) {
    println!("🚀 Running: {}", command);

    // Spawn via PowerShell on Windows, `sh -c` elsewhere
    let status_res = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(&["-Command", command])
            .status()
    } else {
        Command::new("sh")
            .args(&["-c", command])
            .status()
    };

    match status_res {
        // Everything succeeded
        Ok(status) if status.success() => return,

        // The command ran but exited with a non-zero code
        Ok(status) => {
            let code = status.code().map_or(-1, |c| c);
            eprintln!(
                "❌ Command `{}` exited with non-zero status code: {}",
                command, code
            );
            std::process::exit(1);
        }

        // Failed to spawn / execute at all
        Err(error) => {
            eprintln!("❌ Failed to execute `{}`: {}", command, error);
            std::process::exit(1);
        }
    }
}

/// Convenience for calling PowerShell directly.
pub fn run_powershell_command(command: &str) {
    if let Err(err) = Command::new("powershell")
        .args(&["-Command", command])
        .status()
    {
        eprintln!("❌ PowerShell command failed `{}`: {}", command, err);
        std::process::exit(1);
    }
}