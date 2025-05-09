use colored::Colorize;
use std::io::{Write, stdin, stdout};
pub fn yesno(msg: &str) -> Result<bool, String> {
    print!("{}", msg);
    let mut input: String = String::new();
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");
    input = input.trim().to_lowercase();
    let s = input.as_str();
    match s {
        "yes" | "y" => Ok(true),
        "no" | "n" => Ok(false),
        _ => Err(format!("Invalid answer: {}", s)),
    }
}
pub fn yesno_loop(msg: &str) -> bool {
    loop {
        match yesno(msg) {
            Ok(answer) => return answer,
            Err(error) => {
                print!("({}) ", error.red());
                continue;
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let data = yesno_loop("Are you OK?: ");
        println!("{}", data);
    }
}
