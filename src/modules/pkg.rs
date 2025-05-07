use std::fmt;
use super::version::{Version, VersionRange};

// 構造体の定義
pub struct PackageInfo {
    about: AboutInfo,
    install_relation: RelationInfo,
    build_relation: BuildRelationInfo,
    virt_pkg: Vec<PkgVersion>,
}

pub struct AboutInfo {
    name: String,
    id: String,
    version: Version,
    description: String,
    author: AuthorInfo,
    size: u64,
}

pub struct AuthorInfo {
    name: String,
    id: String,
    email: Option<String>,
    website: Option<String>,
}

pub struct RelationInfo {
    depends: Vec<Vec<PkgRange>>,
    pre_depends: Vec<Vec<PkgRange>>,
    recommends: Vec<Vec<PkgRange>>,
    suggests: Vec<Vec<PkgRange>>,
    extension: Vec<PkgVersion>,
    breaks: Vec<PkgRange>,
    conflicts: Vec<PkgRange>,
    replaces: Vec<PkgRange>,
}

pub struct BuildRelationInfo {
    depends: Vec<Vec<PkgRange>>,
}

pub struct PkgVersion {
    name: String,
    version: Version,
}

pub struct PkgRange {
    name: String,
    range: VersionRange,
}

// Defaultトレイトの実装
impl Default for AuthorInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            id: String::new(),
            email: None,
            website: None,
        }
    }
}

impl Default for PkgRange {
    fn default() -> Self {
        Self {
            name: String::new(),
            range: VersionRange::default(),
        }
    }
}

impl Default for PkgVersion {
    fn default() -> Self {
        Self {
            name: String::new(),
            version: Version::default(),
        }
    }
}

impl Default for RelationInfo {
    fn default() -> Self {
        Self {
            depends: Vec::new(),
            pre_depends: Vec::new(),
            recommends: Vec::new(),
            suggests: Vec::new(),
            extension: Vec::new(),
            breaks: Vec::new(),
            conflicts: Vec::new(),
            replaces: Vec::new(),
        }
    }
}

impl Default for BuildRelationInfo {
    fn default() -> Self {
        Self {
            depends: Vec::new(),
        }
    }
}

impl Default for AboutInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            id: String::new(),
            version: Version::default(),
            description: String::new(),
            author: AuthorInfo::default(),
            size: 0,
        }
    }
}

impl Default for PackageInfo {
    fn default() -> Self {
        Self {
            about: AboutInfo::default(),
            install_relation: RelationInfo::default(),
            build_relation: BuildRelationInfo::default(),
            virt_pkg: Vec::new(),
        }
    }
}

// PackageInfoのnewメソッド
impl PackageInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

// Displayトレイトの実装
impl fmt::Display for PkgVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.version)
    }
}

impl fmt::Display for PkgRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.range)
    }
}

impl fmt::Display for PackageInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // AboutInfoの表示
        writeln!(f, "Package: {} ({})", self.about.name, self.about.id)?;
        writeln!(f, "Version: {}", self.about.version)?;
        writeln!(f, "Description: {}", self.about.description)?;

        // AuthorInfoの表示
        let mut author = format!("{} ({})", self.about.author.name, self.about.author.id);
        if let Some(email) = &self.about.author.email {
            author.push_str(&format!(" <{}>", email));
        }
        if let Some(website) = &self.about.author.website {
            author.push_str(&format!(" {}", website));
        }
        writeln!(f, "Author: {}", author)?;
        writeln!(f, "Size: {}", self.about.size)?;

        // Install Relationsの表示
        writeln!(f, "Install Relations:")?;
        let format_dep = |dep: &Vec<Vec<PkgRange>>| -> String {
            dep.iter()
                .map(|group| {
                    let group_str = group
                        .iter()
                        .map(|pkg| format!("{}", pkg))
                        .collect::<Vec<_>>()
                        .join(" | ");
                    if group.len() > 1 {
                        format!("({})", group_str)
                    } else {
                        group_str
                    }
                })
                .collect::<Vec<_>>()
                .join(", ")
        };
        writeln!(
            f,
            "  Depends: {}",
            format_dep(&self.install_relation.depends)
        )?;
        writeln!(
            f,
            "  Pre-Depends: {}",
            format_dep(&self.install_relation.pre_depends)
        )?;
        writeln!(
            f,
            "  Recommends: {}",
            format_dep(&self.install_relation.recommends)
        )?;
        writeln!(
            f,
            "  Suggests: {}",
            format_dep(&self.install_relation.suggests)
        )?;
        writeln!(
            f,
            "  Extension: {}",
            self.install_relation
                .extension
                .iter()
                .map(|pkg| format!("{}", pkg))
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        writeln!(
            f,
            "  Breaks: {}",
            self.install_relation
                .breaks
                .iter()
                .map(|pkg| format!("{}", pkg))
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        writeln!(
            f,
            "  Conflicts: {}",
            self.install_relation
                .conflicts
                .iter()
                .map(|pkg| format!("{}", pkg))
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        writeln!(
            f,
            "  Replaces: {}",
            self.install_relation
                .replaces
                .iter()
                .map(|pkg| format!("{}", pkg))
                .collect::<Vec<_>>()
                .join(", ")
        )?;

        // Build Relationsの表示
        writeln!(f, "Build Relations:")?;
        writeln!(
            f,
            "  Depends: {}",
            format_dep(&self.build_relation.depends)
        )?;

        // Virtual Packagesの表示
        writeln!(
            f,
            "Virtual Packages: {}",
            self.virt_pkg
                .iter()
                .map(|pkg| format!("{}", pkg))
                .collect::<Vec<_>>()
                .join(", ")
        )?;

        Ok(())
    }
}

// テストコード
#[cfg(test)]
mod tests {
    use super::PackageInfo;

    #[test]
    fn pkg_test() {
        let test_pkginfo = PackageInfo::new();
        println!("{}", test_pkginfo);
    }
}