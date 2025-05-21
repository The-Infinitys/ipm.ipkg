use crate::utils::shell;
use std::env;
use std::path::PathBuf; // PathBufを追加
pub fn packageslist_filepath() -> PathBuf {
    // 戻り値をPathBufに変更
    let home_path_str = env::var("HOME").unwrap_or_else(|_| {
        // unwrap_or_elseを使用
        // HOME環境変数が設定されていない場合
        eprintln!("Error: HOME environment variable not set. Attempting to use username.");
        let username = shell::username();
        format!("/home/{}", username)
    });

    let home_path = PathBuf::from(home_path_str); // PathBuf::fromでPathBufを作成

    // joinメソッドはPathではなくPathBufで呼び出すのが一般的
    home_path.join(".ipkg/packages/list.yaml")
}
