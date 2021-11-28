use std::collections::{HashMap, HashSet};
use std::io::{prelude::*, BufReader};
use std::fs::File;

pub fn entry() {
	println!("Starting challenges for day six!");

	let answers = read_people_answers("./resources/day_six_input.txt");

    let mut distinct_answers_per_group = vec!();
    let mut same_answers_per_group = vec!();
    for group_answers in answers {
        distinct_answers_per_group.push(aggregate_distinct_answers(&group_answers));
        same_answers_per_group.push(aggregate_same_answers(&group_answers));
    }
    
    let mut sum_distinct = 0;
    for distinct_per_group in distinct_answers_per_group {
        //println!("{:?}", distinct_per_group);
        sum_distinct += distinct_per_group.len();
    }

    let mut sum_same = 0;
    for same_per_group in same_answers_per_group {
        println!("{:?}", same_per_group);
        sum_same += same_per_group.len();
    }

    println!("Sum of distinct answers is {}", sum_distinct);
    println!("Sum of same answers is {}", sum_same);
}

fn read_people_answers(filename: &str) -> Vec<Vec<String>> {
	let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

	let mut groups_answers = vec!();
    let mut current_group = vec!();
    for line in reader.lines() {
        let line = line.unwrap();

        if line.len() == 0 {
            groups_answers.push(current_group);
            current_group = vec!();
            continue;
        }

        current_group.push(line);
    }
    groups_answers.push(current_group);

    groups_answers
}

fn aggregate_distinct_answers(group_answers: &Vec<String>) -> HashSet<char> {
    let mut distinct_answers: HashSet<char> = HashSet::new();
    for person_answers in group_answers {
        distinct_answers.extend(person_answers.chars());
    }

    distinct_answers
}

fn aggregate_same_answers(group_answers: &Vec<String>) -> Vec<char> {
    let number_people = group_answers.len();

    let mut answers_count = HashMap::new();
    for person_answers in group_answers {
        for answer in person_answers.chars() {
            let answer_counter = answers_count.entry(answer).or_insert(0);
            *answer_counter += 1;
        }
    }

    let mut same_answers = vec!();
    for answer_count in answers_count {
        if answer_count.1 == number_people {
            same_answers.push(answer_count.0);
        }
    }

    same_answers
}