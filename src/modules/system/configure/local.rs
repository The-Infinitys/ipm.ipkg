use crate::utils::files::file_creation;
use crate::utils::shell::shell_type;
use std::env;
use std::io::{Error, ErrorKind};
use std::path::Path;
pub fn configure() -> Result<(), Error> {
    let configure_list = [
        [".ipkg/README.md", include_str!("data/local/README.md")],
        [".ipkg/bin/ipkg-local", "data/local/ipkg-local"],
    ];
    let home_dir = env::var("HOME").map_err(|e| {
        Error::new(
            ErrorKind::NotFound,
            format!("HOME environment variable not found: {}", e),
        )
    })?;
    let home_dir = Path::new(&home_dir);

    // Create .ipkg/bin directory if it doesn't exist
    let ipkg_bin_dir = home_dir.join(".ipkg/bin");
    std::fs::create_dir_all(&ipkg_bin_dir).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("Failed to create .ipkg/bin directory: {}", e),
        )
    })?;

    // Process the configure list
    for configure_data in configure_list {
        let creation_result = file_creation(
            home_dir.join(configure_data[0]).to_str().unwrap(),
            configure_data[1],
        );
        match creation_result {
            Ok(()) => continue,
            Err(e) => {
                if e.kind() != ErrorKind::AlreadyExists {
                    return Err(e);
                }
            }
        }
    }

    // Check and update PATH in .profile
    let ipkg_bin_path = home_dir.join(".ipkg/bin").to_str().unwrap().to_string();
    let path_var = env::var("PATH").unwrap_or_default();
    if !path_var.split(':').any(|p| p == ipkg_bin_path) {
        let profile_path = home_dir
            .join(format!(".{}rc", shell_type()))
            .to_str()
            .unwrap()
            .to_string();
        let path_export = format!("\nexport PATH=\"$PATH:{}\"", ipkg_bin_path);
        let append_result = file_creation(&profile_path, &path_export);
        match append_result {
            Ok(()) => {
                println!("Added {} to PATH in {}", ipkg_bin_path, profile_path);
                println!("Maybe you have to add PATH manually")
            }
            Err(e) => {
                if e.kind() != ErrorKind::AlreadyExists {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("Failed to append to .profile: {}", e),
                    ));
                }
            }
        }
    }

    Ok(())
}
