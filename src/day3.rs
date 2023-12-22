use std::collections::HashMap;
use std::convert::TryFrom;

type Coord = (i32, i32);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Number {
    label: i64,
    start: Coord,
    len: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    numbers: Vec<Number>,
    symbols: HashMap<Coord, char> // Adjacency Map
}

fn parse_grid(grid: Vec<String>) -> Grid {
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: HashMap<Coord, char> = HashMap::new();
    for (x_ix, grid_line) in grid.into_iter().enumerate() {
	let x = u32::try_from(x_ix).unwrap();
	let x_i32 = i32::try_from(x).unwrap();

	let mut num: i64 = 0;
	let mut num_ix: i32 = i32::MAX;
	let mut y_i32: i32 = -1;

	for (y_ix, c) in grid_line.chars().enumerate() {
	    let y = u32::try_from(y_ix).unwrap();
	    y_i32 = i32::try_from(y).unwrap();
	    match c.to_digit(10) {
		Some(d) => {
		    num = (num * 10) + i64::from(d);
		    num_ix = i32::min(num_ix, y_i32);
		},
		None => {
		    if num > 0 {
			numbers.push(Number {
			    label: num,
			    start: (x_i32, num_ix),
			    len: y_i32 - num_ix,
			});
			num = 0;
			num_ix = i32::MAX;
		    };
		    if c != '.' {
			symbols.insert((x_i32, y_i32), c);
		    };
		}
	    }
	}
	if num > 0 {
	    numbers.push(Number {
		label: num,
		start: (x_i32, num_ix),
		len: y_i32 - num_ix,
	    });
	};
    }
    Grid {
	numbers: numbers,
	symbols: symbols,
    }
}

pub fn solve_3a(input: Vec<String>) -> Result<String, &'static str> {
    let grid = parse_grid(input);
    let mut sum: i64 = 0;

    let mut adjacencies = HashMap::new();
    for c in grid.symbols.keys() {
	for x_del in [-1,0,1] {
	    for y_del in [-1,0,1] {
		let coord = (c.0+x_del, c.1+y_del);
		if !adjacencies.contains_key(&coord) {
		    adjacencies.insert(coord, vec![]);
		};
		adjacencies.get_mut(&coord).unwrap().push(grid.symbols.get(c).unwrap());
	    };
	};
    }
    
    for n in grid.numbers {
	if (0..(n.len)).any(|x| adjacencies.contains_key(&(n.start.0, n.start.1+x))) {
	    sum += n.label;
	}
    }
    Ok(sum.to_string())
}

pub fn solve_3b(input: Vec<String>) -> Result<String, &'static str> {
    let grid = parse_grid(input);
    let mut adjacencies = HashMap::new();
    for n in grid.numbers {
	for x_del in [-1,0,1] {
	    for y_del in (-1)..(n.len+1) {
		let coord = (n.start.0+x_del, n.start.1+y_del);
		if !adjacencies.contains_key(&coord) {
		    adjacencies.insert(coord, vec![]);
		};
		adjacencies.get_mut(&coord).unwrap().push(n.label);
	    };
	};
    }

    let mut sum: i64 = 0;
    for (c, &s) in &grid.symbols {
	if s == '*' {
	    if let Some(adjacent) = adjacencies.get(c).filter(|v| v.len() == 2) {
		sum += adjacent.into_iter().product::<i64>()
	    }
	}
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid() -> Vec<String> {
	vec![
	    "467..114..",
	    "...*......",
	    "..35..633.",
	    "......#...",
	    "617*......",
	    ".....+.58.",
	    "..592.....",
	    "......755.",
	    "...$.*....",
	    ".664.598..",
	].into_iter().map(|x| x.to_string()).collect()
    }
    
    #[test]
    fn parse_grid_givens() {
	assert_eq!(parse_grid(grid()), Grid {
	    numbers: vec![
		Number { label: 467, start: (0, 0), len: 3 },
		Number { label: 114, start: (0, 5), len: 3 },
		Number { label: 35, start: (2, 2), len: 2 },
		Number { label: 633, start: (2, 6), len: 3 },
		Number { label: 617, start: (4, 0), len: 3 },
		Number { label: 58, start: (5, 7), len: 2 },
		Number { label: 592, start: (6, 2), len: 3 },
		Number { label: 755, start: (7, 6), len: 3 },
		Number { label: 664, start: (9, 1), len: 3 },
		Number { label: 598, start: (9, 5), len: 3 },
	    ],
	    symbols: [
		((1,3), '*'),
		((3,6), '#'),
		((4,3), '*'),
		((5,5), '+'),
		((8,3), '$'),
		((8,5), '*'),
	    ].into(),
	});
    }

    #[test]
    fn solve_a_given() {
	assert_eq!(solve_3a(grid()), Ok("4361".to_string()));
    }

    #[test]
    fn solve_b_given() {
	assert_eq!(solve_3b(grid()), Ok("467835".to_string()));
    }
}
