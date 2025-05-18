use std::{
    env,
    io::{Error, Write},
    path,
};
// 指定されたパスのディレクトリを作成します
// 関数の戻り値型を Result<(), ()> から Result<(), ProjectCreationError> に変更します
pub fn dir_creation(path_str: &str) -> Result<(), Error> {
    let path = path::Path::new(path_str);
    // std::fs::create_dir_all が返す std::io::Error は
    // ProjectCreationError::IoError に自動変換されます ('?' 演算子と #[from] 属性のおかげ)
    // 注: 提供されたコードのコメントは ProjectCreationError に言及していますが、
    //      実際の戻り値型は std::io::Error です。コードの機能修正に焦点を当てます。
    std::fs::create_dir_all(path)?;

    // 成功した場合に Ok(()) を返します
    Ok(())
}

// ファイルを作成し、内容を書き込みます。
// ファイルの親ディレクトリが存在しない場合は自動的に作成します。
pub fn file_creation(path_str: &str, content: &str) -> Result<(), Error> {
    let path = path::Path::new(path_str);

    // ファイルの親ディレクトリを取得し、存在しない場合は作成する
    if let Some(parent_dir) = path.parent() {
        // parent() が Some を返す場合のみ処理を実行
        // std::fs::create_dir_all は既に存在するディレクトリに対しては何も行いません
        std::fs::create_dir_all(parent_dir)?;
    }

    // ファイルを作成 (親ディレクトリは既に存在保証されている)
    let mut file = std::fs::File::create(path)?;

    // ファイルに内容を書き込む
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn is_exists(path_str: &str) -> bool {
    env::current_dir().unwrap().join(path_str).exists()
}

pub fn is_file_exists(path_str: &str) -> bool {
    env::current_dir().unwrap().join(path_str).is_file()
}

pub fn is_dir_exists(path_str: &str) -> bool {
    env::current_dir().unwrap().join(path_str).is_dir()
}
