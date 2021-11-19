use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn entry() {
	println!("Starting challenge one!");

	let input_numbers = read_input_numbers("./resources/challenge_one_input.txt");
	let (first_number, second_number) = find_2020_numbers(&input_numbers);

	println!("Found a number pair that sums up to 2020: {}, {}", first_number, second_number);
	println!("Challenge answer is {}", first_number * second_number);
}

pub fn read_input_numbers(filename: &str) -> Vec<u32> {
	let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

	let mut numbers = vec!();
    for line in reader.lines() {
        numbers.push(line.unwrap().parse::<u32>().unwrap());
    }

	numbers
}

fn find_2020_numbers(numbers: &Vec<u32>) -> (&u32, &u32) {
	for first_number in numbers {
		for second_number in numbers {
			if first_number + second_number == 2020 {
				return (first_number, second_number);
			}
		}
	}

	panic!("Didn't find a pair of numbers that sum up to 2020...");
}