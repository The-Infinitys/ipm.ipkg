use super::ExecShell;
use super::metadata;
// The dprintln! macro should be defined in one place (e.g., src/utils/debug.rs)
// and marked with #[macro_export].
// Then it can be used throughout the crate.
// We remove the duplicate definition from this file.
// If src/utils/debug.rs contains:
// #[macro_export]
// macro_rules! dprintln { ... }
// And your lib.rs or main.rs has `pub mod utils;` (and utils/mod.rs has `pub mod debug;`)
// then it should be available. Or if dprintln is re-exported from utils.

use colored::Colorize;
use std::fmt::{self, Display};
// std::io::Write was unused, so it's removed.
use crate::dprintln;
use std::str::FromStr;
use zip::CompressionMethod; // Import CompressionMethod
use zip::write::FileOptions; // Import FileOptions // Assuming dprintln! is defined in your crate
/// Defines the options for the packaging process.
#[derive(Default)] // Added Debug
pub struct PackageOptions {
    /// The target type for the package (e.g., source build, normal, minimal).
    pub target: PackageTarget,
    /// The shell to be used for the packaging process.
    pub package_shell: ExecShell,
}

/// Represents the different packaging targets.
#[derive(Default, Debug, Clone, Copy)]
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
        // Using Debug derive for PackageOptions, so direct formatting might be simpler if needed.
        // This Display impl is fine for custom formatting.
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
pub fn package(opts: PackageOptions) -> Result<(), String> {
    dprintln!("Starting packaging process with options: {}", &opts);

    let target_dir = metadata::get_dir().map_err(|e| {
        format!("Error: Couldn't find Ipkg Directory. Make sure you are in an ipkg project. Details: {:?}", e)
    })?;
    dprintln!("Project directory: {}", target_dir.display());

    let project_metadata = metadata::metadata()
        .map_err(|e| format!("Error: Failed to read project metadata: {:?}", e))?;
    dprintln!(
        "Project metadata loaded for: {} version {}",
        project_metadata.about.package.name,
        project_metadata.about.package.version
    );

    let mut package_process_cmd = opts.package_shell.generate();
    package_process_cmd
        .current_dir(&target_dir)
        .env("IPKG_PROJECT_NAME", &project_metadata.about.package.name)
        .env(
            "IPKG_PROJECT_VERSION",
            project_metadata.about.package.version.to_string(),
        )
        .env("IPKG_PROJECT_TARGET", opts.target.to_string());

    let script_path = target_dir.join("ipkg").join("scripts").join("package.sh");
    package_process_cmd.arg(&script_path);
    dprintln!("Executing script: {:?}", &package_process_cmd);

    let status = package_process_cmd.status().map_err(|e| {
        format!(
            "Failed to execute package script '{}': {}",
            script_path.display(),
            e
        )
    })?;

    if status.success() {
        dprintln!("Package script executed successfully.");
        package_data()
    } else {
        let code_info = status.code().map_or_else(
            || "killed by signal".to_string(),
            |c| format!("exit code: {}", c),
        );
        Err(format!(
            "Package script failed with status: {}. {}",
            status, code_info
        ))
    }
}

fn package_data() -> Result<(), String> {
    dprintln!("Starting package_data function.");
    let target_dir = metadata::get_dir().map_err(|e| {
        format!("Error: Couldn't find Ipkg Directory (in package_data). Make sure you are in an ipkg project. Details: {:?}", e)
    })?;
    let package_dir = target_dir.join("package");
    dprintln!("Package source directory: {}", package_dir.display());

    if !package_dir.exists() {
        dprintln!(
            "Package directory does not exist. Creating: {}",
            package_dir.display()
        );
        std::fs::create_dir_all(&package_dir).map_err(|e| {
            format!(
                "Failed to create package directory '{}': {}",
                package_dir.display(),
                e
            )
        })?;
    } else if !package_dir.is_dir() {
        return Err(format!(
            "Error: Expected '{}' to be a directory, but it's not.",
            package_dir.display()
        ));
    }

    let project_metadata = metadata::metadata().map_err(|e| {
        format!(
            "Error: Failed to read project metadata (in package_data): {:?}",
            e
        )
    })?;

    let mut files_to_compress: Vec<std::path::PathBuf> = Vec::new();

    if package_dir.is_dir() {
        for entry_result in std::fs::read_dir(&package_dir).map_err(|e| {
            format!(
                "Failed to read package directory '{}': {}",
                package_dir.display(),
                e
            )
        })? {
            let entry = entry_result.map_err(|e| {
                format!(
                    "Failed to read directory entry in '{}': {}",
                    package_dir.display(),
                    e
                )
            })?;
            let path = entry.path();
            if path.is_file() {
                files_to_compress.push(path);
            } else if path.is_dir() {
                dprintln!(
                    "Skipping subdirectory during flat file collection: {}",
                    path.display()
                );
            }
        }
    }
    dprintln!("Files to compress: {:?}", files_to_compress);
    if files_to_compress.is_empty() {
        dprintln!(
            "Warning: No files found in package directory '{}' to compress.",
            package_dir.display()
        );
    }

    let package_file_name = format!(
        "{}-version-{}.ipkg",
        project_metadata.about.package.name, project_metadata.about.package.version,
    );
    dprintln!("Output package file name: {}", package_file_name);

    let ipkg_file_path = target_dir.join(&package_file_name);
    dprintln!("Full path for .ipkg file: {}", ipkg_file_path.display());

    let ipkg_file = std::fs::File::create(&ipkg_file_path).map_err(|e| {
        format!(
            "Failed to create ipkg file '{}': {}",
            ipkg_file_path.display(),
            e
        )
    })?;

    let mut zip = zip::ZipWriter::new(ipkg_file);
    dprintln!("ZipWriter created for file: {}", ipkg_file_path.display());
    let options: FileOptions<'_, ()> =
        FileOptions::default().compression_method(CompressionMethod::Deflated);

    for file_path in &files_to_compress {
        let relative_path = file_path.strip_prefix(&package_dir).map_err(|e| {
            format!(
                "Failed to strip prefix from file path: '{:?}' (base: '{:?}' Error: {})",
                file_path, package_dir, e
            )
        })?;

        let path_in_zip = match relative_path.to_str() {
            Some(s) => s,
            None => {
                dprintln!("Warning: Skipping non-UTF8 path: {:?}", relative_path);
                continue;
            }
        };

        dprintln!(
            "Adding to zip: '{}' from '{}'",
            path_in_zip,
            file_path.display()
        );
        zip.start_file(path_in_zip, options)
            .map_err(|e| format!("Failed to start file '{}' in zip: {}", path_in_zip, e))?;

        let mut f = std::fs::File::open(file_path).map_err(|e| {
            format!(
                "Failed to open file for zipping '{}': {}",
                file_path.display(),
                e
            )
        })?;

        std::io::copy(&mut f, &mut zip).map_err(|e| {
            format!(
                "Failed to copy file content to zip for '{}': {}",
                file_path.display(),
                e
            )
        })?;
    }

    zip.finish().map_err(|e| {
        format!(
            "Failed to finish writing zip file '{}': {}",
            ipkg_file_path.display(),
            e
        )
    })?;

    dprintln!(
        "Package created successfully at: {}",
        ipkg_file_path.display()
    );
    Ok(())
}
