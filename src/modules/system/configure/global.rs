use crate::utils::files::file_creation;
use std::env;
use std::io::Error;
use std::path::Path;
pub fn configure() -> Result<(), Error> {
    let home_dir = env::var("HOME").unwrap();
    let home_dir = Path::new(&home_dir);
    file_creation(home_dir.join("").to_str().unwrap(), "")?;
    Ok(())
}
