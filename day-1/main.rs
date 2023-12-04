use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::{Regex, Captures};

// This function returns the a Vector containing a list of Strings by line from the input.txt
fn read_lines() -> Vec<String> {
    let file = File::open("input.txt").expect("No such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}

fn replace_word_with_number(input: &String) -> String {
    let number_words = HashMap::from([
        ("zero", "z0o"),
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ]);

    let pattern = number_words.keys()
        .map(|&word| word.to_string())
        .collect::<Vec<String>>()
        .join("|");

    let re = Regex::new(&pattern).unwrap();
    
    re.replace_all(&input, |caps: &Captures| {
        let word = caps.get(0).unwrap().as_str();
        number_words.get(word).map_or_else(||word.to_string(), |&number|
    number.to_string())
    }).to_string()
}

// This function obtains the calibration value based on the String. Calibration value is produced from taking the first digit and last digit (they can be the same digit) and then putting these two digits together
fn get_calibration_value(input: &String) -> u16 {
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
    return calibration_value.parse::<u16>().unwrap();
}

fn main(){
    let mut sum_of_all_calibration_values = 0; // Starts the sum at 0 and makes it mutable so it can be incremented

    for line in read_lines(){ // For each String line, convert word numbers into actual numbers and then gets the calibration value and then puts them in the sum variable
        let clean_line = replace_word_with_number(&line);
        let clean_line_x2 = replace_word_with_number(&clean_line); // This is done twice due to the input data having overlapping number words - first pass replaces non overlapping words and then second pass replaces the overlapping words
        let calibration_value = get_calibration_value(&clean_line_x2);
        sum_of_all_calibration_values = sum_of_all_calibration_values + calibration_value;
    };
    println!("The sum of all calibration values is: {}", sum_of_all_calibration_values);
}