pub fn welcome() {
    let welcome_str = include_str!("./welcome/welcome.txt");
    let welcome_str = welcome_str.replace("{name}", "ipkg");
    println!("{}", welcome_str);
}
