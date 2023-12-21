use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<A>(path: A) -> io::Result<io::Lines<io::BufReader<File>>>
where A: AsRef<Path>, {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line_a(line: String) -> std::result::Result<(u32, u32), &'static str> {
    if let Some(left) = line.chars().find_map(|x| x.to_digit(10)) {
	Ok((left, line.chars().rev().find_map(|x| x.to_digit(10)).unwrap()))
    } else {
	Err("Invalid line")
    }
}

fn parse_line_b(line: String) -> std::result::Result<(u32, u32), &'static str> {
    let mut lookup: HashMap<String, u32> = (0..10)
	.map(|x|(x.to_string(), x))
	.into_iter()
	.collect();
    lookup.extend([
	("one", 1),
	("two", 2),
	("three", 3),
	("four", 4),
	("five", 5),
	("six", 6),
	("seven", 7),
	("eight", 8),
	("nine", 9),
    ].iter().map(|(x,y)| (x.to_string(), *y)));
    let mut left: Option<u32> = None;
    'top: for i in 0..line.len() {
	let ss: String = line.chars().skip(i).collect();
	for (repr, value) in &lookup {
	    if ss.starts_with(repr) {
		left = Some(*value);
		break 'top;
	    }
	}
    }
    if let Some(left_val) = left {
	let mut right: Option<u32> = None;
	'top: for i in (0..line.len()).rev() {
	    let ss: String = line.chars().skip(i).collect();
	    for (repr, value) in &lookup {
		if ss.starts_with(repr) {
		    right = Some(*value);
		    break 'top;
		}
	    }
	}
	return Ok((left_val, right.unwrap()))
    } else {
	Err("Invalid line")	
    }
}

fn solve_1(f: fn(String) -> std::result::Result<(u32, u32), &'static str>) {
    if let Ok(lines) = read_lines(Path::new("./data/day1.txt")) {
	let res: u32 = lines.map(|line| {
	    let (l,r) = f(line.unwrap()).unwrap();
	    l * 10 + r
	}).sum();
	println!("{}", res);
    }
}

pub fn solve_1a() {
    solve_1(parse_line_a);
}

pub fn solve_1b() {
    solve_1(parse_line_b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_a_givens() {
	for (s, expected) in vec![
	    ("1abc2", (1,2)),
	    ("pqr3stu8vwx", (3,8)),
	    ("a1b2c3d4e5f", (1,5)),
	    ("treb7uchet", (7,7)),
	] {
	    assert_eq!(parse_line_a(s.to_string()), Ok(expected));
	}
    }

    #[test]
    fn parse_line_b_givens() {
	for (s, expected) in vec![
	    ("two1nine", (2,9)),
	    ("eightwothree", (8,3)),
	    ("abcone2threexyz", (1,3)),
	    ("xtwone3four", (2,4)),
	    ("4nineeightseven2", (4,2)),
	    ("zoneight234", (1,4)),
	    ("7pqrstsixteen", (7,6)),
	] {
 	    assert_eq!(parse_line_b(s.to_string()), Ok(expected));
	}
    }
}
