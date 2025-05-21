use super::ExecShell;
use super::metadata::{self, metadata};
use crate::dprintln;
use crate::modules::version::Version;
use colored::Colorize;
use std::fmt::{self, Display};
use std::process::Command;

#[derive(Default)]
pub struct PurgeOptions {
    pub purge_shell: ExecShell,
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
    fn setup_execshell(
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

    let mut purge_process = opts.purge_shell.generate();
    setup_execshell(
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
