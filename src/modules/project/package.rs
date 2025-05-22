use super::ExecShell;
use colored::Colorize;
use std::fmt::{self, Display};
use std::str::FromStr;

/// Defines the options for the packaging process.
#[derive(Default)]
pub struct PackageOptions {
    /// The target type for the package (e.g., source build, normal, minimal).
    pub target: PackageTarget,
    /// The shell to be used for the packaging process.
    pub package_shell: ExecShell,
}

/// Represents the different packaging targets.
#[derive(Default)]
pub enum PackageTarget {
    /// Builds from source.
    SourceBuild,
    /// Standard package.
    #[default]
    Normal,
    /// Minimal package.
    Min,
}

impl Display for PackageTarget {
    /// Formats the `PackageTarget` for display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackageTarget::SourceBuild => write!(f, "source-build"),
            PackageTarget::Normal => write!(f, "normal"),
            PackageTarget::Min => write!(f, "minimal"),
        }
    }
}

impl FromStr for PackageTarget {
    type Err = String;

    /// Parses a string into a `PackageTarget`.
    ///
    /// This allows converting user input (e.g., "src", "normal") into the
    /// corresponding `PackageTarget` enum variant.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "src" | "source" | "source-build" => Ok(Self::SourceBuild),
            "normal" | "default" => Ok(Self::Normal),
            "min" | "minimal" => Ok(Self::Min),
            _ => Err(format!("Invalid Package Target: {}", s)),
        }
    }
}

impl Display for PackageOptions {
    /// Formats the `PackageOptions` for display, including the target and shell.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}:", "Package Options".cyan().bold())?;
        writeln!(f, "  {}: {}", "target".green().bold(), self.target)?;
        writeln!(
            f,
            "  {}: {}",
            "package-shell".green().bold(),
            self.package_shell
        )?;
        Ok(())
    }
}

/// Initiates the packaging process based on the provided options.
///
/// **Note**: This function is currently a placeholder and does not perform
/// any actual packaging. It only prints the options and a message indicating
/// that the functionality is not yet available.
///
/// # Arguments
///
/// * `opts` - A `PackageOptions` struct containing the desired packaging settings.
///
/// # Returns
///
/// Always returns `Ok(())` for now, but in a full implementation, it would
/// return `Result<(), String>` to indicate success or an error message.
pub fn package(opts: PackageOptions) -> Result<(), String> {
    println!("{}", opts);
    eprintln!("Sorry, this function is not available yet.");
    Ok(())
}
