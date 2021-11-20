use crate::challenge_one;

pub fn entry() {
	println!("Starting challenge two!");

	let input_numbers = challenge_one::read_input_numbers("./resources/challenge_one_input.txt");
	let (first_number, second_number, third_number) = find_2020_numbers(&input_numbers);

	println!("Found three numbers that sum up to 2020: {}, {}, {}", first_number, second_number, third_number);
	println!("Challenge answer is {}", first_number * second_number * third_number);
}

fn find_2020_numbers(numbers: &Vec<u32>) -> (&u32, &u32, &u32) {
	for first_number in numbers {
		for second_number in numbers {
			for third_number in numbers {
				if first_number + second_number + third_number == 2020 {
					return (first_number, second_number, third_number);
				}
			}
		}
	}

	panic!("Didn't find any three numbers that sum up to 2020...");
}