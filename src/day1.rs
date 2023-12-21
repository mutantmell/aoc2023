use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<A>(path: A) -> io::Result<io::Lines<io::BufReader<File>>>
where A: AsRef<Path>, {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: String) -> std::result::Result<(u32, u32), &'static str> {
    if let Some(left) = line.chars().find_map(|x| x.to_digit(10)) {
	Ok((left, line.chars().rev().find_map(|x| x.to_digit(10)).unwrap()))
    } else {
	Err("Invalid line")
    }
}
