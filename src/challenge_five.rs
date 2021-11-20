use std::io::{prelude::*, BufReader};
use std::fs::File;

pub fn entry() {
	println!("Starting challenge five!");

	let lines = read_lines("./resources/challenge_five_input.txt");

    let slope_11 = Slope { down: 1, right: 1};
    let slope_13 = Slope { down: 1, right: 3};
    let slope_15 = Slope { down: 1, right: 5};
    let slope_17 = Slope { down: 1, right: 7};
    let slope_21 = Slope { down: 2, right: 1};

    let mult: u64 = u64::from(sum_up_trees_for_slope(&lines, &slope_11));
    let mult = mult * u64::from(sum_up_trees_for_slope(&lines, &slope_13));
    let mult = mult * u64::from(sum_up_trees_for_slope(&lines, &slope_15));
    let mult = mult * u64::from(sum_up_trees_for_slope(&lines, &slope_17));
    let mult = mult * u64::from(sum_up_trees_for_slope(&lines, &slope_21));

    println!("Answer is {}!", mult);
}

pub fn read_lines(filename: &str) -> Vec<String> {
	let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

	let mut lines = vec!();
    for line in reader.lines() {
		let line = line.unwrap();
        lines.push(line);
    }

	lines
}

enum TreeNoTree {
    Tree,
    NoTree
}

fn determine_object(line: &String, index: usize) -> TreeNoTree {
    let mut owned_string: String = line.to_owned();
    while owned_string.len() <= index {
        owned_string.push_str(line);
    }

    match owned_string.chars().nth(usize::from(index)) {
        Some(character) => {
            if character == '.' {
                return TreeNoTree::NoTree;
            } else if character == '#' {
                return TreeNoTree::Tree;
            } else {
                panic!("Unrecognized character: {}", character);
            }
        }
        None => panic!("Out of bound, but that shouldn't happen!"),
    }
}

struct Slope {
    down: usize,
    right: usize,
}

fn sum_up_trees_for_slope(lines: &Vec<String>, slope: &Slope) -> u32 {
    let mut tree_number = 0;
    let mut hor_index = 0;
    for i in (0..lines.len()).step_by(slope.down) {
        let line = match lines.get(i) {
            Some(l) => l,
            None => break,
        };

        let object = determine_object(line, hor_index);
        hor_index += slope.right;

        match object {
            TreeNoTree::Tree => tree_number += 1,
            _ => (),
        }
    }

    println!("Found {} trees for slope {{ down: {}, right: {} }}", tree_number, slope.down, slope.right);

    tree_number
}