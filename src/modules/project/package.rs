use super::ExecShell;
use super::metadata;
use crate::dprintln;
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
    dprintln!("{}", &opts); // デバッグ情報を出力

    // プロジェクトのメタデータディレクトリを取得
    let target_dir = metadata::get_dir().map_err(|_| {
        "Error: Couldn't find Ipkg Directory. Make sure you are in an ipkg project.".to_string()
    })?;

    // プロジェクトのメタデータを読み込む
    let project_metadata = metadata::metadata()
        .map_err(|e| format!("Error: Failed to read project metadata: {:?}", e))?;

    // シェルコマンドの準備
    let mut package_process = opts.package_shell.generate();
    package_process
        .current_dir(&target_dir) // プロジェクトディレクトリを作業ディレクトリに設定
        .env("IPKG_PROJECT_NAME", &project_metadata.about.package.name) // パッケージ名を環境変数に設定
        .env(
            "IPKG_PROJECT_VERSION",
            project_metadata.about.package.version.to_string(),
        ) // パッケージバージョンを環境変数に設定
        .env("IPKG_PROJECT_TARGET", opts.target.to_string()) // パッケージターゲットを環境変数に設定
        .arg("ipkg/scripts/package.sh"); // 実行するスクリプトのパス

    // パージ処理の実行
    let status = package_process
        .status()
        .map_err(|e| format!("Failed to execute package process: {}", e))?;

    // 実行結果の確認
    if status.success() {
        Ok(())
    } else {
        // エラーコードがあればそれも表示
        let code_info = status
            .code()
            .map_or("".to_string(), |c| format!(" (exit code: {})", c));
        Err(format!(
            "Package process failed with status: {}{}",
            status, code_info
        ))
    }
}
