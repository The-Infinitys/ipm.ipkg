struct CargoPackageInfo {
    name: &'static str,
    version: &'static str,
    architecture: &'static str,
}
fn get_info() -> CargoPackageInfo {
    CargoPackageInfo {
        name: option_env!("CARGO_PKG_NAME").unwrap_or("ipkg"),
        version: option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"),
        architecture: std::env::consts::ARCH,
    }
}

pub fn welcome() {
    let welcome_str = include_str!("./welcome/welcome.txt");
    let cargo_package = get_info();
    let replace_list = vec![
        ["name", cargo_package.name],
        ["version", cargo_package.version],
        ["architecture", cargo_package.architecture],
    ];
    let mut welcome_str = welcome_str.to_string();
    for replaces in replace_list {
        welcome_str = welcome_str.replace(format!("{{{}}}", replaces[0]).as_str(), replaces[1]);
    }
    println!("{}", welcome_str);
}

pub fn version() {
    let cargo_package = get_info();
    println!("{} {}", cargo_package.name, cargo_package.version);
}
