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
    let setup_list = vec![SetUpList {
        path: String::from(""),
        content: String::from(""),
    }];
    setup(setup_list)
}
