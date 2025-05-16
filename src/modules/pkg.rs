use colored::Colorize;
use std::fmt::Display;

use super::version::{Version, VersionRange};

pub struct PackageData {
    pub about: AboutData,
    pub relation: RelationData,
}

pub struct AboutData {
    pub author: AuthorAboutData,
    pub package: PackageAboutData,
}

pub struct AuthorAboutData {
    pub name: String,
    pub email: String,
}

pub struct PackageAboutData {
    pub name: String,
    pub version: Version,
}

pub struct RelationData {
    pub depend: Vec<Vec<DependPackageData>>, // 依存関係のグループ（代替は内側のVecで表現）
    pub conflict: Vec<DependPackageData>,    // 競合パッケージのリスト
}

pub struct DependPackageData {
    pub name: String,
    pub version: VersionRange,
}

impl Display for PackageData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {}",
            "Package:".bold(),
            self.about.package.name.cyan()
        )?;
        writeln!(f, "{} {}", "Version:".bold(), self.about.package.version)?;
        writeln!(
            f,
            "{} {} <{}>",
            "Author:".bold(),
            self.about.author.name,
            self.about.author.email
        )?;

        if !self.relation.depend.is_empty() {
            writeln!(f, "\n{}", "Dependencies:".bold())?;
            for group in &self.relation.depend {
                if group.len() == 1 {
                    // 単一の依存関係
                    let dep = &group[0];
                    writeln!(f, "  - {} ({})", dep.name.green(), dep.version)?;
                } else {
                    // 代替依存のグループ
                    let alts: Vec<String> = group
                        .iter()
                        .map(|d| format!("{} ({})", d.name, d.version))
                        .collect();
                    let alts_str = alts.join(" | ");
                    writeln!(f, "  - ({})", alts_str.green())?;
                }
            }
        }

        if !self.relation.conflict.is_empty() {
            writeln!(f, "\n{}", "Conflicts:".bold())?;
            for conflict in &self.relation.conflict {
                writeln!(f, "  - {} ({})", conflict.name.red(), conflict.version)?;
            }
        }

        Ok(())
    }
}

impl Default for PackageData {
    fn default() -> Self {
        PackageData {
            about: AboutData {
                author: AuthorAboutData {
                    name: "default".to_string(),
                    email: "default@default.com".to_string(),
                },
                package: PackageAboutData {
                    name: "default-package".to_string(),
                    version: Version::default(),
                },
            },
            relation: RelationData {
                depend: Vec::new(),
                conflict: Vec::new(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let data = PackageData::default();
        println!("{}", data);
    }
}
