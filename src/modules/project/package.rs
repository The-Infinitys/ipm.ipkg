use colored::Colorize;
use std::fmt::{self, Display};
use std::str::FromStr;
#[derive(Default)]
pub struct PackageOptions {
    pub target: PackageTarget,
    pub package_shell: PackageShell,
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
#[derive(Default)]
pub enum PackageShell {
    #[default]
    RBash,
    Bash,
    Zsh,
    Csh,
}
impl FromStr for PackageShell {
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

impl Display for PackageShell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackageShell::RBash => write!(f, "restricted bash"),
            PackageShell::Bash => write!(f, "bash"),
            PackageShell::Zsh => write!(f, "zsh"),
            PackageShell::Csh => write!(f, "csh"),
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
