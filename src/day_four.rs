use std::io::{prelude::*, BufReader};
use std::fs::File;
use regex::Regex;

pub fn entry() {
	println!("Starting challenges for day four!");

	let lines = read_passports("./resources/day_four_input.txt");
    println!("Found {} passport entries", lines.len());

    let mut valid_passports = 0;
    for line in lines {
        let passport = PassportData::import_passport_data(&line);
        if passport.valid() {
            //println!("Found a valid passport {:?}", passport);
            valid_passports += 1;
        } else {
            //println!("Found an invalid passport {:?}", passport);
        }
    }

    println!("Found {} valid passports.", valid_passports);
}

#[derive(Debug)]
struct PassportData {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl PassportData {
    fn import_passport_data(passport_str: &str) -> PassportData {
        let re_byr: Regex = Regex::new(r"byr:(?P<byr>[^\s]+)").unwrap();
        let re_iyr: Regex = Regex::new(r"iyr:(?P<iyr>[^\s]+)").unwrap();
        let re_eyr: Regex = Regex::new(r"eyr:(?P<eyr>[^\s]+)").unwrap();
        let re_hgt: Regex = Regex::new(r"hgt:(?P<hgt>[^\s]+)").unwrap();
        let re_hcl: Regex = Regex::new(r"hcl:(?P<hcl>[^\s]+)").unwrap();
        let re_ecl: Regex = Regex::new(r"ecl:(?P<ecl>[^\s]+)").unwrap();
        let re_pid: Regex = Regex::new(r"pid:(?P<pid>[^\s]+)").unwrap();
        let re_cid: Regex = Regex::new(r"cid:(?P<cid>[^\s]+)").unwrap();

        PassportData {
            byr: PassportData::parse_string_for(passport_str, &re_byr, "byr"),
            iyr: PassportData::parse_string_for(passport_str, &re_iyr, "iyr"),
            eyr: PassportData::parse_string_for(passport_str, &re_eyr, "eyr"),
            hgt: PassportData::parse_string_for(passport_str, &re_hgt, "hgt"),
            hcl: PassportData::parse_string_for(passport_str, &re_hcl, "hcl"),
            ecl: PassportData::parse_string_for(passport_str, &re_ecl, "ecl"),
            pid: PassportData::parse_string_for(passport_str, &re_pid, "pid"),
            cid: PassportData::parse_string_for(passport_str, &re_cid, "cid"),
        }
    }

    fn parse_string_for(passport_str: &str, re: &Regex, code: &str) -> Option<String> {
        let caps = match re.captures(passport_str) {
            Some(caps) => caps,
            None => return None,
        };
        
        let content = caps[code].to_owned();
        return Some(content);
    }

    fn valid(&self) -> bool {
        return self.validate_byr() && self.validate_iyr() && self.validate_eyr() &&
            self.validate_hgt() && self.validate_hcl() && self.validate_ecl() &&
            self.validate_pid();
    }

    fn validate_byr(&self) -> bool {
        let byr = match &self.byr {
            Some(byr) => byr,
            None => return false,
        };

        let byr = match byr.parse::<u32>() {
            Ok(num) => num,
            Err(_) => return false,
        };

        return byr >= 1920 && byr <= 2002;
    }

    fn validate_iyr(&self) -> bool {
        let iyr = match &self.iyr {
            Some(iyr) => iyr,
            None => return false,
        };

        let iyr = match iyr.parse::<u32>() {
            Ok(num) => num,
            Err(_) => return false,
        };

        return iyr >= 2010 && iyr <= 2020;
    }

    fn validate_eyr(&self) -> bool {
        let eyr = match &self.eyr {
            Some(eyr) => eyr,
            None => return false,
        };

        let eyr = match eyr.parse::<u32>() {
            Ok(num) => num,
            Err(_) => return false,
        };

        return eyr >= 2020 && eyr <= 2030;
    }

    fn validate_hgt(&self) -> bool {
        let hgt = match &self.hgt {
            Some(hgt) => hgt,
            None => return false,
        };

        let re_hgt: Regex = Regex::new(r"(?P<num>\d+)(?P<unit>cm|in)").unwrap();
        let caps = match re_hgt.captures(&hgt) {
            Some(caps) => caps,
            None => return false,
        };

        let hgt_num = &caps["num"];
        let hgt_unit = &caps["unit"];

        let hgt_num = match hgt_num.parse::<u32>() {
            Ok(num) => num,
            Err(_) => return false,
        };

        return (hgt_unit == "in" && hgt_num >= 59 && hgt_num <= 76) || (hgt_unit == "cm" && hgt_num >= 150 && hgt_num <= 193);
    }

    fn validate_hcl(&self) -> bool {
        let hcl = match &self.hcl {
            Some(hcl) => hcl,
            None => return false,
        };

        let re_hcl: Regex = Regex::new(r"#[0-9a-fA-F]{6}").unwrap();
        match re_hcl.captures(&hcl) {
            Some(_) => return true,
            None => return false,
        };
    }

    fn validate_ecl(&self) -> bool {
        let ecl = match &self.ecl {
            Some(ecl) => ecl,
            None => return false,
        };

        return ecl == "amb" || ecl == "blu" || ecl == "brn" ||
            ecl == "gry" || ecl == "grn" || ecl == "hzl" ||
            ecl == "oth";
    }

    fn validate_pid(&self) -> bool {
        let pid = match &self.pid {
            Some(pid) => pid,
            None => return false,
        };

        let re_pid: Regex = Regex::new(r"\A[0-9]{9}\z").unwrap();
        match re_pid.captures(&pid) {
            Some(_) => return true,
            None => return false,
        };
    }
}

pub fn read_passports(filename: &str) -> Vec<String> {
	let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

	let mut passport_strings = vec!();
    let mut current_passport_string = "".to_owned();
    for line in reader.lines() {
        let line = line.unwrap();

        if line.len() == 0 {
            passport_strings.push(current_passport_string.clone());
            current_passport_string.clear();
            continue;
        }
		
        current_passport_string.push_str(" ");
        current_passport_string.push_str(&line);
    }

    passport_strings.push(current_passport_string.clone());

    passport_strings
}