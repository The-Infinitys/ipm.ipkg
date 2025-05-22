use super::list::PackageListData;
use super::{PackageData, PackageRange, RelationData};
use crate::utils::shell;

/// 単一の依存関係がインストール済みのパッケージまたは仮想パッケージで満たされているかをチェックします。
///
/// # 引数
/// * `dep` - チェックする依存関係（パッケージ名とバージョン範囲）。
/// * `installed_packages` - インストール済みのパッケージリスト。
///
/// # 戻り値
/// 依存関係が満たされている場合は`true`、そうでない場合は`false`。
fn is_dependency_satisfied(dep: &PackageRange, installed_packages: &PackageListData) -> bool {
    // 具体的なパッケージのチェック
    if installed_packages.installed_packages.iter().any(|p| {
        p.info.about.package.name == dep.name && dep.range.compare(&p.info.about.package.version)
    }) {
        return true;
    }
    // 仮想パッケージのチェック
    installed_packages.installed_packages.iter().any(|p| {
        p.info
            .relation
            .virtuals
            .iter()
            .any(|v| v.name == dep.name && dep.range.compare(&v.version))
    })
}

/// パッケージのすべての依存関係が満たされているかをチェックします。
///
/// 各依存関係グループ内で、少なくとも1つの依存関係が満たされていれば、そのグループは満たされたと見なされます。
///
/// # 引数
/// * `package` - 依存関係をチェックするパッケージ。
/// * `installed_packages` - インストール済みのパッケージリスト。
///
/// # 戻り値
/// すべての依存関係グループが満たされている場合は`true`、そうでない場合は`false`。
pub fn are_dependencies_satisfied(
    package: &PackageData,
    installed_packages: &PackageListData,
) -> bool {
    package.relation.depend.iter().all(|group| {
        group
            .iter()
            .any(|dep| is_dependency_satisfied(dep, installed_packages))
    })
}

/// 満たされていない依存関係グループを返します。
///
/// # 引数
/// * `package` - 依存関係をチェックするパッケージ。
/// * `installed_packages` - インストール済みのパッケージリスト。
///
/// # 戻り値
/// 満たされていない依存関係グループのベクター。
pub fn get_missing_dependencies(
    package: &PackageData,
    installed_packages: &PackageListData,
) -> Vec<Vec<PackageRange>> {
    package
        .relation
        .depend
        .iter()
        .filter(|group| {
            !group
                .iter()
                .any(|dep| is_dependency_satisfied(dep, installed_packages))
        })
        .cloned()
        .collect()
}

/// パッケージが必要とするすべてのシステムコマンドが利用可能かをチェックします。
///
/// # 引数
/// * `relation` - チェックするパッケージの関係データ。
///
/// # 戻り値
/// すべての必須コマンドが利用可能な場合は`true`、そうでない場合は`false`。
pub fn are_depend_cmds_available(relation: &RelationData) -> bool {
    relation
        .depend_cmds
        .iter()
        .all(|cmd| shell::is_cmd_available(cmd))
}

/// 利用できない必須コマンドのリストを返します。
///
/// # 引数
/// * `relation` - チェックするパッケージの関係データ。
///
/// # 戻り値
/// 利用できないコマンドのベクター。
pub fn get_missing_depend_cmds(relation: &RelationData) -> Vec<String> {
    relation
        .depend_cmds
        .iter()
        .filter(|cmd| !shell::is_cmd_available(cmd))
        .cloned()
        .collect()
}

/// パッケージがインストール済みのパッケージと競合するかをチェックします。
///
/// # 引数
/// * `package` - 競合をチェックするパッケージ。
/// * `installed_packages` - インストール済みのパッケージリスト。
///
/// # 戻り値
/// 競合が存在する場合は`true`、そうでない場合は`false`。
pub fn has_conflicts(package: &PackageData, installed_packages: &PackageListData) -> bool {
    package.relation.conflicts.iter().any(|conflict| {
        installed_packages.installed_packages.iter().any(|p| {
            p.info.about.package.name == conflict.name
                && conflict.range.compare(&p.info.about.package.version)
        })
    })
}

#[cfg(test)]
mod tests {
    use super::super::list::{InstalledPackageData, PackageListData};
    use super::super::{
        AboutData, PackageAboutData, PackageData, PackageRange, PackageVersion, RelationData,
    };
    use super::*;
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
