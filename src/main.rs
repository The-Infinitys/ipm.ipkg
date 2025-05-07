use ipkg::modules::version;

fn main() {
    let test_ver = version::Version::from_str("1.2.3");
    println!("Hello, world!: {}", &test_ver);
}
