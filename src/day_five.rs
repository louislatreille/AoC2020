use std::io::{prelude::*, BufReader};
use std::fs::File;

pub fn entry() {
	println!("Starting challenges for day five!");

	let boarding_passes = read_boarding_passes("./resources/day_five_input.txt");
    
    let mut rows_columns = vec!();
    for boarding_pass in boarding_passes {
        rows_columns.push(extract_seat_row_column(&boarding_pass));
    }

    let mut seat_ids = vec!();
    for row_column in rows_columns {
        seat_ids.push(calculate_seat_id(row_column));
    }

    let mut highest_seat_id = 0;
    for seat_id in &seat_ids {
        if *seat_id > highest_seat_id {
            highest_seat_id = *seat_id;
        }
    }

    println!("Highest seat ID found is {}", highest_seat_id);

    seat_ids.sort();
    println!("Sorted seat IDs: {:?}", seat_ids);

    match find_missing_seat(&seat_ids) {
        Some(seat) => println!("My seat is {}", seat),
        None => println!("Didn't find any missing seat...")
    }
}

fn find_missing_seat(seat_ids: &Vec<u32>) -> Option<u32> {
    for i in 1 .. seat_ids.len() - 2 {
        if seat_ids.get(i).unwrap() - seat_ids.get(i-1).unwrap() != 1 {
            return Some(seat_ids.get(i).unwrap() - 1);
        }
    }

    None
}

fn calculate_seat_id(row_column: (u8, u8)) -> u32 {
    u32::from(row_column.0) * 8 + u32::from(row_column.1)
}

fn read_boarding_passes(filename: &str) -> Vec<String> {
	let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

	let mut boarding_passes = vec!();
    for line in reader.lines() {
        let line = line.unwrap();
        boarding_passes.push(line);
    }

    boarding_passes
}

fn extract_seat_row_column(boarding_pass: &str) -> (u8, u8) {
    (extract_seat_row(boarding_pass), extract_seat_column(boarding_pass))
}

fn extract_seat_row(boarding_pass: &str) -> u8 {
    //println!("Boarding pass directives {}", boarding_pass);

    let row_directives = boarding_pass.chars().take(7);

    let mut row = 0;
    let mut row_index = 64;
    for row_directive in row_directives {
        //println!("Got directive {}", row_directive);

        if row_directive == 'B' {
            row += row_index;
        }
        
        //println!("Applied directive. Row number now: {:b}", row);

        row_index >>= 1;
    };

    row
}

fn extract_seat_column(boarding_pass: &str) -> u8 {
    //println!("Boarding pass directives {}", boarding_pass);

    let column_directives = boarding_pass.chars().skip(7).take(3);

    let mut column = 0;
    let mut column_index = 4;
    for column_directive in column_directives {
        //println!("Got directive {}", column_directive);

        if column_directive == 'R' {
            column += column_index;
        }
        
        //println!("Applied directive. Column number now: {:b}", column);

        column_index >>= 1;
    };

    column
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_extract_seat_row() {
        assert_eq!(extract_seat_row("FBFBBFFRLR"), 44);
        assert_eq!(extract_seat_row("BFFFBBFRRR"), 70);
        assert_eq!(extract_seat_row("FFFBBBFRRR"), 14);
        assert_eq!(extract_seat_row("BBFFBBFRLL"), 102);
    }

    #[test]
    fn test_extract_seat_column() {
        assert_eq!(extract_seat_column("FBFBBFFRLR"), 5);
        assert_eq!(extract_seat_column("BFFFBBFRRR"), 7);
        assert_eq!(extract_seat_column("FFFBBBFRRR"), 7);
        assert_eq!(extract_seat_column("BBFFBBFRLL"), 4);
    }

    #[test]
    fn test_extract_seat_row_column() {
        assert_eq!(extract_seat_row_column("FBFBBFFRLR"), (44, 5));
        assert_eq!(extract_seat_row_column("BFFFBBFRRR"), (70, 7));
        assert_eq!(extract_seat_row_column("FFFBBBFRRR"), (14, 7));
        assert_eq!(extract_seat_row_column("BBFFBBFRLL"), (102, 4));
    }
}