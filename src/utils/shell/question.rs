use colored::Colorize;
use regex::Regex;
use std::io::{Write, stdin, stdout};
fn str_input(msg: &str) -> String {
    print!("{}", msg);
    let mut input: String = String::new();
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");
    input
}
pub fn yesno(msg: &str) -> Result<bool, String> {
    let input = str_input(msg).trim().to_lowercase();
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

pub fn regex_string(msg: &str, regex: Regex) -> Result<String, String> {
    let input = str_input(msg).trim().to_string();
    match regex.is_match(&input) {
        true => Ok(input),
        false => Err(format!("Invalid input: {}", input)),
    }
}

pub fn camel_case(msg: &str) -> Result<String, String> {
    let camel_regex = Regex::new(r"^[A-Z][a-z]+(?:[A-Z][a-z]+)*$").unwrap();
    let input = str_input(msg).trim().to_string();
    match camel_regex.is_match(&input) {
        true => Ok(input),
        false => Err(format!("Invalid input: {}", input)),
    }
}
pub fn camel_loop(msg: &str) -> String {
    loop {
        match camel_case(msg) {
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
    #[test]
    fn test() {}
}
