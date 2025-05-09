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
    pub depend: Vec<DependPackageData>,
    pub conflict: Vec<DependPackageData>,
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
            for dep in &self.relation.depend {
                writeln!(f, "  - {} ({})", dep.name.green(), dep.version)?;
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
                    name: "".to_string(),
                    email: "".to_string(),
                },
                package: PackageAboutData {
                    name: "".to_string(),
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
