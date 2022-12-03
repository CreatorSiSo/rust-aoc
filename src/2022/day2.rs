use std::str::FromStr;

fn generator(input: &str) -> impl Iterator<Item = (&str, &str)> {
	input.lines().map(|line| {
		let mut iter = line.split_whitespace();
		let left = iter.next().unwrap();
		let right = iter.next().unwrap();
		debug_assert_eq!(iter.next(), None);
		(left, right)
	})
}

fn calc_score(other: Shape, own: Shape) -> u32 {
	own as u32
		+ match (own, other) {
			// loose
			(Shape::Rock, Shape::Paper)
			| (Shape::Paper, Shape::Scissors)
			| (Shape::Scissors, Shape::Rock) => 0,
			// draw
			(Shape::Rock, Shape::Rock)
			| (Shape::Paper, Shape::Paper)
			| (Shape::Scissors, Shape::Scissors) => 3,
			// win
			(Shape::Rock, Shape::Scissors)
			| (Shape::Paper, Shape::Rock)
			| (Shape::Scissors, Shape::Paper) => 6,
		}
}

pub fn part_1(input: &str) -> u32 {
	generator(input).fold(0, |score, (left, right)| {
		let other = left.parse().unwrap();
		let own = right.parse().unwrap();
		score + calc_score(other, own)
	})
}

pub fn part_2(input: &str) -> u32 {
	generator(input).fold(0, |score, (left, right)| {
		let other = left.parse().unwrap();
		let outcome = right.parse().unwrap();
		score
			+ calc_score(
				other,
				match (outcome, other) {
					(Outcome::Draw, _) => other,
					(Outcome::Loose, Shape::Rock) => Shape::Scissors,
					(Outcome::Loose, Shape::Paper) => Shape::Rock,
					(Outcome::Loose, Shape::Scissors) => Shape::Paper,
					(Outcome::Win, Shape::Rock) => Shape::Paper,
					(Outcome::Win, Shape::Paper) => Shape::Scissors,
					(Outcome::Win, Shape::Scissors) => Shape::Rock,
				},
			)
	})
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Shape {
	Rock = 1,
	Paper = 2,
	Scissors = 3,
}

impl FromStr for Shape {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"A" | "X" => Ok(Self::Rock),
			"B" | "Y" => Ok(Self::Paper),
			"C" | "Z" => Ok(Self::Scissors),
			_ => Err(format!("Unable to parse Shape from `{s}`")),
		}
	}
}

#[derive(PartialEq, Eq)]
pub enum Outcome {
	Loose = 1,
	Draw = 2,
	Win = 3,
}

impl FromStr for Outcome {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"X" => Ok(Self::Loose),
			"Y" => Ok(Self::Draw),
			"Z" => Ok(Self::Win),
			_ => Err(format!("Unable to parse Outcome from `{s}`")),
		}
	}
}
