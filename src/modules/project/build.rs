use super::metadata::{self, metadata};
use crate::dprintln;
use crate::modules::version::Version;
use colored::Colorize;
use std::fmt::{self, Display};
use std::process::Command;
use std::str::FromStr;

#[derive(Default)]
pub struct BuildOptions {
    pub build_mode: BuildMode,
    pub build_shell: ExecShell,
}

impl Display for BuildOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines = [
            format!("{}{}", "Build Options".cyan().bold(), ":"),
            format!(
                "  {}{} {}",
                "build-mode".green().bold(),
                ":",
                self.build_mode
            ),
            format!(
                "  {}{} {}",
                "build-shell".green().bold(),
                ":",
                self.build_shell
            ),
        ];
        for line in lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

#[derive(Default)]
pub enum BuildMode {
    Release,
    #[default]
    Debug,
}

impl Display for BuildMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildMode::Release => write!(f, "release"),
            BuildMode::Debug => write!(f, "debug"),
        }
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

pub fn build(opts: BuildOptions) -> Result<(), String> {
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

    // Configure build shell
    fn setup_execshell(
        cmd: &mut Command,
        target_dir: &std::path::Path,
        project_name: &str,
        project_version: &Version,
        build_mode: &BuildMode,
    ) {
        let build_mode = build_mode.to_string();
        cmd.current_dir(target_dir)
            .env("IPKG_PACKAGE_NAME", project_name)
            .env("IPKG_PACKAGE_VERSION", project_version.to_string())
            .env("IPKG_BUILD_MODE", build_mode)
            .arg("ipkg/scripts/build.sh");
    }

    let mut build_process = match opts.build_shell {
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
        &mut build_process,
        &target_dir,
        &project_metadata.about.package.name,
        &project_metadata.about.package.version,
        &opts.build_mode,
    );

    // Execute the build process and handle the result
    let status = build_process
        .status()
        .map_err(|e| format!("Failed to execute build process: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Build process failed with status: {}", status))
    }
}
