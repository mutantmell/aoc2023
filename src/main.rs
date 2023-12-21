use std::boxed::Box;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

mod day1;
mod day2;
mod day3;

type Solve = fn(Vec<String>) -> Result<String, &'static str>;

struct AoC {
    input: Box<std::path::PathBuf>,
    a: Solve,
    b: Option<Solve>,
}

fn read_lines<A>(path: &A) -> io::Result<io::Lines<io::BufReader<File>>>
where A: AsRef<Path>, {
    File::open(path).map(|file| io::BufReader::new(file).lines())
}

fn main() {
    let mut args = env::args();
    args.next();  // Program name
    let day = args.next().and_then(|a| a.parse::<i64>().ok()).unwrap_or(3);
    let sub_day = args.next().unwrap_or("a".to_string());

    let aoc: HashMap<i64, AoC> = [
	(1, AoC {
	    input: Box::new(PathBuf::from("./data/day1.txt")),
	    a: day1::solve_1a,
	    b: Some(day1::solve_1b),
	}),
	(2, AoC {
	    input: Box::new(PathBuf::from("./data/day2.txt")),
	    a: day2::solve_2a,
	    b: Some(day2::solve_2b),
	}),
	(3, AoC {
	    input: Box::new(PathBuf::from("./data/day3.txt")),
	    a: day3::solve_3a,
	    b: None,
	}),
    ].into();

    let problem = &aoc[&day];
    let read = read_lines(problem.input.as_ref()).map_err(|_| "invalid input");
    let lines = read.and_then(|r| r.into_iter().collect::<Result<_,_>>().map_err(|_| "also bad"));
    let part = match sub_day.as_ref() {
	"a" => Some(problem.a),
	"b" => problem.b,
	_ => None,
    };

    let result = match (lines, part) {
	(Ok(lines_ok), Some(part_some)) => part_some(lines_ok),
	_ => Err("Could not parse input"),
    };

    if let Ok(success) = result {
	println!("{}", success);
    } else {
	println!("Error!");
    }
}
