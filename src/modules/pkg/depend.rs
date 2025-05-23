use super::list::PackageListData;
use super::super::version::Version;
use super::{PackageData, PackageRange, RelationData};
use crate::utils::shell;
use std::collections::HashMap;

/// インストール済みのパッケージから、実パッケージと利用可能なパッケージ（実パッケージと仮想パッケージ）のマップを構築します。
fn build_package_maps(installed_packages: &PackageListData) -> (HashMap<String, Vec<Version>>, HashMap<String, Vec<Version>>) {
    let mut real_packages = HashMap::new();
    let mut available_packages = HashMap::new();
    for p in &installed_packages.installed_packages {
        let name = p.info.about.package.name.clone();
        let version = p.info.about.package.version.clone();
        real_packages
            .entry(name.clone())
            .or_insert_with(Vec::new)
            .push(version.clone());
        available_packages
            .entry(name)
            .or_insert_with(Vec::new)
            .push(version);
        for v in &p.info.relation.virtuals {
            let v_name = v.name.clone();
            let v_version = v.version.clone();
            available_packages
                .entry(v_name)
                .or_insert_with(Vec::new)
                .push(v_version);
        }
    }
    (real_packages, available_packages)
}

/// 単一の依存関係がインストール済みのパッケージまたは仮想パッケージで満たされているかをチェックします。
pub fn is_dependency_satisfied(dep: &PackageRange, installed_packages: &PackageListData) -> bool {
    let (_, available_packages) = build_package_maps(installed_packages);
    if let Some(versions) = available_packages.get(&dep.name) {
        versions.iter().any(|v| dep.range.compare(v))
    } else {
        false
    }
}

/// パッケージのすべての依存関係が満たされているかをチェックします。
pub fn are_dependencies_satisfied(
    package: &PackageData,
    installed_packages: &PackageListData,
) -> bool {
    let (_, available_packages) = build_package_maps(installed_packages);
    package.relation.depend.iter().all(|group| {
        group.iter().any(|dep| {
            if let Some(versions) = available_packages.get(&dep.name) {
                versions.iter().any(|v| dep.range.compare(v))
            } else {
                false
            }
        })
    })
}

/// 満たされていない依存関係グループを返します。
pub fn get_missing_dependencies(
    package: &PackageData,
    installed_packages: &PackageListData,
) -> Vec<Vec<PackageRange>> {
    let (_, available_packages) = build_package_maps(installed_packages);
    package
        .relation
        .depend
        .iter()
        .filter(|group| {
            !group.iter().any(|dep| {
                if let Some(versions) = available_packages.get(&dep.name) {
                    versions.iter().any(|v| dep.range.compare(v))
                } else {
                    false
                }
            })
        })
        .cloned()
        .collect()
}

/// パッケージが必要とするすべてのシステムコマンドが利用可能かをチェックします。
pub fn are_depend_cmds_available(relation: &RelationData) -> bool {
    relation
        .depend_cmds
        .iter()
        .all(|cmd| shell::is_cmd_available(cmd))
}

/// 利用できない必須コマンドのリストを返します。
pub fn get_missing_depend_cmds(relation: &RelationData) -> Vec<String> {
    relation
        .depend_cmds
        .iter()
        .filter(|cmd| !shell::is_cmd_available(cmd))
        .cloned()
        .collect()
}

/// パッケージがインストール済みのパッケージと競合するかをチェックします。
pub fn has_conflicts(package: &PackageData, installed_packages: &PackageListData) -> bool {
    let (real_packages, _) = build_package_maps(installed_packages);
    package.relation.conflicts.iter().any(|conflict| {
        if let Some(versions) = real_packages.get(&conflict.name) {
            versions.iter().any(|v| conflict.range.compare(v))
        } else {
            false
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::list::{InstalledPackageData, PackageListData};
    use super::super::{
        AboutData, PackageAboutData, PackageData, PackageRange, PackageVersion, RelationData,
    };
    use crate::modules::version::{Version, VersionRange};
    use std::str::FromStr;

    #[test]
    fn test_are_dependencies_satisfied() {
        let mut package = PackageData::default();
        package.relation.depend = vec![vec![PackageRange {
            name: "dep1".to_string(),
            range: VersionRange::from_str(">=1.0").unwrap(),
        }]];

        let mut installed_packages = PackageListData::default();
        installed_packages.installed_packages = vec![InstalledPackageData {
            info: PackageData {
                about: AboutData {
                    package: PackageAboutData {
                        name: "dep1".to_string(),
                        version: Version::from_str("1.2").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }];

        assert!(are_dependencies_satisfied(&package, &installed_packages));

        // 依存関係が欠けている場合
        let mut package2 = package.clone();
        package2.relation.depend[0][0].name = "dep2".to_string();
        assert!(!are_dependencies_satisfied(&package2, &installed_packages));

        // 仮想パッケージのテスト
        let mut package3 = PackageData::default();
        package3.relation.depend = vec![vec![PackageRange {
            name: "virtual-pkg".to_string(),
            range: VersionRange::from_str(">=1.0").unwrap(),
        }]];

        let mut installed_packages2 = PackageListData::default();
        installed_packages2.installed_packages = vec![InstalledPackageData {
            info: PackageData {
                about: AboutData {
                    package: PackageAboutData {
                        name: "provider".to_string(),
                        version: Version::from_str("2.0").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                relation: RelationData {
                    virtuals: vec![PackageVersion {
                        name: "virtual-pkg".to_string(),
                        version: Version::from_str("1.5").unwrap(),
                    }],
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }];

        assert!(are_dependencies_satisfied(&package3, &installed_packages2));
    }

    #[test]
    fn test_get_missing_dependencies() {
        let mut package = PackageData::default();
        package.relation.depend = vec![
            vec![PackageRange {
                name: "dep1".to_string(),
                range: VersionRange::from_str(">=1.0").unwrap(),
            }],
            vec![PackageRange {
                name: "dep2".to_string(),
                range: VersionRange::from_str(">=2.0").unwrap(),
            }],
        ];

        let installed_packages = PackageListData::default();
        let missing = get_missing_dependencies(&package, &installed_packages);
        assert_eq!(missing.len(), 2);
        assert_eq!(missing[0][0].name, "dep1");
        assert_eq!(missing[1][0].name, "dep2");
    }

    #[test]
    fn test_has_conflicts() {
        let mut package = PackageData::default();
        package.relation.conflicts = vec![PackageRange {
            name: "conflict1".to_string(),
            range: VersionRange::from_str(">=1.0").unwrap(),
        }];

        let mut installed_packages = PackageListData::default();
        installed_packages.installed_packages = vec![InstalledPackageData {
            info: PackageData {
                about: AboutData {
                    package: PackageAboutData {
                        name: "conflict1".to_string(),
                        version: Version::from_str("1.2").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }];

        assert!(has_conflicts(&package, &installed_packages));

        let mut package2 = package.clone();
        package2.relation.conflicts[0].name = "conflict2".to_string();
        assert!(!has_conflicts(&package2, &installed_packages));
    }
}