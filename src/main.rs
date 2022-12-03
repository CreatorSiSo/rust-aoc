mod day1;
mod day2;

aoc_main::main! {
	year 2022;
	day1 : generator   => part_1, part_2;
	day2 : generator => part_1, part_2;
	// day3             => part_1, part_2; // no generator, a &str is passed
}
