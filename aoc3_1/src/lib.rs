use regex::Regex;
use std::error::Error;
use std::fs;
use std::sync::LazyLock;

static RX_SYM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\*").unwrap());
static RX_DIG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+").unwrap());

pub struct Config {
    pub file_path: String,
}

#[derive(Debug)]
pub struct Gear {
    row: usize,
    ind: usize,
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

fn num_before(line: &str) -> Option<u32> {
    let rev: String = line.chars().rev().collect();
    if let Some(mat) = RX_DIG.find(&rev) {
        let num: String = mat.as_str().chars().rev().collect();
        return Some(num.parse().unwrap());
    }
    None
}

fn check_row(line: &str, i: usize) -> Vec<u32> {
    let mut gears: Vec<u32> = Vec::new();
    let ch = line.chars().nth(i).unwrap();
    // Char above is not a digit
    if !ch.is_ascii_digit() {
        // Check for number after
        if line.len() - 1 > i {
            if let Some(mat) = RX_DIG.find(&line[i + 1..]) {
                gears.push(mat.as_str().parse().unwrap());
            }
        }
        // Check for number before
        if i > 0 {
            if let Some(mat) = num_before(&line[0..i]) {
                gears.push(mat);
            }
        }
    } else if i > 0 {
        // Number "around"
        let mut s = i;
        while s > 0 {
            let ch = line.chars().nth(s - 1).unwrap();
            if !ch.is_ascii_digit() {
                break;
            }
            s -= 1;
        }
        if let Some(mat) = RX_DIG.find(&line[s..]) {
            gears.push(mat.as_str().parse().unwrap());
        }
    }
    gears
}

pub fn run(config: Config) -> Result<u32, Box<dyn Error>> {
    let mut sum = 0;
    let contents =
        fs::read_to_string(config.file_path).expect("Something went wrong reading the file.");
    let lines: Vec<_> = contents.lines().collect();
    for (row, line) in contents.lines().enumerate() {
        for m in RX_SYM.find_iter(line) {
            let mut gears: Vec<u32> = Vec::new();
            let g = Gear {
                row,
                ind: m.start(),
            };
            // Check for number after gear
            if g.ind < line.len() - 1 {
                if let Some(mat) = RX_DIG.find(&line[g.ind + 1..]) {
                    gears.push(mat.as_str().parse().unwrap());
                }
            }
            // Check for number before gear
            if g.ind > 0 {
                if let Some(n) = num_before(&line[..g.ind]) {
                    gears.push(n)
                }
            }
            // Check if next row has a gear
            if g.row > 0 {
                let res = check_row(lines[g.row - 1], g.ind);
                gears.extend(res);
            }
            // Check if prev row has a gear
            if g.row < lines.len() - 1 {
                let res = check_row(lines[g.row + 1], g.ind);
                gears.extend(res);
            }
            if gears.len() == 2 {
                sum += gears[0] * gears[1];
            }
            println!("{:?}", gears);
        }
    }
    Ok(sum)
}
