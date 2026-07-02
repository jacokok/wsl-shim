use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, ExitCode};

/// Check if wsl-shim.toml has login = true
fn use_login_shell(exe_dir: &Path) -> bool {
    let config_path = exe_dir.join("wsl-shim.toml");
    let content = match fs::read_to_string(&config_path) {
        Ok(c) => c,
        Err(_) => return false,
    };

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }
        if let Some((key, value)) = trimmed.split_once('=') {
            if key.trim() == "login" {
                return value.trim() == "true";
            }
        }
    }

    false
}

fn main() -> ExitCode {
    // Figure out what command to run based on this exe's name.
    let exe_path = env::current_exe().unwrap_or_default();
    let exe_dir = exe_path.parent().unwrap_or(Path::new("."));

    let command = Path::new(&exe_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("docker")
        .to_string();

    // Grab all the args that were passed to us
    let args: Vec<String> = env::args().skip(1).collect();

    let wsl_args = if use_login_shell(exe_dir) {
        // Run via bash login shell so profile-dependent tools are available
        let mut shell_cmd = command.clone();
        for arg in &args {
            let escaped = arg.replace('\'', "'\\''");
            shell_cmd.push_str(&format!(" '{}'", escaped));
        }
        vec!["-e".to_string(), "bash".to_string(), "-lc".to_string(), shell_cmd]
    } else {
        // Direct exec, no shell interpretation
        let mut wsl_args: Vec<String> = vec!["-e".to_string(), command];
        wsl_args.extend(args);
        wsl_args
    };

    let status = Command::new("wsl")
        .args(&wsl_args)
        .status();

    match status {
        Ok(s) => ExitCode::from(s.code().unwrap_or(1) as u8),
        Err(e) => {
            eprintln!("wsl-shim: failed to execute wsl: {e}");
            ExitCode::from(1)
        }
    }
}
