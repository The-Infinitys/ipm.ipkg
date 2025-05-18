use std::{io, io::Write, path};
use thiserror::Error; // エラー型定義を簡単にするクレート (Cargo.toml に [dependencies] thiserror = "1.0" を追加)
// あるいは手動で std::fmt::Display と std::error::Error を実装しても良い

use super::super::pkg::PackageData;
use super::{ProjectParams, ProjectTemplateType};

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
    let project_data_filename = "project.yaml";
    // serde_yaml::to_string が返す serde_yaml::Error は
    // ProjectCreationError::YamlError に自動変換されます ('?' 演算子と #[from] 属性のおかげ)
    let data = serde_yaml::to_string(&project_data)?;

    // std::fs::write が返す std::io::Error は
    // ProjectCreationError::IoError に自動変換されます ('?' 演算子と #[from] 属性のおかげ)
    file_creation(project_data_filename, &data)?;
    dir_creation("scripts")?;
    match params.project_template {
        ProjectTemplateType::Default => {
            // 必要に応じて、ここでデフォルトテンプレート用のディレクトリ作成などを呼び出す
            // 例えば、 src ディレクトリを作成する場合:
            // dir_creation("src")?; // dir_creation のエラーも自動変換される
        }
    }

    // 成功した場合に Ok(()) を返します
    Ok(())
}

// 指定されたパスのディレクトリを作成します
// 関数の戻り値型を Result<(), ()> から Result<(), ProjectCreationError> に変更します
fn dir_creation(path_str: &str) -> Result<(), ProjectCreationError> {
    let path = path::Path::new(path_str);

    // std::fs::create_dir_all が返す std::io::Error は
    // ProjectCreationError::IoError に自動変換されます ('?' 演算子と #[from] 属性のおかげ)
    std::fs::create_dir_all(path)?;

    // 成功した場合に Ok(()) を返します
    Ok(())
}
fn file_creation(path_str: &str, content: &str) -> Result<(), ProjectCreationError> {
    let path = path::Path::new(path_str);
    let mut file = std::fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

// Note: このコードを使用するには、Cargo.toml に thiserror クレートを追加してください。
/*
[dependencies]
serde_yaml = "..." # 既存
serde = "..."      # 既存 (derive feature が必要かも)
thiserror = "1.0"
*/
