use aoc4::Config;
use aoc4::run;
use std::env;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {}", err);
        process::exit(1);
    });
    let sum = run(config);
    match sum {
        Ok(s) => println!("{}", s),
        Err(e) => {
            eprintln!("Application error. {}", e);
            process::exit(1);
        }
    }
}
