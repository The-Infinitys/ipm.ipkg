use super::super::system::path;
use super::PackageData;
use crate::utils::shell;
use chrono::{DateTime, Local};
use cmd_arg::cmd_arg::Option;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fmt::{self, Display, Formatter};
use std::fs; // fs::File を fs に変更して、fs::read_to_string も利用可能にする
use std::io; // io::Error を利用するために追加
use std::path::PathBuf;

/// インストールされているパッケージリストのメタデータを表します。
#[derive(Serialize, Deserialize)]
pub struct PackageListData {
    pub last_modified: DateTime<Local>,
    // `installed_packages` にリネームして複数形を明示
    pub installed_packages: Vec<InstalledPackageData>,
}

impl Default for PackageListData {
    fn default() -> Self {
        Self {
            last_modified: Local::now(),
            installed_packages: Vec::new(), // デフォルトは空のVec
        }
    }
}

/// 個々のインストール済みパッケージの詳細情報を表します。
#[derive(Serialize, Deserialize, Default)] // Default をderive
pub struct InstalledPackageData {
    pub info: PackageData,
    pub last_modified: DateTime<Local>,
    pub is_auto_installed: bool,
}

impl PackageListData {
    /// 指定されたパスからパッケージリストデータを読み込みます。
    ///
    /// # 引数
    /// * `list_filepath` - 読み込むパッケージリストファイルのパス。
    ///
    /// # 戻り値
    /// 読み込みとパースが成功した場合は `PackageListData` を、失敗した場合は `io::Error` を返します。
    fn from_filepath(list_filepath: &PathBuf) -> Result<PackageListData, io::Error> {
        let packageslist_str = fs::read_to_string(list_filepath).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!(
                    "Failed to read packages list file '{}': {}",
                    list_filepath.display(),
                    e
                ),
            )
        })?;

        serde_yaml::from_str(&packageslist_str).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Failed to parse packages list file '{}': {}",
                    list_filepath.display(),
                    e
                ),
            )
        })
    }
}

impl Display for PackageListData {
    /// `PackageListData` を整形して表示します。
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}: {}",
            "Last Modified".green().bold(),
            self.last_modified.to_rfc3339()
        )?;
        writeln!(f, "{}:", "Packages".cyan().bold())?; // 改行のみ
        if self.installed_packages.is_empty() {
            writeln!(f, "  No packages installed in this scope.")?;
        } else {
            for pkg in &self.installed_packages {
                writeln!(f, "{}", pkg)?; // 各パッケージ情報を表示
            }
        }
        Ok(())
    }
}

impl Display for InstalledPackageData {
    /// `InstalledPackageData` を整形して表示します。
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "  {}: {}",
            "Name".bold(),
            self.info.about.package.name.cyan()
        )?;
        writeln!(
            f,
            "    {}: {}",
            "Version".bold(),
            self.info.about.package.version
        )?;
        writeln!(
            f,
            "    {}: {} <{}>",
            "Author".bold(),
            self.info.about.author.name,
            self.info.about.author.email
        )?;
        writeln!(
            f,
            "    {}: {}",
            "Last Modified".bold(),
            self.last_modified.to_rfc3339()
        )?;
        writeln!(
            f,
            "    {}: {}",
            "Installation Type".bold(),
            if self.is_auto_installed {
                "Automatic".yellow()
            } else {
                "Manual".green()
            }
        )?;
        // RelationData も表示する場合
        if !self.info.relation.is_empty() {
            writeln!(f, "    {}", "Relations:".bold())?;
            // RelationData の Display 実装を再利用
            let mut indented_relations = String::new();
            // RelationData の Display 実装から出力される各行にインデントを追加
            for line in format!("{}", self.info.relation).lines() {
                indented_relations.push_str(&format!("      {}\n", line));
            }
            write!(f, "{}", indented_relations)?;
        }
        Ok(())
    }
}

/// インストールされているパッケージのリストを表示します。
///
/// この関数は、`--local` または `--global` オプションに基づいて、
/// ローカルまたはグローバルなパッケージリストを読み込み、表示します。
/// デフォルトでは、現在のユーザーがスーパーユーザーでない限りローカルリストを表示します。
///
/// # 引数
/// * `args` - コマンドライン引数のリスト。
///
/// # 戻り値
/// リスト表示が成功した場合は `Ok(())` を、エラーが発生した場合は `std::io::Error` を返します。
pub fn list(args: Vec<&Option>) -> Result<(), std::io::Error> {
    // デフォルトのリストターゲットは、現在のユーザーがスーパーユーザーでない場合ローカル
    let mut list_local = !shell::is_superuser();

    // 引数を解析してリストターゲットを決定
    for arg in args {
        match arg.opt_str.as_str() {
            "--local" | "-l" => list_local = true,
            "--global" | "-g" => list_local = false,
            _ => {
                // 不明なオプションの場合、エラーを返すか、無視するかはアプリケーションのポリシーによる
                eprintln!("Warning: Unknown option '{}'. Ignoring.", arg.opt_str);
                // エラーを返す場合は以下のように変更
                // return Err(std::io::Error::new(
                //     std::io::ErrorKind::InvalidInput,
                //     format!("Unknown option: {}", arg.opt_str)
                // ));
            }
        }
    }

    let target_filepath = if list_local {
        path::local::packageslist_filepath()
    } else {
        path::global::packageslist_filepath()
    };

    // パッケージリストの読み込みと表示
    let packages_list_data = PackageListData::from_filepath(&target_filepath).or_else(|e| {
        // ファイルが存在しない場合（NotFound）は、空のリストとして扱う
        if e.kind() == io::ErrorKind::NotFound {
            println!(
                "No packages list found at {}. Assuming empty list.",
                target_filepath.display()
            );
            Ok(PackageListData::default())
        } else {
            // その他のエラーはそのまま伝播
            Err(e)
        }
    })?;

    println!("{}", packages_list_data);
    Ok(())
}
