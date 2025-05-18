use std::io;
use thiserror::Error;
mod templates;
use super::super::pkg::PackageData;
use super::{ProjectParams, ProjectTemplateType};
use crate::utils::files::{dir_creation, file_creation};

// カスタムエラー型を定義します
#[derive(Error, Debug)]
pub enum ProjectCreationError {
    #[error("YAML serialization error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    // 将来的に他の種類のエラーが増えた場合、ここに追加します
    // #[error("Template specific error: {0}")]
    // TemplateError(String),
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
    dir_creation("scripts")?;
    match params.project_template {
        ProjectTemplateType::Default => match templates::default() {
            Ok(()) => Ok(()),
            Err(e) => Err(ProjectCreationError::IoError(e)),
        },
    }
}
