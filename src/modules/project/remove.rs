use super::metadata::{self, metadata};
use crate::dprintln;
use crate::modules::version::Version;
use colored::Colorize;
use std::fmt::{self, Display};
use std::process::Command;
use std::str::FromStr;

#[derive(Default)]
pub struct RemoveOptions {
    pub remove_shell: RemoveShell,
}

impl Display for RemoveOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines = [
            format!("{}{}", "Remove Options".cyan().bold(), ":"),
            format!(
                "  {}{} {}",
                "remove-shell".green().bold(),
                ":",
                self.remove_shell
            ),
        ];
        for line in lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

#[derive(Default)]
pub enum RemoveShell {
    #[default]
    RBash,
    Bash,
    Zsh,
    Csh,
}
impl FromStr for RemoveShell {
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

impl Display for RemoveShell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RemoveShell::RBash => write!(f, "restricted bash"),
            RemoveShell::Bash => write!(f, "bash"),
            RemoveShell::Zsh => write!(f, "zsh"),
            RemoveShell::Csh => write!(f, "csh"),
        }
    }
}

pub fn remove(opts: RemoveOptions) -> Result<(), String> {
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

    // Configure remove shell
    fn setup_removeshell(
        cmd: &mut Command,
        target_dir: &std::path::Path,
        project_name: &str,
        project_version: &Version,
    ) {
        cmd.current_dir(target_dir)
            .env("IPKG_PACKAGE_NAME", project_name)
            .env("IPKG_PACKAGE_VERSION", project_version.to_string())
            .arg("ipkg/scripts/remove.sh");
    }

    let mut remove_process = match opts.remove_shell {
        RemoveShell::RBash => {
            let mut cmd = Command::new("bash");
            cmd.arg("-r");
            cmd
        }
        RemoveShell::Bash => Command::new("bash"),
        RemoveShell::Zsh => Command::new("zsh"),
        RemoveShell::Csh => Command::new("csh"),
    };
    setup_removeshell(
        &mut remove_process,
        &target_dir,
        &project_metadata.about.package.name,
        &project_metadata.about.package.version,
    );

    // Execute the remove process and handle the result
    let status = remove_process
        .status()
        .map_err(|e| format!("Failed to execute remove process: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Remove process failed with status: {}", status))
    }
}
