use super::metadata;
use crate::dprintln;
use colored::Colorize;
use ignore::gitignore::GitignoreBuilder;
use serde_yaml;
use std::fmt::{self, Display};
use std::path::Path;
use std::str::FromStr;
use walkdir::WalkDir;
use zip::CompressionMethod;
use zip::write::FileOptions;
use std::fs::File;
use zip::ZipWriter;

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

    // Prepare package directory
    let package_dir = target_dir.join("package");
    if package_dir.exists() {
        dprintln!(
            "Removing existing package directory: {}",
            package_dir.display()
        );
        std::fs::remove_dir_all(&package_dir)
            .map_err(|e| format!("Failed to remove '{}': {}", package_dir.display(), e))?;
    }
    std::fs::create_dir_all(&package_dir)
        .map_err(|e| format!("Failed to create '{}': {}", package_dir.display(), e))?;
    dprintln!("Package directory created: {}", package_dir.display());

    // Copy files to package directory, respecting ignore list
    copy_files(&target_dir, &package_dir, &ignore_list)?;

    // Create zip package
    package_data(
        &target_dir,
        &package_dir,
        &project_metadata.about.package.name,
        &project_metadata.about.package.version.to_string(),
    )
}

/// Copies files from source to destination, respecting .gitignore-style patterns.
fn copy_files(src: &Path, dst: &Path, ignore_list: &[String]) -> Result<(), String> {
    dprintln!("Copying files from {} to {}", src.display(), dst.display());

    // Build Gitignore from ignore_list
    let mut builder = GitignoreBuilder::new(src);
    for pattern in ignore_list {
        if let Err(_e) = builder.add_line(None, pattern) {
            dprintln!("Warning: Invalid ignore pattern '{}': {}", pattern, _e);
        }
    }
    let gitignore = builder
        .build()
        .map_err(|e| format!("Failed to build gitignore: {}", e))?;

    for entry in WalkDir::new(src).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path == src || path == dst {
            continue;
        }

        // Check if path is ignored
        let is_ignored = gitignore.matched(path, path.is_dir()).is_ignore();
        if is_ignored {
            dprintln!("Ignoring path: {}", path.display());
            continue;
        }

        // Compute destination path
        let relative_path = path
            .strip_prefix(src)
            .map_err(|e| format!("Failed to strip prefix from '{}': {}", path.display(), e))?;
        let dst_path = dst.join(relative_path);

        if path.is_dir() {
            std::fs::create_dir_all(&dst_path).map_err(|e| {
                format!("Failed to create directory '{}': {}", dst_path.display(), e)
            })?;
        } else if path.is_file()
            && relative_path
                .to_str()
                .unwrap()
                .to_string()
                .starts_with("package/")
        {
            std::fs::copy(path, &dst_path).map_err(|e| {
                format!(
                    "Failed to copy '{}' to '{}': {}",
                    path.display(),
                    dst_path.display(),
                    e
                )
            })?;
            dprintln!("Copied: {} -> {}", path.display(), dst_path.display());
        }
    }
    Ok(())
}

/// Creates a zip package from the package directory.
fn package_data(
    target_dir: &Path,
    package_dir: &Path,
    project_name: &str,
    project_version: &str,
) -> Result<(), String> {
    dprintln!("Starting package_data function.");

    // Create a single folder inside package_dir
    let folder_name = format!("{}-{}", project_name, project_version);
    let inner_dir = package_dir.join(&folder_name);
    std::fs::create_dir_all(&inner_dir).map_err(|e| {
        format!(
            "Failed to create inner directory '{}': {}",
            inner_dir.display(),
            e
        )
    })?;

    // Move all contents of package_dir to inner_dir
    for entry in std::fs::read_dir(package_dir)
        .map_err(|e| format!("Failed to read '{}': {}", package_dir.display(), e))?
    {
        let entry = entry
            .map_err(|e| format!("Failed to read entry in '{}': {}", package_dir.display(), e))?;
        let path = entry.path();
        if path == inner_dir {
            continue;
        }
        let dest = inner_dir.join(
            path.file_name()
                .ok_or_else(|| format!("Invalid file name for '{}'", path.display()))?,
        );
        std::fs::rename(&path, &dest).map_err(|e| {
            format!(
                "Failed to move '{}' to '{}': {}",
                path.display(),
                dest.display(),
                e
            )
        })?;
        dprintln!("Moved: {} -> {}", path.display(), dest.display());
    }

    // Create zip file path
    let zip_file_path = target_dir.join(format!("{}-{}.zip", project_name, project_version));

    // Call zip_process to create the zip file
    zip_process(&inner_dir, &zip_file_path)?;

    Ok(())
}

/// Zips the contents of from_path to to_path.
fn zip_process(from_path: &Path, to_path: &Path) -> Result<(), String> {
    dprintln!("Zipping {} to {}", from_path.display(), to_path.display());

    let file = File::create(to_path).map_err(|e| format!("Failed to create zip file '{}': {}", to_path.display(), e))?;
    let mut zip = ZipWriter::new(file);

    let prefix = from_path.file_name().unwrap().to_string_lossy();

    for entry in WalkDir::new(from_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let relative_path = path.strip_prefix(from_path).unwrap();
            let zip_path = format!("{}/{}", prefix, relative_path.to_string_lossy());
            let options = FileOptions::<'_, ()>::default().compression_method(CompressionMethod::Deflated);
            zip.start_file(&zip_path, options).map_err(|e| format!("Failed to start file '{}' in zip: {}", zip_path, e))?;
            let mut f = File::open(path).map_err(|e| format!("Failed to open file '{}': {}", path.display(), e))?;
            std::io::copy(&mut f, &mut zip).map_err(|e| format!("Failed to copy file '{}' to zip: {}", path.display(), e))?;
        }
    }
    zip.finish().map_err(|e| format!("Failed to finish zip: {}", e))?;
    Ok(())
}