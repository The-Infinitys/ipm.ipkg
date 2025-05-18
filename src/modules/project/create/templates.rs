use crate::utils::files::file_creation;
use std::io::Error;

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
            path: String::from("scripts/install.sh"),
            content: include_str!("templates/default/scripts/install.sh").to_string(),
        },
        SetUpList {
            path: String::from("scripts/remove.sh"),
            content: include_str!("templates/default/scripts/remove.sh").to_string(),
        },
        SetUpList {
            path: String::from("scripts/purge.sh"),
            content: include_str!("templates/default/scripts/purge.sh").to_string(),
        },
    ];
    setup(setup_list)
}
