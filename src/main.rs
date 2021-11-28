use std::env;
mod challenge_one;
mod challenge_two;
mod challenge_three;
mod challenge_four;
mod challenge_five;
mod day_four;
mod day_five;
mod day_six;
mod day_seven;
mod day_eight;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 2 {
		panic!("Unexpected number of arguments. Expecting 2, got {}", args.len());
	}
    
	let challenge_number = &args[1].parse::<u8>().unwrap();

	let challenge_executor = match challenge_number {
		1 => challenge_one::entry,
		2 => challenge_two::entry,
		3 => challenge_three::entry,
		4 => challenge_four::entry,
		5 => challenge_five::entry,
		6 => day_four::entry,
		7 => day_five::entry,
		8 => day_six::entry,
		9 => day_seven::entry,
		10 => day_eight::entry,
		_ => panic!("Unknown/unimplemented challenge number")
	};

	challenge_executor();
}
