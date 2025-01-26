use std::error::Error;
use std::fs;

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path."),
        };
        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<i32, Box<dyn Error>> {
    let mut sum = 0;
    let contents =
        fs::read_to_string(config.file_path).expect("Something went wrong reading the file.");
    for line in contents.lines() {
        let chars: Vec<char> = line
            .chars()
            .filter(|ch| ch.is_ascii_digit())
            .collect::<Vec<_>>();
        sum += format!("{}{}", chars[0], chars[chars.len() - 1])
            .parse::<i32>()
            .unwrap();
    }
    Ok(sum)
}
