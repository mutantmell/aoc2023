use std::collections::HashMap;
use std::str::FromStr;



#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    Red, Blue, Green
}

impl FromStr for Color {
    type Err = ();
    fn from_str(input: &str) -> Result<Color, Self::Err> {
	match input {
	    "red" => Ok(Color::Red),
	    "blue" => Ok(Color::Blue),
	    "green" => Ok(Color::Green),
	    _ => Err(()),
	}
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    ix: i64,
    plays: Vec<HashMap<Color, i64>>,
}

fn parse_line(line: String) -> Result<Game, &'static str> {
    fn parse_play(play: String) -> Result<(Color, i64), &'static str> {
	let parts: Vec<_> = play.split(' ').collect();
	if let (Ok(color), Ok(count)) = (
	    (*parts[1]).parse::<Color>(),
	    (*parts[0]).parse::<i64>(),
	) {
	    Ok((color, count))
	} else {
	    Err("invalid line")
	}
    }
    fn parse_sets(sets: String) -> Result<HashMap<Color, i64>, &'static str> {
	sets.split(", ").map(|play| parse_play(play.to_string())).collect()
    }
    fn parse_plays(plays: String) -> Result<Vec<HashMap<Color, i64>>, &'static str> {
	plays.split("; ").map(|set| parse_sets(set.to_string())).into_iter().collect()
    }
    let split: Vec<_> = line.split(": ").collect();
    parse_plays(split[1].to_string()).and_then(|plays| {
	let header: Vec<_> = split[0].split(' ').collect();
	header[1].parse::<i64>().map(|ix| Game {
	    ix: ix,
	    plays: plays,
	}).map_err(|_| "invalid header")
    })
}

fn is_possible(g: &Game) -> bool {
    let bag: HashMap<Color, i64> = [
	(Color::Red, 12),
	(Color::Green, 13),
	(Color::Blue, 14),
    ].into();

    (*g).plays.iter().all(
	|p| bag.iter().all(
	    |(c, limit)| p.get(&c).into_iter().all(|game_val| game_val <= &limit)
	)
    )
}

fn solve(games: Vec<Game>) -> i64 {
    games.into_iter().filter(is_possible).map(|g| g.ix).sum()
}

pub fn solve_2a(lines: Vec<String>) -> Result<String, &'static str> {
    let parsed: Result<Vec<Game>, &'static str> = lines.into_iter().map(|line| parse_line(line)).into_iter().collect();
    parsed.map(|games| solve(games).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn games() -> Vec<(&'static str, Game)> {
	vec![
	    ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", Game {
		ix: 1,
		plays: vec![
		    [(Color::Blue, 3), (Color::Red, 4)].into(),
		    [(Color::Red, 1), (Color::Green, 2), (Color::Blue, 6)].into(),
		    [(Color::Green, 2)].into(),
		],
	    }),
	    ("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", Game {
		ix: 2,
		plays: vec![
		    [(Color::Blue, 1), (Color::Green, 2)].into(),
		    [(Color::Green, 3), (Color::Blue, 4), (Color::Red, 1)].into(),
		    [(Color::Green, 1), (Color::Blue, 1)].into(),
		],
	    }),
	    ("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", Game {
		ix: 3,
		plays: vec![
		    [(Color::Green, 8), (Color::Blue, 6), (Color::Red, 20)].into(),
		    [(Color::Blue, 5), (Color::Red, 4), (Color::Green, 13)].into(),
		    [(Color::Green, 5), (Color::Red, 1)].into(),
		],
	    }),
	    ("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", Game {
		ix: 4,
		plays: vec![
		    [(Color::Green, 1), (Color::Red, 3), (Color::Blue, 6)].into(),
		    [(Color::Green, 3), (Color::Red, 6)].into(),
		    [(Color::Green, 3), (Color::Blue, 15), (Color::Red, 14)].into(),
		],
	    }),
	    ("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", Game {
		ix: 5,
		plays: vec![
		    [(Color::Red, 6), (Color::Blue, 1), (Color::Green, 3)].into(),
		    [(Color::Blue, 2), (Color::Red, 1), (Color::Green, 2)].into(),
		],
	    }),
	]
    }

    #[test]
    fn parse_line_givens() {
	for (s, expected) in games() {
	    assert_eq!(parse_line(s.to_string()), Ok(expected));
	}
    }

    #[test]
    fn test_solve() {
	let games_only: Vec<Game> = games().into_iter().map(|(_s, v)| v).collect();
	assert_eq!(solve(games_only), 8);
    }
}
