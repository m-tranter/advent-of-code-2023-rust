use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

// Adapted from minigrep - config not really needed.
pub struct Config {
    pub file_path: String,
}

// Get the file_path from args.
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
    let numbers = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    let re = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let mut sum = 0;
    let contents =
        fs::read_to_string(config.file_path).expect("Something went wrong reading the file.");

    for line in contents.lines() {
        let mut digits = String::new();
        let mut i = 0;
        while i < line.len() {
            // Go through the line, finding every match.
            if let Some(my_match) = re.find_at(line, i) {
                let str = my_match.as_str();
                // Allow for overlapping matches.
                if str.len() > 1 {
                    i = my_match.end() - 1;
                } else {
                    i = my_match.end();
                }
                if str.len() == 1 {
                    digits.push_str(str);
                } else {
                    digits.push_str(numbers[str]);
                }
            } else {
                i = line.len();
            };
        }

        // Collect the digits and update the sum.
        if !digits.is_empty() {
            sum += format!(
                "{}{}",
                digits.chars().next().unwrap(),
                digits.chars().nth_back(0).unwrap()
            )
            .parse::<i32>()
            .unwrap();
        }
    }
    Ok(sum)
}
