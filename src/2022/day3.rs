use std::collections::HashMap;

fn char_to_prio(c: char) -> u32 {
	match c {
		c @ 'a'..='z' => c as u32 - 96,
		c @ 'A'..='Z' => c as u32 - 64 + 26,
		c => panic!("Invalid character `{c}`"),
	}
}

pub fn part_1(input: &str) -> u32 {
	input
		.lines()
		.map(|line| line.split_at(line.len() / 2))
		.map(|(left, right)| {
			let maybe_duplicate = left.chars().find_map(|l_char| match right.find(l_char) {
				Some(_) => Some(l_char),
				None => None,
			});
			match maybe_duplicate {
				Some(c) => char_to_prio(c),
				None => 0,
			}
		})
		.sum()
}

pub fn part_2(input: &str) -> u32 {
	if !input.is_ascii() {
		panic!("Input must be ASCII")
	}

	input
		.lines()
		.collect::<Vec<&str>>()
		.chunks_exact(3)
		.map(|lines| {
			let mut amounts = HashMap::new();
			for line in lines {
				for c in line.chars() {
					amounts
						.entry(c)
						.and_modify(|amount| *amount += 1)
						.or_insert(1);
				}
			}
			let (common_char, _) = amounts
				.into_iter()
				.fold(('\0', 0), |accum, (char, amount)| {
					if lines.iter().all(|line| line.contains(char)) && amount > accum.1 {
						(char, amount)
					} else {
						accum
					}
				});
			char_to_prio(common_char)
		})
		.sum()
}
