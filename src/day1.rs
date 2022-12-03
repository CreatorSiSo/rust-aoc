pub fn generator(input: &str) -> Vec<u32> {
	input
		.split("\n\n")
		.map(|calories_str| {
			calories_str
				.lines()
				.filter_map(|num_str| match num_str.parse::<u32>() {
					Ok(val) => Some(val),
					Err(_) => None,
				})
				.sum()
		})
		.collect()
}

pub fn part_1(input: &[u32]) -> u32 {
	let mut input = Vec::from(input);
	input.sort();
	input[0]
}

pub fn part_2(input: &[u32]) -> u32 {
	let mut input = Vec::from(input);
	input.sort();
	input.reverse();

	input[0] + input[1] + input[2]
}
