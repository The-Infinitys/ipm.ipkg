use crate::utils::files::file_creation;
use crate::utils::shell;
use std::{
    io::{Error, ErrorKind},
    process::Command,
}; // ErrorKind を追加

struct SetUpList {
    path: String,
    content: String,
}

fn setup(setup_list: Vec<SetUpList>) -> Result<(), Error> {
    for setup in setup_list {
        file_creation(&setup.path, &setup.content)?;
    }
    Ok(())
}

pub fn default() -> Result<(), Error> {
    let setup_list = vec![
        SetUpList {
            path: String::from("src/main.sh"),
            content: include_str!("templates/default/src/main.sh").to_string(),
        },
        SetUpList {
            path: String::from("ipkg/scripts/build.sh"),
            content: include_str!("templates/default/ipkg/scripts/build.sh").to_string(),
        },
        SetUpList {
            path: String::from("ipkg/scripts/install.sh"),
            content: include_str!("templates/default/ipkg/scripts/install.sh").to_string(),
        },
        SetUpList {
            path: String::from("ipkg/scripts/remove.sh"),
            content: include_str!("templates/default/ipkg/scripts/remove.sh").to_string(),
        },
        SetUpList {
            path: String::from("ipkg/scripts/purge.sh"),
            content: include_str!("templates/default/ipkg/scripts/purge.sh").to_string(),
        },
    ];
    setup(setup_list)
}

pub fn rust() -> Result<(), Error> {
    if !shell::is_cmd_available("cargo") {
        let rustup_url = "https://www.rust-lang.org/";
        eprintln!("Cargo is not available.");
        eprintln!("You have to install Cargo for creating ipkg rust project.");
        eprintln!("For more information: {}", rustup_url);
        // cargoが利用できない場合はエラーを返す
        return Err(Error::new(ErrorKind::NotFound, "Cargo command not found"));
    }
    let status = Command::new("cargo").arg("init").status().map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("Failed to execute cargo init: {}", e),
        )
    })?; // エラー型を修正

    if !status.success() {
        // cargo initが成功しなかった場合のエラーメッセージを具体的にする
        return Err(Error::new(
            ErrorKind::Other,
            "cargo init command failed to execute successfully",
        ));
    }

    let setup_list = vec![
        SetUpList {
            path: String::from("ipkg/scripts/build.sh"),
            content: include_str!("templates/rust/ipkg/scripts/build.sh").to_string(),
        },
        SetUpList {
            path: String::from("ipkg/scripts/install.sh"),
            content: include_str!("templates/rust/ipkg/scripts/install.sh").to_string(),
        },
        SetUpList {
            path: String::from("ipkg/scripts/remove.sh"),
            content: include_str!("templates/rust/ipkg/scripts/remove.sh").to_string(),
        },
        SetUpList {
            path: String::from("ipkg/scripts/purge.sh"),
            content: include_str!("templates/rust/ipkg/scripts/purge.sh").to_string(),
        },
    ];
    setup(setup_list)
}
