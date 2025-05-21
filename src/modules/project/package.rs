use super::ExecShell;
use colored::Colorize;
use std::fmt::{self, Display};
use std::str::FromStr;
#[derive(Default)]
pub struct PackageOptions {
    pub target: PackageTarget,
    pub package_shell: ExecShell,
}
#[derive(Default)]
pub enum PackageTarget {
    SourceBuild,
    #[default]
    Normal,
    Min,
}
impl Display for PackageTarget {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines = [
            format!("{}{}", "Package Options".cyan().bold(), ":"),
            format!("  {}{} {}", "target".green().bold(), ":", self.target),
            format!(
                "  {}{} {}",
                "package-shell".green().bold(),
                ":",
                self.package_shell
            ),
        ];
        for line in lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}
pub fn package(opts: PackageOptions) -> Result<(), String> {
    println!("{}", opts);
    Ok(())
}
