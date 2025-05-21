use super::super::version::{Version, VersionRange};
use super::{DependPackageData, PackageData};
use crate::utils::shell::is_cmd_available;
use colored::Colorize;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct DependencyResolver {
    packages: HashMap<String, PackageData>,
}

#[derive(Debug)]
pub enum DependencyError {
    CircularDependency(String),
    VersionConflict(String, Box<VersionRange>, Version),
    MissingDependency(String),
    CommandNotFound(String),
}

impl fmt::Display for DependencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DependencyError::CircularDependency(pkg) => {
                write!(f, "{}: {}", "Circular dependency".red().bold(), pkg)
            }
            DependencyError::VersionConflict(pkg, range, version) => {
                write!(
                    f,
                    "{}: {} ({} vs {})",
                    "Version conflict".red().bold(),
                    pkg,
                    range,
                    version
                )
            }
            DependencyError::MissingDependency(pkg) => {
                write!(f, "{}: {}", "Missing dependency".red().bold(), pkg)
            }
            DependencyError::CommandNotFound(cmd) => {
                write!(f, "{}: {}", "Command not found".red().bold(), cmd)
            }
        }
    }
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl DependencyResolver {
    pub fn new() -> Self {
        DependencyResolver {
            packages: HashMap::new(),
        }
    }

    /// パッケージをリゾルバに追加
    pub fn add_package(&mut self, package: PackageData) {
        self.packages
            .insert(package.about.package.name.clone(), package);
    }

    /// 依存関係の解決を試みる
    pub fn resolve(&self, package_name: &str) -> Result<Vec<DependPackageData>, DependencyError> {
        let mut resolved = Vec::new();
        let mut visited = HashSet::new();

        self.resolve_recursive(package_name, &mut resolved, &mut visited)?;
        Ok(resolved)
    }

    /// 再帰的に依存関係を解決
    fn resolve_recursive(
        &self,
        package_name: &str,
        resolved: &mut Vec<DependPackageData>,
        visited: &mut HashSet<String>,
    ) -> Result<(), DependencyError> {
        if !visited.insert(package_name.to_string()) {
            return Err(DependencyError::CircularDependency(
                package_name.to_string(),
            ));
        }

        let package = self
            .packages
            .get(package_name)
            .ok_or_else(|| DependencyError::MissingDependency(package_name.to_string()))?;

        for dep_group in &package.relation.depend {
            let mut group_resolved = false;
            for dep in dep_group {
                if self.can_satisfy_dependency(dep, &self.packages) {
                    resolved.push(dep.clone());
                    self.resolve_recursive(&dep.name, resolved, visited)?;
                    group_resolved = true;
                    break;
                }
            }
            if !group_resolved {
                return Err(DependencyError::MissingDependency(
                    dep_group[0].name.clone(),
                ));
            }
        }

        visited.remove(package_name);
        Ok(())
    }

    /// 依存関係が満たされるかチェック
    fn can_satisfy_dependency(
        &self,
        dep: &DependPackageData,
        packages: &HashMap<String, PackageData>,
    ) -> bool {
        if let Some(pkg) = packages.get(&dep.name) {
            dep.version.compare(&pkg.about.package.version)
        } else {
            false
        }
    }

    /// 競合チェック
    pub fn check_conflicts(&self, package_name: &str) -> Result<(), DependencyError> {
        let package = self
            .packages
            .get(package_name)
            .ok_or_else(|| DependencyError::MissingDependency(package_name.to_string()))?;

        for conflict in &package.relation.conflicts {
            if let Some(existing_pkg) = self.packages.get(&conflict.name) {
                if conflict
                    .version
                    .compare(&existing_pkg.about.package.version)
                {
                    return Err(DependencyError::VersionConflict(
                        conflict.name.clone(),
                        Box::new(conflict.version.clone()),
                        existing_pkg.about.package.version.clone(),
                    ));
                }
            }
        }
        Ok(())
    }

    /// 推奨および提案パッケージの取得
    pub fn get_recommendations(&self, package_name: &str) -> Option<Vec<DependPackageData>> {
        let package = self.packages.get(package_name)?;
        let mut recommendations = Vec::new();

        for group in &package.relation.recommends {
            if let Some(dep) = group
                .iter()
                .find(|dep| self.can_satisfy_dependency(dep, &self.packages))
            {
                recommendations.push(dep.clone());
            }
        }

        for group in &package.relation.suggests {
            if let Some(dep) = group
                .iter()
                .find(|dep| self.can_satisfy_dependency(dep, &self.packages))
            {
                recommendations.push(dep.clone());
            }
        }

        Some(recommendations)
    }

    /// 依存関係ツリーをグラフィカルに表示
    pub fn display_dependency_tree(&self, package_name: &str) -> Result<(), DependencyError> {
        let mut visited = HashSet::new();
        println!("{}", "Dependency Tree:".bold().cyan());
        self.display_dependency_recursive(package_name, 0, &mut visited)?;
        Ok(())
    }

    /// 再帰的に依存関係ツリーを表示
    fn display_dependency_recursive(
        &self,
        package_name: &str,
        depth: usize,
        visited: &mut HashSet<String>,
    ) -> Result<(), DependencyError> {
        if !visited.insert(package_name.to_string()) {
            println!(
                "{}└── {} {}",
                "  ".repeat(depth * 2),
                "Circular:".red().bold(),
                package_name.red()
            );
            return Err(DependencyError::CircularDependency(
                package_name.to_string(),
            ));
        }

        let package = self
            .packages
            .get(package_name)
            .ok_or_else(|| DependencyError::MissingDependency(package_name.to_string()))?;

        let prefix = if depth == 0 {
            "".to_string()
        } else {
            format!("{}├── ", "  ".repeat(depth * 2))
        };

        println!(
            "{}{} ({})",
            prefix,
            package_name.green().bold(),
            package.about.package.version
        );

        for dep_group in &package.relation.depend {
            let group_str = dep_group
                .iter()
                .map(|dep| format!("{} ({})", dep.name, dep.version))
                .collect::<Vec<_>>()
                .join(" | ");
            println!(
                "{}└── {} ({})",
                "  ".repeat((depth + 1) * 2),
                "Depends:".blue(),
                group_str.green()
            );

            for dep in dep_group {
                if self.can_satisfy_dependency(dep, &self.packages) {
                    self.display_dependency_recursive(&dep.name, depth + 1, visited)?;
                    break;
                }
            }
        }

        // コマンドの存在確認と表示
        if !package.relation.cmds.is_empty() {
            println!(
                "{}└── {}",
                "  ".repeat((depth + 1) * 2),
                "Commands:".yellow().bold()
            );
            for cmd in &package.relation.cmds {
                let status = if is_cmd_available(cmd) {
                    "Available".green()
                } else {
                    "Not Found".red()
                };
                println!(
                    "{}    ├── {} [{}]",
                    "  ".repeat((depth + 1) * 2),
                    cmd.cyan(),
                    status
                );
            }
        }

        // 推奨パッケージ
        if !package.relation.recommends.is_empty() {
            println!(
                "{}└── {}",
                "  ".repeat((depth + 1) * 2),
                "Recommends:".blue().bold()
            );
            for group in &package.relation.recommends {
                let group_str = group
                    .iter()
                    .map(|dep| format!("{} ({})", dep.name, dep.version))
                    .collect::<Vec<_>>()
                    .join(" | ");
                println!(
                    "{}    ├── {}",
                    "  ".repeat((depth + 1) * 2),
                    group_str.blue()
                );
            }
        }

        // 提案パッケージ
        if !package.relation.suggests.is_empty() {
            println!(
                "{}└── {}",
                "  ".repeat((depth + 1) * 2),
                "Suggests:".yellow().bold()
            );
            for group in &package.relation.suggests {
                let group_str = group
                    .iter()
                    .map(|dep| format!("{} ({})", dep.name, dep.version))
                    .collect::<Vec<_>>()
                    .join(" | ");
                println!(
                    "{}    ├── {}",
                    "  ".repeat((depth + 1) * 2),
                    group_str.yellow()
                );
            }
        }

        // 競合パッケージ
        if !package.relation.conflicts.is_empty() {
            println!(
                "{}└── {}",
                "  ".repeat((depth + 1) * 2),
                "Conflicts:".red().bold()
            );
            for conflict in &package.relation.conflicts {
                let status = if self.can_satisfy_dependency(conflict, &self.packages) {
                    "Present".red()
                } else {
                    "Not Present".green()
                };
                println!(
                    "{}    ├── {} ({}) [{}]",
                    "  ".repeat((depth + 1) * 2),
                    conflict.name,
                    conflict.version,
                    status
                );
            }
        }

        visited.remove(package_name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::version::{Version, VersionRange};
    use super::super::{AboutData, DependPackageData, PackageAboutData, PackageData, RelationData};
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_display_dependency_tree() {
        let mut resolver = DependencyResolver::new();

        // パッケージA: B (>= 1.0) に依存、gcc コマンドを要求
        let pkg_a = PackageData {
            about: AboutData {
                package: PackageAboutData {
                    name: "pkg-a".to_string(),
                    version: Version::from_str("1.0.0").unwrap(),
                },
                ..Default::default()
            },
            relation: RelationData {
                depend: vec![vec![DependPackageData {
                    name: "pkg-b".to_string(),
                    version: VersionRange::from_str(">= 1.0").unwrap(),
                }]],
                recommends: vec![vec![DependPackageData {
                    name: "pkg-rec".to_string(),
                    version: VersionRange::from_str(">= 2.0").unwrap(),
                }]],
                suggests: vec![vec![DependPackageData {
                    name: "pkg-sug".to_string(),
                    version: VersionRange::from_str(">= 3.0").unwrap(),
                }]],
                conflicts: vec![DependPackageData {
                    name: "pkg-conf".to_string(),
                    version: VersionRange::from_str(">= 1.0").unwrap(),
                }],
                cmds: vec!["gcc".to_string()],
                ..Default::default()
            },
        };

        // パッケージB
        let pkg_b = PackageData {
            about: AboutData {
                package: PackageAboutData {
                    name: "pkg-b".to_string(),
                    version: Version::from_str("1.1.0").unwrap(),
                },
                ..Default::default()
            },
            ..Default::default()
        };

        resolver.add_package(pkg_a);
        resolver.add_package(pkg_b);

        resolver.display_dependency_tree("pkg-a").unwrap();
    }

    #[test]
    fn test_circular_dependency_display() {
        let mut resolver = DependencyResolver::new();

        let pkg_a = PackageData {
            about: AboutData {
                package: PackageAboutData {
                    name: "pkg-a".to_string(),
                    version: Version::from_str("1.0.0").unwrap(),
                },
                ..Default::default()
            },
            relation: RelationData {
                depend: vec![vec![DependPackageData {
                    name: "pkg-b".to_string(),
                    version: VersionRange::from_str(">= 1.0").unwrap(),
                }]],
                ..Default::default()
            },
        };

        let pkg_b = PackageData {
            about: AboutData {
                package: PackageAboutData {
                    name: "pkg-b".to_string(),
                    version: Version::from_str("1.1.0").unwrap(),
                },
                ..Default::default()
            },
            relation: RelationData {
                depend: vec![vec![DependPackageData {
                    name: "pkg-a".to_string(),
                    version: VersionRange::from_str(">= 1.0").unwrap(),
                }]],
                ..Default::default()
            },
        };

        resolver.add_package(pkg_a);
        resolver.add_package(pkg_b);

        let result = resolver.display_dependency_tree("pkg-a");
        assert!(matches!(
            result,
            Err(DependencyError::CircularDependency(_))
        ));
    }
}
