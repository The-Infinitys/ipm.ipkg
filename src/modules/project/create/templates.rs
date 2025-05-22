use crate::utils::files::file_creation;
use crate::utils::shell;
use std::{
    io::{self, Error, ErrorKind},
    process::Command,
};

/// プロジェクトのセットアップに必要なファイルパスとコンテンツを保持する構造体。
///
/// この構造体は、テンプレートファイルパスとその内容を関連付けます。
struct SetUpItem {
    path: String,
    content: String,
}

/// 指定されたファイルリストに基づいてファイルを生成します。
///
/// 各ファイルは、そのパスとコンテンツに従って作成されます。
/// ファイル作成中にエラーが発生した場合、具体的なエラーメッセージと共に
/// `std::io::Error` が返されます。
///
/// # 引数
///
/// * `setup_list` - 生成するファイルのパスとコンテンツのリスト。
///
/// # 戻り値
///
/// ファイル生成がすべて成功した場合は `Ok(())`、一つでも失敗した場合は `std::io::Error` を返します。
fn setup_files(setup_list: Vec<SetUpItem>) -> Result<(), io::Error> {
    for item in setup_list {
        // file_creation の結果を直接伝播させ、エラー発生時に詳細な情報を付与する
        file_creation(&item.path, &item.content).map_err(|e| {
            Error::new(
                e.kind(),
                format!("Failed to create file '{}': {}", item.path, e),
            )
        })?;
    }
    Ok(())
}

/// デフォルトのプロジェクトテンプレートを設定します。
///
/// このテンプレートには、基本的なシェルスクリプト (`src/main.sh`) と、
/// `ipkg/scripts/` ディレクトリ内にビルド、インストール、削除、パージの各スクリプトが含まれます。
/// これらは新しいプロジェクトの初期構造を提供します。
///
/// # 戻り値
///
/// テンプレートの設定が成功した場合は `Ok(())`、ファイル作成に失敗した場合は `std::io::Error` を返します。
pub fn default() -> Result<(), io::Error> {
    let setup_list = vec![
        SetUpItem {
            path: "src/main.sh".to_string(),
            content: include_str!("templates/default/src/main.sh").to_string(),
        },
        SetUpItem {
            path: "ipkg/scripts/build.sh".to_string(),
            content: include_str!("templates/default/ipkg/scripts/build.sh").to_string(),
        },
        SetUpItem {
            path: "ipkg/scripts/install.sh".to_string(),
            content: include_str!("templates/default/ipkg/scripts/install.sh").to_string(),
        },
        SetUpItem {
            path: "ipkg/scripts/remove.sh".to_string(),
            content: include_str!("templates/default/ipkg/scripts/remove.sh").to_string(),
        },
        SetUpItem {
            path: "ipkg/scripts/purge.sh".to_string(),
            content: include_str!("templates/default/ipkg/scripts/purge.sh").to_string(),
        },
    ];
    setup_files(setup_list)
}

/// Rust プロジェクトテンプレートを設定します。
///
/// この関数は、最初にシステムに `cargo` コマンド（Rustのパッケージマネージャー）が
/// インストールされているかを確認します。`cargo` が利用可能な場合、`cargo init` を実行して
/// 標準的なRustプロジェクト構造を初期化し、その後、ipkg固有のビルド、インストール、
/// 削除、パージスクリプトを `ipkg/scripts/` ディレクトリ内に配置します。
///
/// # 戻り値
///
/// テンプレートの設定が成功した場合は `Ok(())`、`cargo` が見つからない場合や
/// コマンドの実行に失敗した場合は `std::io::Error` を返します。
pub fn rust() -> Result<(), io::Error> {
    // 'cargo' コマンドの利用可能性をチェック
    if !shell::is_cmd_available("cargo") {
        let rustup_url = "https://www.rust-lang.org/tools/install";
        eprintln!("Error: 'cargo' command not found.");
        eprintln!("To create a Rust project, you need to install Cargo (Rust's package manager).");
        eprintln!("Please visit {} for installation instructions.", rustup_url);
        return Err(Error::new(
            ErrorKind::NotFound,
            "Cargo command not found. Please install Rust and Cargo.",
        ));
    }

    // 'cargo init' を実行してRustプロジェクトを初期化
    let status = Command::new("cargo").arg("init").status().map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("Failed to execute 'cargo init': {}", e),
        )
    })?;

    if !status.success() {
        return Err(Error::new(
            ErrorKind::Other,
            format!("'cargo init' command failed with exit status: {}", status),
        ));
    }

    // ipkg スクリプトをRustプロジェクトに追加
    let setup_list = vec![
        SetUpItem {
            path: "ipkg/scripts/build.sh".to_string(),
            content: include_str!("templates/rust/ipkg/scripts/build.sh").to_string(),
        },
        SetUpItem {
            path: "ipkg/scripts/install.sh".to_string(),
            content: include_str!("templates/rust/ipkg/scripts/install.sh").to_string(),
        },
        SetUpItem {
            path: "ipkg/scripts/remove.sh".to_string(),
            content: include_str!("templates/rust/ipkg/scripts/remove.sh").to_string(),
        },
        SetUpItem {
            path: "ipkg/scripts/purge.sh".to_string(),
            content: include_str!("templates/rust/ipkg/scripts/purge.sh").to_string(),
        },
    ];
    setup_files(setup_list)
}
