use crate::utils::shell;
use std::env;
use std::path::PathBuf; // PathBufを追加
fn home_path() -> PathBuf {
    let home_path_str = env::var("HOME").unwrap_or_else(|_| {
        // unwrap_or_elseを使用
        // HOME環境変数が設定されていない場合
        eprintln!("Error: HOME environment variable not set. Attempting to use username.");
        let username = shell::username();
        format!("/home/{}", username)
    });
    PathBuf::from(home_path_str)
}
fn ipkg_path() -> PathBuf {
    home_path().join(".ipkg")
}
pub fn packageslist_filepath() -> PathBuf {
    packages_dirpath().join("list.yaml")
}
pub fn packages_dirpath() -> PathBuf {
    ipkg_path().join("packages")
}
pub fn cache_path() -> PathBuf {
    ipkg_path().join("caches")
}
