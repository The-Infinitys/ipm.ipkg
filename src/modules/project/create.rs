use std::io;
use std::str::FromStr;
use thiserror::Error;
mod templates;
use super::super::package::AuthorAboutData;
use super::super::package::PackageData;
use crate::utils::files::file_creation;
use std::fmt::{self, Display, Formatter};
#[derive(PartialEq, Eq)]
pub enum ProjectTemplateType {
    Default,
    Rust,
}
impl FromStr for ProjectTemplateType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "default" => Ok(Self::Default),
            "rust" => Ok(Self::Rust),
            _ => Err(format!("Unavailable Template: {}", s)),
        }
    }
}
pub struct ProjectParams {
    pub project_name: String,
    pub project_template: ProjectTemplateType,
    pub author: AuthorAboutData,
}

impl Display for ProjectParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let template = match self.project_template {
            ProjectTemplateType::Default => "default",
            ProjectTemplateType::Rust => "rust",
        };
        write!(f, "Project: {}\nTemplate: {}", self.project_name, template)
    }
}

#[derive(Debug, Error)]
pub enum ProjectCreationError {
    #[error("YAML serialization error: {0}")]
    YamlError(#[from] serde_yaml::Error),
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
}

// 関数の戻り値型を Result<(), ()> から Result<(), ProjectCreationError> に変更します
pub fn create(params: &ProjectParams) -> Result<(), ProjectCreationError> {
    let mut project_data = PackageData::default();
    project_data.about.package.name = params.project_name.to_string();
    project_data.about.author = params.author.clone();
    let project_data_filename = "package.yaml";
    let data = serde_yaml::to_string(&project_data)?;
    file_creation(project_data_filename, &data)?;
    let creation_result = match params.project_template {
        ProjectTemplateType::Default => templates::default(),
        ProjectTemplateType::Rust => templates::rust(),
    };
    match creation_result {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(ProjectCreationError::IoError(e))
        }
    }
}
