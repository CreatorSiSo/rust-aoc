pub fn part_1(input: &str) -> i32 {
	input
		.chars()
		.fold(0, |sum, c| if c == '(' { sum + 1 } else { sum - 1 })
}
