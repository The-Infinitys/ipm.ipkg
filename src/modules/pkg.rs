use super::version::{Version, VersionRange};
use crate::utils::{generate_email_address, shell::username};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub mod depend;
mod install;
pub mod list;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
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
    pub depend: Vec<Vec<PackageRange>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub depend_cmds: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub suggests: Vec<Vec<PackageRange>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recommends: Vec<Vec<PackageRange>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub conflicts: Vec<PackageRange>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub virtuals: Vec<PackageVersion>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub provide_cmds: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PackageRange {
    pub name: String,
    pub range: VersionRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PackageVersion {
    pub name: String,
    pub version: Version,
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
                    let dep = &group[0];
                    writeln!(f, "  - {} ({})", dep.name.green(), dep.range)?;
                } else {
                    // alts_str の生成を維持し、可読性を優先
                    let alts: Vec<String> = group
                        .iter()
                        .map(|d| format!("{} ({})", d.name, d.range))
                        .collect();
                    let alts_str = alts.join(" | ");
                    writeln!(f, "  - ({})", alts_str.green())?;
                }
            }
        }

        if !self.relation.depend_cmds.is_empty() {
            writeln!(f, "\n{}", "Necessary Commands:".bold())?;
            for cmd in &self.relation.depend_cmds {
                writeln!(f, "  - {}", cmd.green())?;
            }
        }

        if !self.relation.suggests.is_empty() {
            writeln!(f, "\n{}", "Suggests:".bold())?;
            for group in &self.relation.suggests {
                if group.len() == 1 {
                    let dep = &group[0];
                    writeln!(f, "  - {} ({})", dep.name.yellow(), dep.range)?;
                } else {
                    let alts: Vec<String> = group
                        .iter()
                        .map(|d| format!("{} ({})", d.name, d.range))
                        .collect();
                    let alts_str = alts.join(" | ");
                    writeln!(f, "  - ({})", alts_str.yellow())?;
                }
            }
        }

        if !self.relation.recommends.is_empty() {
            writeln!(f, "\n{}", "Recommends:".bold())?;
            for group in &self.relation.recommends {
                if group.len() == 1 {
                    let dep = &group[0];
                    writeln!(f, "  - {} ({})", dep.name.blue(), dep.range)?;
                } else {
                    let alts: Vec<String> = group
                        .iter()
                        .map(|d| format!("{} ({})", d.name, d.range))
                        .collect();
                    let alts_str = alts.join(" | ");
                    writeln!(f, "  - ({})", alts_str.blue())?;
                }
            }
        }

        if !self.relation.conflicts.is_empty() {
            writeln!(f, "\n{}", "Conflicts:".bold())?;
            for conflict in &self.relation.conflicts {
                writeln!(f, "  - {} ({})", conflict.name.red(), conflict.range)?;
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

        if !self.relation.provide_cmds.is_empty() {
            writeln!(f, "\n{}", "Providing Commands:".bold())?;
            for cmd in &self.relation.provide_cmds {
                writeln!(f, "  - {}", cmd.green())?;
            }
        }
        Ok(())
    }
}

impl Display for AboutData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", "Author:".bold(), self.author)?;
        writeln!(f, "{} {}", "Package:".bold(), self.package)?;
        Ok(())
    }
}

impl Display for AuthorAboutData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)?;
        Ok(())
    }
}

impl Display for PackageAboutData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name.cyan(), self.version)?;
        Ok(())
    }
}

impl Display for RelationData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.depend.is_empty() {
            writeln!(f, "{}", "Dependencies:".bold())?;
            for group in &self.depend {
                if group.len() == 1 {
                    // `to_string()` の呼び出しを減らし、`Display` 実装を利用
                    writeln!(f, "  - {}", &group[0])?;
                } else {
                    let alts: Vec<String> = group.iter().map(|d| d.to_string()).collect();
                    let alts_str = alts.join(" | ");
                    writeln!(f, "  - ({})", alts_str.green())?;
                }
            }
        }

        if !self.depend_cmds.is_empty() {
            writeln!(f, "\n{}", "Necessary Commands:".bold())?;
            for cmd in &self.depend_cmds {
                writeln!(f, "  - {}", cmd.green())?;
            }
        }

        if !self.suggests.is_empty() {
            writeln!(f, "\n{}", "Suggests:".bold())?;
            for group in &self.suggests {
                if group.len() == 1 {
                    writeln!(f, "  - {}", group[0].to_string().yellow())?; // `to_string()` は `colored` のために必要
                } else {
                    let alts: Vec<String> = group.iter().map(|d| d.to_string()).collect();
                    let alts_str = alts.join(" | ");
                    writeln!(f, "  - ({})", alts_str.yellow())?;
                }
            }
        }

        if !self.recommends.is_empty() {
            writeln!(f, "\n{}", "Recommends:".bold())?;
            for group in &self.recommends {
                if group.len() == 1 {
                    writeln!(f, "  - {}", group[0].to_string().blue())?; // `to_string()` は `colored` のために必要
                } else {
                    let alts: Vec<String> = group.iter().map(|d| d.to_string()).collect();
                    let alts_str = alts.join(" | ");
                    writeln!(f, "  - ({})", alts_str.blue())?;
                }
            }
        }

        if !self.conflicts.is_empty() {
            writeln!(f, "\n{}", "Conflicts:".bold())?;
            for conflict in &self.conflicts {
                writeln!(f, "  - {}", conflict.to_string().red())?; // `to_string()` は `colored` のために必要
            }
        }

        if !self.virtuals.is_empty() {
            writeln!(f, "\n{}", "Virtual Packages:".bold())?;
            for virtual_pkg in &self.virtuals {
                writeln!(f, "  - {}", virtual_pkg.to_string().purple())?; // `to_string()` は `colored` のために必要
            }
        }

        if !self.provide_cmds.is_empty() {
            writeln!(f, "\n{}", "Providing Commands:".bold())?;
            for cmd in &self.provide_cmds {
                writeln!(f, "  - {}", cmd.green())?;
            }
        }
        Ok(())
    }
}

