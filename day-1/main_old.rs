use std::fs::File;
use std::io::{self, BufRead};

// This function returns the a Vector containing a list of Strings by line from the input.txt
fn read_lines() -> Vec<String> {
    let file = File::open("input.txt").expect("No such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}

// This function obtains the calibration value based on the String. Calibration value is produced from taking the first digit and last digit (they can be the same digit) and then putting these two digits together
fn get_calibration_value(input: String) -> i32 {
    let digits: Vec<char> = input.chars() // Converts the input String into chars and then filters them into digits (base-10)
        .filter(|c| c.is_digit(10))
        .collect();

    let first_digit = match digits.first(){ // Obtains the first digit and checks if it's not empty otherwise it panics
        Some(first) => *first,
        None => panic!("Unexpected error - no first digit found in this string: {}", input)
    };

    let last_digit = match digits.last(){ // Obtains the last digit and checks if it's not empty otherwise it uses the first digit (as it is also the last digit too)
        Some(last) => *last,
        None => {
            first_digit
        },
    };
    let mut calibration_value = first_digit.to_string(); // Some shuffling about with data types (big bugbear of mine in Rust)
    calibration_value.push(last_digit);
    return calibration_value.parse::<i32>().unwrap();
}

fn main(){
    let mut sum_of_all_calibration_values = 0; // Starts the sum at 0 and makes it mutable so it can be incremented. 

    for line in read_lines(){ // For each String line, get the calibration value and then puts them in the sum variable.
        let calibration_value = get_calibration_value(line);
        sum_of_all_calibration_values = sum_of_all_calibration_values + calibration_value
    }

    println!("The sum of all calibration values is: {}", sum_of_all_calibration_values);
}