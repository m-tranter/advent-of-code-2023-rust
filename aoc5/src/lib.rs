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
static RX_MAP: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+)\s(\d+)\s(\d+)").unwrap());

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

#[derive(Clone, Debug)]
struct Range(i64, i64);
// impl Range {
//     fn contains(&self, n: i64) -> bool {
//         if n >= self.0 && n <= self.1 {
//             return true;
//         }
//         false
//     }
// }

#[derive(Debug)]
struct Map {
    offset: i64,
    range: Range,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn do_map_range(maps: &[Map], ranges: Vec<Range>) -> Vec<Range> {
    let mut working = ranges;
    let mut done = Vec::new();
    for map in maps {
        let mut holding = Vec::new();
        for range in working {
            if range.0 < map.range.0 {
                holding.push(Range(range.0, min(range.1, map.range.0 - 1)));
            }
            if range.1 > map.range.1 {
                holding.push(Range(max(range.0, map.range.1 + 1), range.1));
            }
            if range.1 >= map.range.0 && range.0 <= map.range.1 {
                done.push(Range(
                    max(range.0, map.range.0) + map.offset,
                    min(range.1, map.range.1) + map.offset,
                ));
            }
        }
        working = holding;
    }
    done.extend(working);
    done
}

// Map function from part 1
// fn do_map(maps: &Vec<Map>, n: i64) -> i64 {
//     let mut s = n;
//     for map in maps {
//         if map.range.contains(n) {
//             s = n + map.offset;
//             break;
//         }
//     }
//     s
// }

// Main part of code
pub fn run(config: Config) -> Result<i64, Box<dyn Error>> {
    let mut res = i64::MAX;
    let mut seeds: Vec<i64> = Vec::new();
    let mut maps: Vec<Vec<Map>> = Vec::new();

    if let Ok(input) = read_lines(config.file_path) {
        let lines: Vec<String> = input.map_while(Result::ok).collect();
        let mut l_iter = lines.iter();

        // Parse input
        while l_iter.len() > 0 {
            let line = l_iter.next().unwrap();
            if !line.is_empty() {
                if line.starts_with("seeds:") {
                    for (_, [num]) in RX_NUM.captures_iter(line).map(|c| c.extract()) {
                        seeds.push(num.parse().unwrap());
                    }
                } else if line.contains("map:") {
                    let mut temp_maps: Vec<Map> = Vec::new();
                    while l_iter.len() > 0 {
                        let line = l_iter.next().unwrap();
                        if !line.is_empty() {
                            for (_, [d_start, s_start, range]) in
                                RX_MAP.captures_iter(line).map(|caps| caps.extract())
                            {
                                let s: i64 = s_start.parse().unwrap();
                                let d: i64 = d_start.parse().unwrap();
                                let r: i64 = range.parse().unwrap();
                                temp_maps.push(Map {
                                    range: Range(s, s + r),
                                    offset: d - s,
                                });
                            }
                        } else {
                            break;
                        }
                    }
                    maps.push(temp_maps);
                }
            }
        }

        //Display the transformations found
        // for elem in &maps {
        //     println!();
        //     for map in elem {
        //         println!("{}, {:?}", map.offset, map.range);
        //     }
        // }

        // Part two
        let now = Instant::now();
        let mut s_iter = seeds.iter();
        let mut working: Vec<Range> = Vec::new();

        // Construct ranges vector
        while s_iter.len() >= 2 {
            let start: i64 = *s_iter.next().unwrap();
            working.push(Range(start, start + *s_iter.next().unwrap()));
        }

        for cat in &maps {
            working = do_map_range(cat, working);
        }

        for r in working {
            if r.0 < res {
                res = r.0
            }
        }

        // Part one
        // for mut s in seeds {
        //     for cat in &maps {
        //         s = do_map(cat, s);
        //     }
        //     if s < res {
        //         res = s;
        //     }
        // }

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
    Ok(res)
}
