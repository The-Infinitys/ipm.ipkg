use super::metadata::{self, metadata};
use crate::dprintln;
use crate::modules::version::Version;
use colored::Colorize;
use std::fmt::{self, Display};
use std::process::Command;
use std::str::FromStr;

#[derive(Default)]
pub struct PurgeOptions {
    pub purge_shell: PurgeShell,
}

impl Display for PurgeOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines = [
            format!("{}{}", "Purge Options".cyan().bold(), ":"),
            format!(
                "  {}{} {}",
                "purge-shell".green().bold(),
                ":",
                self.purge_shell
            ),
        ];
        for line in lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

#[derive(Default)]
pub enum PurgeShell {
    #[default]
    RBash,
    Bash,
    Zsh,
    Csh,
}
impl FromStr for PurgeShell {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "bash" => Ok(Self::Bash),
            "zsh" => Ok(Self::Zsh),
            "csh" => Ok(Self::Csh),
            "rbash" => Ok(Self::RBash),
            _ => Err(format!("Unavailable Shell: {}", s)),
        }
    }
}

impl Display for PurgeShell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PurgeShell::RBash => write!(f, "restricted bash"),
            PurgeShell::Bash => write!(f, "bash"),
            PurgeShell::Zsh => write!(f, "zsh"),
            PurgeShell::Csh => write!(f, "csh"),
        }
    }
}

pub fn purge(opts: PurgeOptions) -> Result<(), String> {
    dprintln!("{}", &opts);
    let target_dir = metadata::get_dir();
    let target_dir = match target_dir {
        Ok(path) => path,
        Err(()) => {
            let msg = "Error: Couldn't find Ipkg Directory".to_string();
            eprintln!("{}", msg);
            return Err(msg);
        }
    };
    let project_metadata = metadata().unwrap();

    // Configure purge shell
    fn setup_purgeshell(
        cmd: &mut Command,
        target_dir: &std::path::Path,
        project_name: &str,
        project_version: &Version,
    ) {
        cmd.current_dir(target_dir)
            .env("IPKG_PACKAGE_NAME", project_name)
            .env("IPKG_PACKAGE_VERSION", project_version.to_string())
            .arg("ipkg/scripts/purge.sh");
    }

    let mut purge_process = match opts.purge_shell {
        PurgeShell::RBash => {
            let mut cmd = Command::new("bash");
            cmd.arg("-r");
            cmd
        }
        PurgeShell::Bash => Command::new("bash"),
        PurgeShell::Zsh => Command::new("zsh"),
        PurgeShell::Csh => Command::new("csh"),
    };
    setup_purgeshell(
        &mut purge_process,
        &target_dir,
        &project_metadata.about.package.name,
        &project_metadata.about.package.version,
    );

    // Execute the purge process and handle the result
    let status = purge_process
        .status()
        .map_err(|e| format!("Failed to execute purge process: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Purge process failed with status: {}", status))
    }
}
