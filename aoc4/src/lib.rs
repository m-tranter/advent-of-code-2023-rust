use regex::Regex;
use std::error::Error;
use std::fs;
use std::sync::LazyLock;

static RX_NUM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+)").unwrap());

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
fn check_line(size: usize, line: &str) -> Vec<usize> {
    let mut lucky: Vec<u32> = Vec::new();
    let mut inds: Vec<usize> = Vec::new();
    let mut win = 0;
    let mut i: usize = 0;
    // Split the line to get card number etc.
    let pipe = line.chars().position(|c| c == '|').unwrap();
    let colon = line.chars().position(|c| c == ':').unwrap();
    if let Some(n) = RX_NUM.find(&line[..colon]) {
        i = n.as_str().parse().unwrap();
    }
    let card = &line[colon + 1..pipe];
    let mine = &line[pipe..];
    // Get the prize nums
    for (_, [num]) in RX_NUM.captures_iter(card).map(|c| c.extract()) {
        lucky.push(num.parse().unwrap());
    }
    // Get winning nums
    for (_, [num]) in RX_NUM.captures_iter(mine).map(|c| c.extract()) {
        if lucky.contains(&num.parse().unwrap()) {
            if win + i < size {
                inds.push(win + i);
            }
            win += 1;
        }
    }
    inds
}

// Recursive function - sum passes as mutable ref
fn recur(sum: &mut u32, cards: &[Vec<usize>], i: usize) {
    *sum += 1;
    let el = &cards[i];
    if !el.is_empty() {
        for j in el {
            recur(sum, cards, *j);
        }
    }
}

pub fn run(config: Config) -> Result<u32, Box<dyn Error>> {
    let mut cards: Vec<Vec<usize>> = Vec::new();
    let contents =
        fs::read_to_string(config.file_path).expect("Something went wrong reading the file.");
    for line in contents.lines() {
        cards.push(check_line(contents.lines().count(), line));
    }

    let mut total = 0;
    for k in 0..cards.len() {
        //println!("{}: {:?}", k, &cards[k]);
        if !cards[k].is_empty() {
            recur(&mut total, &cards, k);
        } else {
            total += 1
        }
    }
    Ok(total)
}
