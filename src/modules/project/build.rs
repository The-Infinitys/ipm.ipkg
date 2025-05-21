use colored::Colorize;
use std::fmt::{self, Display};

#[derive(Default)]
pub struct BuildOptions {
    pub build_mode: BuildMode,
    pub build_shell: BuildShell,
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
pub enum BuildShell {
    #[default]
    RBash,
}
impl Display for BuildShell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildShell::RBash => write!(f, "restricted bash"),
        }
    }
}
pub fn build(opts: BuildOptions) -> Result<(), String> {
    println!("{}", opts);
    Ok(())
}
