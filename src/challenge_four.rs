use crate::challenge_three;

pub fn entry() {
	println!("Starting challenge four!");

	let pwd_db_entries = challenge_three::extract_policies_passwords("./resources/challenge_three_input.txt");

	let mut valid_pwds = 0;
	for pwd_db_entry in pwd_db_entries {
		if validate_password(&pwd_db_entry) {
			valid_pwds += 1;
		}
	}

	println!("Found {} valid passwords", valid_pwds);
}

fn validate_password(pwd_db_entry: &challenge_three::PwdDatabaseEntry) -> bool {
	//println!("Received the following entry: {}-{} {}: {}", pwd_db_entry.policy_lower_bound, pwd_db_entry.policy_upper_bound, pwd_db_entry.policy_letter, pwd_db_entry.pwd);

	let first_char = pwd_db_entry.pwd.chars().nth(usize::from(pwd_db_entry.policy_lower_bound) - 1).unwrap();
	let second_char = pwd_db_entry.pwd.chars().nth(usize::from(pwd_db_entry.policy_upper_bound) - 1).unwrap();
	
	if first_char == pwd_db_entry.policy_letter && second_char == pwd_db_entry.policy_letter {
		//println!("Found an invalid password: {}-{} {}: {}", pwd_db_entry.policy_lower_bound, pwd_db_entry.policy_upper_bound, pwd_db_entry.policy_letter, pwd_db_entry.pwd);
		return false;
	} else if first_char != pwd_db_entry.policy_letter && second_char != pwd_db_entry.policy_letter {
		//println!("Found an invalid password: {}-{} {}: {}", pwd_db_entry.policy_lower_bound, pwd_db_entry.policy_upper_bound, pwd_db_entry.policy_letter, pwd_db_entry.pwd);
		return false;
	}

	true
}