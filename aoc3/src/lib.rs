use regex::Regex;
use std::error::Error;
use std::fs;
use std::sync::LazyLock;

static RX_NUM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+)").unwrap());
static RX_SYM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([^0-9.])").unwrap());

pub struct Config {
    pub file_path: String,
}

#[derive(Debug)]
pub struct PartNum {
    row: usize,
    start: usize,
    end: usize,
    num: i32,
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
    let lines: Vec<_> = contents.lines().collect();
    for (row, line) in contents.lines().enumerate() {
        for m in RX_NUM.find_iter(line) {
            let p = PartNum {
                row,
                start: m.start(),
                end: m.end(),
                num: m.as_str().parse().unwrap(),
            };
            if p.start > 0 && RX_SYM.find(&line[p.start - 1..p.start]).is_some() {
                sum += p.num;
            }
            if p.end < line.len() && RX_SYM.find(&line[p.end..p.end + 1]).is_some() {
                sum += p.num;
            }

            if p.row < lines.len() - 1 {
                let mut start = p.start;
                if p.start > 0 {
                    start = p.start - 1
                }
                let mut end = p.end;
                if p.end < line.len() - 1 {
                    end = p.end + 1;
                }
                let my_row = lines[p.row + 1];
                if RX_SYM.find(&my_row[start..end]).is_some() {
                    sum += p.num;
                };
            }
            if p.row > 0 {
                let mut start = p.start;
                if p.start > 0 {
                    start = p.start - 1
                }
                let mut end = p.end;
                if p.end < line.len() - 1 {
                    end = p.end + 1;
                }
                let my_row = lines[p.row - 1];
                if RX_SYM.find(&my_row[start..end]).is_some() {
                    sum += p.num;
                };
            }
        }
    }
    Ok(sum)
}
