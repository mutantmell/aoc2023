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
    symbols: HashMap<Coord, Vec<char>> // Adjacency Map
}

fn parse_grid(grid: Vec<String>) -> Grid {
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: HashMap<Coord, Vec<char>> = HashMap::new();
    for (x_ix, grid_line) in grid.into_iter().enumerate() {
	let x = u32::try_from(x_ix).unwrap();
	let x_i32 = i32::try_from(x).unwrap();

	let mut num: i64 = 0;
	let mut num_ix: i32 = i32::MAX;

	for (y_ix, c) in grid_line.chars().enumerate() {
	    let y = u32::try_from(y_ix).unwrap();
	    let y_i32 = i32::try_from(y).unwrap();
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
			for x_del in [-1,0,1] {
			    for y_del in [-1,0,1] {
				let coord = (x_i32+x_del, y_i32+y_del);
				if !symbols.contains_key(&coord) {
				    symbols.insert(coord, vec![]);
				};
				symbols.get_mut(&coord).unwrap().push(c);
			    };
			};
		    };
		}
	    }
	}
    }
    Grid {
	numbers: numbers,
	symbols: symbols,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_grid_givens() {
	let grid = vec![
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
	].into_iter().map(|x| x.to_string()).collect();

	assert_eq!(parse_grid(grid), Grid {
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
		((0,2), vec!['*']),
		((0,3), vec!['*']),
		((0,4), vec!['*']),

		((1,2), vec!['*']),
		((1,3), vec!['*']),
		((1,4), vec!['*']),

		((2,2), vec!['*']),
		((2,3), vec!['*']),
		((2,4), vec!['*']),

		((2,5), vec!['#']),
		((2,6), vec!['#']),
		((2,7), vec!['#']),

		((3,2), vec!['*']),
		((3,3), vec!['*']),
		((3,4), vec!['*']),
		((3,5), vec!['#']),
		((3,6), vec!['#']),
		((3,7), vec!['#']),

		((4,2), vec!['*']),
		((4,3), vec!['*']),
		((4,4), vec!['*', '+']),
		((4,5), vec!['#', '+']),
		((4,6), vec!['#', '+']),
		((4,7), vec!['#']),

		((5,2), vec!['*']),
		((5,3), vec!['*']),
		((5,4), vec!['*', '+']),
		((5,5), vec!['+']),
		((5,6), vec!['+']),

		((6,4), vec!['+']),
		((6,5), vec!['+']),
		((6,6), vec!['+']),

		((7,2), vec!['$']),
		((7,3), vec!['$']),
		((7,4), vec!['$', '*']),
		((7,5), vec!['*']),
		((7,6), vec!['*']),

		((8,2), vec!['$']),
		((8,3), vec!['$']),
		((8,4), vec!['$', '*']),
		((8,5), vec!['*']),
		((8,6), vec!['*']),

		((9,2), vec!['$']),
		((9,3), vec!['$']),
		((9,4), vec!['$', '*']),
		((9,5), vec!['*']),
		((9,6), vec!['*']),
	    ].into(),
	});
    }
}
