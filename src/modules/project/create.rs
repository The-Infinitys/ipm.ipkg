use std::io;
use thiserror::Error;
mod templates;
use super::super::pkg::AuthorAboutData;
use super::super::pkg::PackageData;
use crate::utils::files::file_creation;
use std::fmt::{self, Display, Formatter};
#[derive(PartialEq, Eq)]
pub enum ProjectTemplateType {
    Default,
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
    // serde_yaml::to_string が返す serde_yaml::Error は
    // ProjectCreationError::YamlError に自動変換されます ('?' 演算子と #[from] 属性のおかげ)
    let data = serde_yaml::to_string(&project_data)?;

    // std::fs::write が返す std::io::Error は
    // ProjectCreationError::IoError に自動変換されます ('?' 演算子と #[from] 属性のおかげ)
    file_creation(project_data_filename, &data)?;
    match params.project_template {
        ProjectTemplateType::Default => match templates::default() {
            Ok(()) => Ok(()),
            Err(e) => {
                eprintln!("Error: {}", e);
                Err(ProjectCreationError::IoError(e))
            }
        },
    }
}
