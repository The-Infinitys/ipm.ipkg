use super::version::{Version, VersionRange};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
pub mod depend;
pub mod list;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PackageData {
    pub about: AboutData,
    #[serde(skip_serializing_if = "RelationData::is_empty")]
    pub relation: RelationData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct AboutData {
    pub author: AuthorAboutData,
    pub package: PackageAboutData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AuthorAboutData {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PackageAboutData {
    pub name: String,
    pub version: Version,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct RelationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub depend: Vec<Vec<DependPackageData>>, // 依存関係のグループ（代替は内側のVecで表現）
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub suggests: Vec<Vec<DependPackageData>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recommends: Vec<Vec<DependPackageData>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub conflicts: Vec<DependPackageData>, // 競合パッケージのリスト
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub virtuals: Vec<DependPackageData>,
    pub cmds: Vec<String>, // 依存コマンドのリスト
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
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

        // 依存関係の表示 (既存部分)
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

        if !self.relation.cmds.is_empty() {
            writeln!(f, "\n{}", "Necessary Commands:".bold())?;
            for cmd in &self.relation.cmds {
                writeln!(f, "  - {}", cmd.green(),)?;
            }
        }
        // サジェストの表示 (追加部分)
        if !self.relation.suggests.is_empty() {
            writeln!(f, "\n{}", "Suggests:".bold())?;
            for group in &self.relation.suggests {
                if group.len() == 1 {
                    let dep = &group[0];
                    writeln!(f, "  - {} ({})", dep.name.yellow(), dep.version)?;
                } else {
                    let alts: Vec<String> = group
                        .iter()
                        .map(|d| format!("{} ({})", d.name, d.version))
                        .collect();
                    let alts_str = alts.join(" | ");
                    writeln!(f, "  - ({})", alts_str.yellow())?;
                }
            }
        }

        // 推奨の表示 (追加部分)
        if !self.relation.recommends.is_empty() {
            writeln!(f, "\n{}", "Recommends:".bold())?;
            for group in &self.relation.recommends {
                if group.len() == 1 {
                    let dep = &group[0];
                    writeln!(f, "  - {} ({})", dep.name.blue(), dep.version)?;
                } else {
                    let alts: Vec<String> = group
                        .iter()
                        .map(|d| format!("{} ({})", d.name, d.version))
                        .collect();
                    let alts_str = alts.join(" | ");
                    writeln!(f, "  - ({})", alts_str.blue())?;
                }
            }
        }
        if !self.relation.conflicts.is_empty() {
            writeln!(f, "\n{}", "Conflicts:".bold())?;
            for conflict in &self.relation.conflicts {
                writeln!(f, "  - {} ({})", conflict.name.red(), conflict.version)?;
            }
        }

        if !self.relation.virtuals.is_empty() {
            writeln!(f, "\n{}", "Virtual Packages:".bold())?;
            for virtual_pkg in &self.relation.virtuals {
                writeln!(
                    f,
                    "  - {} ({})",
                    virtual_pkg.name.purple(),
                    virtual_pkg.version
                )?;
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
                suggests: Vec::new(),
                recommends: Vec::new(),
                conflicts: Vec::new(),
                virtuals: Vec::new(),
                cmds: Vec::new(),
            },
        }
    }
}

impl RelationData {
    /// 全ての関係フィールドが空かどうかを判定します
    fn is_empty(&self) -> bool {
        self.depend.is_empty()
            && self.suggests.is_empty()
            && self.recommends.is_empty()
            && self.conflicts.is_empty()
            && self.cmds.is_empty()
    }
}

impl Default for AuthorAboutData {
    fn default() -> Self {
        AuthorAboutData {
            name: "default".to_string(),
            email: "default@default.com".to_string(),
        }
    }
}

impl Default for PackageAboutData {
    fn default() -> Self {
        PackageAboutData {
            name: "default-package".to_string(),
            version: Version::default(),
        }
    }
}

impl Default for DependPackageData {
    fn default() -> Self {
        DependPackageData {
            name: "default-dependency".to_string(),
            version: VersionRange::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_display_default() {
        let data = PackageData::default();
        println!("{}", data);
    }

    #[test]
    fn test_display_with_relations() {
        let mut data = PackageData::default();
        data.about.author = AuthorAboutData {
            name: "Test Author".to_string(),
            email: "test@example.com".to_string(),
        };
        data.about.package = PackageAboutData {
            name: "my-package".to_string(),
            version: Version::default(),
        };

        // Add dependencies
        data.relation.depend.push(vec![DependPackageData {
            name: "dep-a".to_string(),
            version: VersionRange::from_str(">= 1.0, < 2.0").unwrap(),
        }]);
        data.relation.depend.push(vec![
            DependPackageData {
                name: "dep-b".to_string(),
                version: VersionRange::from_str("= 2.0.0").unwrap(),
            },
            DependPackageData {
                name: "dep-c".to_string(),
                version: VersionRange::from_str("> 1.5.0").unwrap(),
            },
        ]);

        // Add suggests
        data.relation.suggests.push(vec![DependPackageData {
            name: "suggest-x".to_string(),
            version: VersionRange::from_str(" = 3.0").unwrap(),
        }]);

        // Add recommends
        data.relation.recommends.push(vec![
            DependPackageData {
                name: "rec-y".to_string(),
                version: VersionRange::from_str("< 4.0.0").unwrap(),
            },
            DependPackageData {
                name: "rec-z".to_string(),
                version: VersionRange::from_str("= 4.1.0").unwrap(),
            },
        ]);

        // Add conflicts
        data.relation.conflicts.push(DependPackageData {
            name: "old-package".to_string(),
            version: VersionRange::from_str("0.9.0").unwrap(),
        });

        println!("{}", data);
    }
}
