use colored::Colorize;
use std::fmt::Display;

// 仮の定義。実際のコードではsuper::versionからインポートされます。
#[derive(Debug, Default, PartialEq)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct VersionRange {
    // バージョン範囲を表すフィールド（例: >, >=, <, <=, = など）
    op: String,
    version: Version,
}

impl Display for VersionRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.op, self.version)
    }
}


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
    pub suggests: Vec<Vec<DependPackageData>>,
    pub recommends: Vec<Vec<DependPackageData>>,
    pub conflict: Vec<DependPackageData>, // 競合パッケージのリスト
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


        // 競合パッケージの表示 (既存部分)
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
                suggests: Vec::new(),
                recommends: Vec::new(),
                conflict: Vec::new(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
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
            version: Version { major: 1, minor: 2, patch: 0 },
        };

        // Add dependencies
        data.relation.depend.push(vec![DependPackageData {
            name: "dep-a".to_string(),
            version: VersionRange { op: ">=".to_string(), version: Version { major: 1, minor: 0, patch: 0 } },
        }]);
        data.relation.depend.push(vec![
            DependPackageData {
                name: "dep-b".to_string(),
                version: VersionRange { op: "".to_string(), version: Version { major: 2, minor: 0, patch: 0 } },
            },
            DependPackageData {
                name: "dep-c".to_string(),
                version: VersionRange { op: ">".to_string(), version: Version { major: 1, minor: 5, patch: 0 } },
            },
        ]);

        // Add suggests
        data.relation.suggests.push(vec![DependPackageData {
            name: "suggest-x".to_string(),
            version: VersionRange { op: "".to_string(), version: Version { major: 3, minor: 0, patch: 0 } },
        }]);

        // Add recommends
        data.relation.recommends.push(vec![
             DependPackageData {
                name: "rec-y".to_string(),
                version: VersionRange { op: "<".to_string(), version: Version { major: 4, minor: 0, patch: 0 } },
            },
             DependPackageData {
                name: "rec-z".to_string(),
                version: VersionRange { op: "".to_string(), version: Version { major: 4, minor: 1, patch: 0 } },
            },
        ]);


        // Add conflicts
        data.relation.conflict.push(DependPackageData {
            name: "old-package".to_string(),
            version: VersionRange { op: "<=".to_string(), version: Version { major: 0, minor: 9, patch: 0 } },
        });

        println!("{}", data);
    }
}