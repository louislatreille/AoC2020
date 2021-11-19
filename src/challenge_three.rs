use std::io::{prelude::*, BufReader};
use std::fs::File;
use regex::Regex;

pub fn entry() {
	println!("Starting challenge three!");

	let pwd_db_entries = extract_policies_passwords("./resources/challenge_three_input.txt");

	let mut valid_pwds = 0;
	for pwd_db_entry in pwd_db_entries {
		if validate_password(&pwd_db_entry) {
			valid_pwds += 1;
		}
	}

	println!("Found {} valid passwords", valid_pwds);
}

pub struct PwdDatabaseEntry {
	pub policy_letter: char,
	pub policy_lower_bound: u8,
	pub policy_upper_bound: u8,
	pub pwd: String,
}

pub fn extract_policies_passwords(filename: &str) -> Vec<PwdDatabaseEntry> {
	let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

	let re = Regex::new(r"(?P<lower_bound>\d+)-(?P<upper_bound>\d+)\s+(?P<policy_letter>[a-z]):\s+(?P<pwd>[a-z]+)").unwrap();

	let mut pwd_db_entries = vec!();
    for line in reader.lines() {
		let line = line.unwrap();
		
		let re_match = re.captures(&line).unwrap();

        pwd_db_entries.push(PwdDatabaseEntry {
			policy_lower_bound: re_match["lower_bound"].parse::<u8>().unwrap(),
			policy_upper_bound: re_match["upper_bound"].parse::<u8>().unwrap(),
			policy_letter: re_match["policy_letter"].parse::<char>().unwrap(),
			pwd: re_match["pwd"].to_string(),
		});
    }

	pwd_db_entries
}

fn validate_password(pwd_db_entry: &PwdDatabaseEntry) -> bool {
	//println!("Received the following entry: {}-{} {}: {}", pwd_db_entry.policy_lower_bound, pwd_db_entry.policy_upper_bound, pwd_db_entry.policy_letter, pwd_db_entry.pwd);

	let mut count = 0;
	for character in pwd_db_entry.pwd.chars() {
		if character == pwd_db_entry.policy_letter {
			count += 1;
		}
	}

	if count >= pwd_db_entry.policy_lower_bound && count <= pwd_db_entry.policy_upper_bound {
		return true
	}

	//println!("Found an invalid password: {}-{} {}: {}", pwd_db_entry.policy_lower_bound, pwd_db_entry.policy_upper_bound, pwd_db_entry.policy_letter, pwd_db_entry.pwd);
	false
}