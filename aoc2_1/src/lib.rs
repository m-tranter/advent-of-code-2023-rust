use regex::Regex;
use std::error::Error;
use std::fs;
use std::sync::LazyLock;

static RX_COL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+)\s*(green|red|blue)").unwrap());

pub struct Config {
    pub file_path: String,
}

#[derive(Debug)]
struct Round {
    red: i32,
    green: i32,
    blue: i32,
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

fn my_match(str: &str) -> Round {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for cap in RX_COL.captures_iter(str) {
        match &cap[2] {
            "red" => red = cap[1].parse().unwrap(),
            "blue" => blue = cap[1].parse().unwrap(),
            "green" => green = cap[1].parse().unwrap(),
            _ => println!("Error"),
        }
    }
    Round { red, blue, green }
}

pub fn run(config: Config) -> Result<i32, Box<dyn Error>> {
    let mut sum = 0;
    let contents =
        fs::read_to_string(config.file_path).expect("Something went wrong reading the file.");
    for line in contents.lines() {
        let mut test = Round {
            red: 0,
            green: 0,
            blue: 0,
        };
        let items = line.split(';');
        for item in items {
            let r = my_match(item);
            if r.red > test.red {
                test.red = r.red;
            }
            if r.blue > test.blue {
                test.blue = r.blue;
            }
            if r.green > test.green {
                test.green = r.green;
            }
        }
        sum += test.red * test.green * test.blue;
    }
    Ok(sum)
}
