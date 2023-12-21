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

pub fn solve_1a() {
    if let Ok(lines) = read_lines(Path::new("./data/day1.txt")) {
	let res: u32 = lines.map(|line| {
	    let (l,r) = parse_line(line.unwrap()).unwrap();
	    l * 10 + r
	}).sum();
	println!("{}", res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_givens() {
	for (s, expected) in vec![
	    ("1abc2", (1,2)),
	    ("pqr3stu8vwx", (3,8)),
	    ("a1b2c3d4e5f", (1,5)),
	    ("treb7uchet", (7,7)),
	] {
	    assert_eq!(parse_line(s.to_string()), Ok(expected));
	}
    }
}
