use super::PackageData;
use chrono::{DateTime, Local};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

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