impl Display for PackageRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.range)?;
        Ok(())
    }
}

impl Display for PackageVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.version)?;
        Ok(())
    }
}

impl Default for AuthorAboutData {
    fn default() -> Self {
        AuthorAboutData {
            name: username(),
            email: generate_email_address(),
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

impl Default for PackageRange {
    fn default() -> Self {
        PackageRange {
            name: "default-dependency".to_string(),
            range: VersionRange::default(),
        }
    }
}

impl Default for PackageVersion {
    fn default() -> Self {
        PackageVersion {
            name: "default-version".to_string(),
            version: Version::default(),
        }
    }
}

impl RelationData {
    /// RelationData が空かどうかを判定します。
    /// Serde の `skip_serializing_if` に使用されます。
    fn is_empty(&self) -> bool {
        self.depend.is_empty()
            && self.depend_cmds.is_empty()
            && self.suggests.is_empty()
            && self.recommends.is_empty()
            && self.conflicts.is_empty()
            && self.virtuals.is_empty()
            && self.provide_cmds.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

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

        data.relation.depend.push(vec![PackageRange {
            name: "dep-a".to_string(),
            range: VersionRange::from_str(">= 1.0, < 2.0").unwrap(),
        }]);
        data.relation.depend.push(vec![
            PackageRange {
                name: "dep-b".to_string(),
                range: VersionRange::from_str("= 2.0.0").unwrap(),
            },
            PackageRange {
                name: "dep-c".to_string(),
                range: VersionRange::from_str("> 1.5.0").unwrap(),
            },
        ]);

        data.relation.suggests.push(vec![PackageRange {
            name: "suggest-x".to_string(),
            range: VersionRange::from_str("= 3.0").unwrap(),
        }]);

        data.relation.recommends.push(vec![
            PackageRange {
                name: "rec-y".to_string(),
                range: VersionRange::from_str("< 4.0.0").unwrap(),
            },
            PackageRange {
                name: "rec-z".to_string(),
                range: VersionRange::from_str("= 4.1.0").unwrap(),
            },
        ]);

        data.relation.conflicts.push(PackageRange {
            name: "old-package".to_string(),
            range: VersionRange::from_str("0.9.0").unwrap(),
        });

        data.relation.virtuals.push(PackageVersion {
            name: "my-virtual-pkg".to_string(),
            version: Version::from_str("1.0.0").unwrap(),
        });

        data.relation.provide_cmds.push("my-command".to_string());
        data.relation
            .provide_cmds
            .push("another-command".to_string());
        data.relation.depend_cmds.push("git".to_string());
        data.relation.depend_cmds.push("make".to_string());

        println!("{}", data);
    }

    #[test]
    fn test_display_author() {
        let author = AuthorAboutData {
            name: "Test Author".to_string(),
            email: "test@example.com".to_string(),
        };
        println!("{}", author);
    }

    #[test]
    fn test_display_package() {
        let package = PackageAboutData {
            name: "test-package".to_string(),
            version: Version::default(),
        };
        println!("{}", package);
    }

    #[test]
    fn test_display_relation() {
        let mut relation = RelationData::default();
        relation.depend.push(vec![PackageRange {
            name: "dep-a".to_string(),
            range: VersionRange::from_str(">= 1.0").unwrap(),
        }]);
        relation.suggests.push(vec![PackageRange {
            name: "suggest-x".to_string(),
            range: VersionRange::from_str("= 3.0").unwrap(),
        }]);
        relation.conflicts.push(PackageRange {
            name: "conflicting-pkg".to_string(),
            range: VersionRange::from_str("< 1.0").unwrap(),
        });
        println!("{}", relation);
    }

    #[test]
    fn test_display_package_range() {
        let range = PackageRange {
            name: "test-dep".to_string(),
            range: VersionRange::from_str(">= 1.0").unwrap(),
        };
        println!("{}", range);
    }

    #[test]
    fn test_display_package_version() {
        let version = PackageVersion {
            name: "test-version".to_string(),
            version: Version::default(),
        };
        println!("{}", version);
    }
}
