use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::LazyLock;
use std::time::Instant;

static RX_HAND: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([2-9TJQKA]{5})\s+(\d+)").unwrap());

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

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
    cards: [u32; 5],
    value: u32,
    bid: u32,
}

fn set_value(cards: [u32; 5]) -> u32 {
    let mut freq: HashMap<u32, u32> = HashMap::new();
    let mut max: u32 = 0;
    for card in cards {
        *freq.entry(card.to_owned()).or_default() += 1;
        max = cmp::max(*freq.get(&card).unwrap(), max);
    }
    let length = freq.len() as u32;
    // This represents the hand's value
    // 8 = 5 of a kind etc.
    match max + 4 - length {
        8 => 6,
        0 => 0,
        n => n - 1,
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn make_hand(hand: &str, bid: &str) -> Hand {
    let chars: Vec<char> = hand.chars().collect();
    let mut numbers: Vec<u32> = Vec::new();
    for ch in chars {
        match ch {
            'A' => numbers.push(14),
            'K' => numbers.push(13),
            'Q' => numbers.push(12),
            'J' => numbers.push(11),
            'T' => numbers.push(10),
            n => numbers.push(n.to_digit(10).unwrap()),
        }
    }
    let slice = numbers.as_slice();
    let cards: [u32; 5] = match slice.try_into() {
        Ok(n) => n,
        Err(_) => panic!("Error making the array"),
    };
    Hand {
        cards,
        value: set_value(cards),
        bid: bid.parse().unwrap(),
    }
}

// Main part of code
pub fn run(config: Config) -> Result<u32, Box<dyn Error>> {
    // Test
    if set_value([2, 2, 2, 2, 2]) != 6 {
        panic!("5OK broken");
    }
    if set_value([2, 2, 2, 2, 3]) != 5 {
        panic!("4OK broken");
    }
    if set_value([2, 2, 2, 3, 3]) != 4 {
        panic!("FH broken");
    }
    if set_value([2, 2, 2, 3, 4]) != 3 {
        panic!("3OK broken");
    }
    if set_value([2, 2, 3, 3, 4]) != 2 {
        panic!("2 pairs broken");
    }
    if set_value([2, 2, 3, 4, 5]) != 1 {
        panic!("One pair broken");
    }
    if set_value([2, 3, 4, 5, 6]) != 0 {
        panic!("High card broken");
    }

    let now = Instant::now();
    let mut sum = 0;
    let mut hands: Vec<Hand> = Vec::new();
    if let Ok(input) = read_lines(config.file_path) {
        let lines: Vec<String> = input.map_while(Result::ok).collect();
        let l_iter = lines.iter();
        for line in l_iter {
            for (_, [hand, bid]) in RX_HAND.captures_iter(line).map(|c| c.extract()) {
                hands.push(make_hand(hand, bid));
            }
        }
        hands.sort_by(|a, b| {
            let mut cmp = a.value.cmp(&b.value);
            if cmp.is_eq() {
                cmp = a.cards.cmp(&b.cards);
            }
            cmp
        });
        for (i, hand) in hands.into_iter().enumerate() {
            sum += (i + 1) as u32 * hand.bid;
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
    Ok(sum)
}
