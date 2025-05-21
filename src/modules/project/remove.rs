use super::ExecShell;
use super::metadata::{self, metadata};
use crate::dprintln;
use crate::modules::version::Version;
use colored::Colorize;
use std::fmt::{self, Display};
use std::process::Command;

#[derive(Default)]
pub struct RemoveOptions {
    pub remove_shell: ExecShell,
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
    fn setup_execshell(
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

    let mut remove_process = opts.remove_shell.generate();
    setup_execshell(
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
