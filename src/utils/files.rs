use std::{env, io::{Error, Write}, path};
// 指定されたパスのディレクトリを作成します
// 関数の戻り値型を Result<(), ()> から Result<(), ProjectCreationError> に変更します
pub fn dir_creation(path_str: &str) -> Result<(), Error> {
    let path = path::Path::new(path_str);
    // std::fs::create_dir_all が返す std::io::Error は
    // ProjectCreationError::IoError に自動変換されます ('?' 演算子と #[from] 属性のおかげ)
    std::fs::create_dir_all(path)?;

    // 成功した場合に Ok(()) を返します
    Ok(())
}
pub fn file_creation(path_str: &str, content: &str) -> Result<(), Error> {
    let path = path::Path::new(path_str);
    let mut file = std::fs::File::create(path)?;
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
