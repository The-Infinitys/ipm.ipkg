use super::super::system::path;
use super::PackageData;
use crate::utils::shell;
use chrono::{DateTime, Local};
use cmd_arg::cmd_arg::Option;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::Read;
#[derive(Serialize, Deserialize)]
pub struct PackageListData {
    last_modified: DateTime<Local>,
    packages: InstalledPackageData,
}
impl Default for PackageListData {
    fn default() -> Self {
        Self {
            last_modified: Local::now(),
            packages: InstalledPackageData::default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct InstalledPackageData {
    info: PackageData,
    last_modified: DateTime<Local>,
    is_auto_installed: bool,
}
impl PackageListData {
    fn new(is_local: bool) -> PackageListData {
        let target_path = if is_local {
            path::local::packageslist_filepath()
        } else {
            path::global::packageslist_filepath()
        };
        let mut target_file = File::open(target_path).expect("Couldn't found packages list file.");
        let mut packageslist_str = String::new();

        target_file
            .read_to_string(&mut packageslist_str)
            .expect("Failed to read packages list file");
        let mut result_data: PackageListData = serde_yaml::from_str(&packageslist_str).unwrap();
        result_data.last_modified = Local::now();
        result_data
    }
}
impl Default for InstalledPackageData {
    fn default() -> Self {
        Self {
            info: PackageData::default(),
            last_modified: Local::now(),
            is_auto_installed: false,
        }
    }
}
impl Display for PackageListData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lines = [
            format!(
                "{}: {}",
                "Last Modified".green().bold(),
                self.last_modified.to_rfc3339()
            ),
            format!("{}: \n{}", "Packages".cyan().bold(), self.packages),
        ];
        for line in lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

impl Display for InstalledPackageData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lines = [
            format!(
                "{}: {}",
                "Last Modified".green().bold(),
                self.last_modified.to_rfc3339()
            ),
            if self.is_auto_installed {
                "Automatic Installed".to_owned()
            } else {
                "Manually Installed".to_owned()
            },
            format!("{}:\n{}", "INFO".green().bold(), self.info),
        ];
        for line in lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}
pub fn list(args: Vec<&Option>) {
    let mut list_target = !shell::is_superuser();
    for arg in args {
        match arg.opt_str.as_str() {
            "--local" | "-l" => list_target = true,
            "--global" | "-g" => list_target = false,
            _ => {}
        }
    }
    let packages_list_data = PackageListData::new(list_target);
    println!("{}", packages_list_data);
}
