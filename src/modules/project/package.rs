use super::metadata;
use crate::dprintln;
use colored::Colorize;
use ignore::gitignore::GitignoreBuilder;
use serde_yaml;
use std::fmt::{self, Display};
use std::path::Path;
use std::str::FromStr;
use walkdir::WalkDir;

/// Defines the options for the packaging process.
#[derive(Debug, Default)]
pub struct PackageOptions {
    /// The target type for the package (e.g., source build, normal, minimal).
    pub target: PackageTarget,
}

/// Represents the different packaging targets.
#[derive(Debug, Clone, Copy, Default)]
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
        writeln!(f, "{}:", "Package Options".cyan().bold())?;
        writeln!(f, "  {}: {}", "target".green().bold(), self.target)?;
        Ok(())
    }
}

/// Structure to deserialize project-ignore.yaml
#[derive(serde::Deserialize)]
struct ProjectIgnore {
    #[serde(rename = "source-build")]
    source_build: Vec<String>,
    normal: Vec<String>,
    min: Vec<String>,
}

/// Initiates the packaging process based on the provided options.
pub fn package(opts: PackageOptions) -> Result<(), String> {
    dprintln!("Starting packaging process with options: {}", &opts);

    // Get project directory
    let target_dir = metadata::get_dir().map_err(|e| {
        format!(
            "Error: Couldn't find Ipkg Directory. Make sure you are in an ipkg project. Details: {:?}", 
            e
        )
    })?;
    dprintln!("Project directory: {}", target_dir.display());

    // Load project metadata
    let project_metadata = metadata::metadata()
        .map_err(|e| format!("Error: Failed to read project metadata: {:?}", e))?;
    dprintln!(
        "Project metadata loaded for: {} version {}",
        project_metadata.about.package.name,
        project_metadata.about.package.version
    );

    // Load project-ignore.yaml
    let ignore_file = target_dir.join("ipkg").join("project-ignore.yaml");
    let ignore_config: ProjectIgnore = if ignore_file.exists() {
        let file = std::fs::File::open(&ignore_file)
            .map_err(|e| format!("Failed to open '{}': {}", ignore_file.display(), e))?;
        serde_yaml::from_reader(file)
            .map_err(|e| format!("Failed to parse '{}': {}", ignore_file.display(), e))?
    } else {
        dprintln!(
            "Warning: '{}' not found, using empty ignore lists",
            ignore_file.display()
        );
        ProjectIgnore {
            source_build: vec![],
            normal: vec![],
            min: vec![],
        }
    };

    // Select ignore list based on PackageTarget
    let ignore_list: Vec<String> = match opts.target {
        PackageTarget::SourceBuild => ignore_config.source_build,
        PackageTarget::Normal => {
            let mut list = ignore_config.source_build;
            list.extend(ignore_config.normal);
            list
        }
        PackageTarget::Min => {
            let mut list = ignore_config.source_build;
            list.extend(ignore_config.normal);
            list.extend(ignore_config.min);
            list
        }
    };

    dprintln!("Ignore list for target {}: {:?}", opts.target, ignore_list);
    Ok(())
}
