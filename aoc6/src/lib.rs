use regex::Regex;
use std::cmp::{max, min};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::LazyLock;
use std::time::Instant;

// Creates regular expressions
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn wins(t: u64, d: u64) -> u64 {
    let mut first = 1;
    let mut last = t;
    while first * (t - first) <= d {
        first += 1;
    }
    while last * (t - last) <= d {
        last -= 1;
    }
    println!("{}, {}", first, last);
    last - first + 1
}

// Main part of code
pub fn run(config: Config) -> Result<u64, Box<dyn Error>> {
    if wins(7, 9) != 4 {
        panic!();
    }
    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();
    let mut product: u64 = 1;
    if let Ok(input) = read_lines(config.file_path) {
        let lines: Vec<String> = input.map_while(Result::ok).collect();
        let l_iter = lines.iter();
        // Parse input
        for line in l_iter {
            let mut nums: Vec<u64> = Vec::new();
            for (_, [num]) in RX_NUM.captures_iter(line).map(|c| c.extract()) {
                nums.push(num.parse().unwrap());
            }
            if line.starts_with("Time:") {
                times.extend(nums);
            } else {
                distances.extend(nums);
            }
        }
        let now = Instant::now();
        for i in 0..times.len() {
            product *= wins(times[i], distances[i]);
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
    Ok(product)
}
