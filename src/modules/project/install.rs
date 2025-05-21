use super::metadata::{self, metadata};
use crate::dprintln;
use crate::modules::version::Version;
use colored::Colorize;
use std::fmt::{self, Display};
use std::process::Command;
use std::str::FromStr;

#[derive(Default)]
pub struct InstallOptions {
    pub install_shell: ExecShell,
    pub install_mode: InstallMode,
}
#[derive(Default)]
pub enum InstallMode {
    #[default]
    Local,
    Global,
}
impl Display for InstallMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstallMode::Local => write!(f, "local"),
            InstallMode::Global => write!(f, "global"),
        }
    }
}
impl Display for InstallOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines = [
            format!("{}{}", "Install Options".cyan().bold(), ":"),
            format!(
                "  {}{} {}",
                "install-shell".green().bold(),
                ":",
                self.install_shell
            ),
            format!(
                "  {}{} {}",
                "install-mode".green().bold(),
                ":",
                self.install_mode
            ),
        ];
        for line in lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

#[derive(Default)]
pub enum ExecShell {
    #[default]
    RBash,
    Bash,
    Zsh,
    Csh,
}
impl FromStr for ExecShell {
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

impl Display for ExecShell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecShell::RBash => write!(f, "restricted bash"),
            ExecShell::Bash => write!(f, "bash"),
            ExecShell::Zsh => write!(f, "zsh"),
            ExecShell::Csh => write!(f, "csh"),
        }
    }
}

pub fn install(opts: InstallOptions) -> Result<(), String> {
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

    // Configure install shell
    fn setup_execshell(
        cmd: &mut Command,
        target_dir: &std::path::Path,
        project_name: &str,
        project_version: &Version,
        install_mode: &InstallMode,
    ) {
        cmd.current_dir(target_dir)
            .env("IPKG_PACKAGE_NAME", project_name)
            .env("IPKG_PACKAGE_VERSION", project_version.to_string())
            .env("IPKG_INSTALL_MODE", install_mode.to_string())
            .arg("ipkg/scripts/install.sh");
    }

    let mut install_process = match opts.install_shell {
        ExecShell::RBash => {
            let mut cmd = Command::new("bash");
            cmd.arg("-r");
            cmd
        }
        ExecShell::Bash => Command::new("bash"),
        ExecShell::Zsh => Command::new("zsh"),
        ExecShell::Csh => Command::new("csh"),
    };
    setup_execshell(
        &mut install_process,
        &target_dir,
        &project_metadata.about.package.name,
        &project_metadata.about.package.version,
        &opts.install_mode,
    );

    // Execute the install process and handle the result
    let status = install_process
        .status()
        .map_err(|e| format!("Failed to execute install process: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Install process failed with status: {}", status))
    }
}
