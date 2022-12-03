pub fn generator(input: &str) -> Vec<(&str, &str)> {
	input
		.lines()
		.map(|line| {
			let mut iter = line.split_whitespace();
			let left = iter.next().unwrap();
			let right = iter.next().unwrap();
			(left, right)
		})
		.collect()
}

fn calc_score(other: Shape, own: Shape) -> u32 {
	own as u32
		+ match (own, other) {
			// draw
			(Shape::Rock, Shape::Rock)
			| (Shape::Paper, Shape::Paper)
			| (Shape::Scissors, Shape::Scissors) => 3,
			// win
			(Shape::Rock, Shape::Scissors) => 6,
			(Shape::Paper, Shape::Rock) => 6,
			(Shape::Scissors, Shape::Paper) => 6,
			// loose
			(Shape::Rock, Shape::Paper) => 0,
			(Shape::Paper, Shape::Scissors) => 0,
			(Shape::Scissors, Shape::Rock) => 0,
		}
}

pub fn part_1<'a>(input: &[(&'a str, &'a str)]) -> u32 {
	input
		.into_iter()
		.map(|(left, right)| (Shape::from(left), Shape::from(right)))
		.fold(0, |score, (other, own)| score + calc_score(other, own))
}

pub fn part_2<'a>(input: &[(&'a str, &'a str)]) -> u32 {
	input
		.into_iter()
		.map(|(left, right)| (Shape::from(left), Guide::from(right)))
		.fold(0, |score, (other, guide)| {
			score
				+ calc_score(
					other,
					match (guide, other) {
						(Guide::Draw, other) => other,
						(Guide::Loose, Shape::Rock) => Shape::Scissors,
						(Guide::Loose, Shape::Paper) => Shape::Rock,
						(Guide::Loose, Shape::Scissors) => Shape::Paper,
						(Guide::Win, Shape::Rock) => Shape::Paper,
						(Guide::Win, Shape::Paper) => Shape::Scissors,
						(Guide::Win, Shape::Scissors) => Shape::Rock,
					},
				)
		})
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Shape {
	Rock = 1,
	Paper = 2,
	Scissors = 3,
}

impl Shape {
	fn from(input: &str) -> Self {
		match input {
			"A" | "X" => Self::Rock,
			"B" | "Y" => Self::Paper,
			"C" | "Z" => Self::Scissors,
			_ => panic!("Cant parse Shape from `{input}`"),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Guide {
	Loose = 1,
	Draw = 2,
	Win = 3,
}

impl Guide {
	fn from(input: &str) -> Self {
		match input {
			"A" | "X" => Self::Loose,
			"B" | "Y" => Self::Draw,
			"C" | "Z" => Self::Win,
			_ => panic!("Cant parse Guide from `{input}`"),
		}
	}
}
