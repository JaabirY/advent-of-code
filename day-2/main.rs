// Bag contains unknown amount of red, green and blue cubes
// Each game, the Elf shows you the handful of cubes from the bag
// Which game would be possible if we knew the amount of R, G and B cubes?

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

// enum to store keys referring to colours of cubes
#[derive(Hash, Eq, PartialEq, Debug)]
enum Colour {
    Red,
    Green,
    Blue,
}

// Creates a Struct containing Game and their reveals
struct Game {
    game_number: u8,
    reveals: Vec<Reveals>,
}

// Creates a Struct containing Reveals and
struct Reveals {
    cubes: HashMap<Colour, u8>,
}

impl Reveals {
    fn new() -> Self {
        Reveals {
            cubes: HashMap::new(),
        }
    }

    fn add_cubes(&mut self, colour: Colour, quantity: u8){
        *self.cubes.entry(colour).or_insert(0) += quantity;
    }

    fn count(&self, colour: Colour) -> u8{
        *self.cubes.get(&colour).unwrap_or(&0)
    }
}

// This function returns the a Vector containing a list of Strings by line from the input.txt
fn read_lines() -> Vec<String> {
    let file = File::open("input.txt").expect("No such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}

// Parses the game data into easily accessable Game struct containing a vector of Games with variables of red, green and blue cube counts 
fn parse_game_data(input: &str) -> Game {
    let parts: Vec<&str> = input.split(":").collect();
    let game_number = parts[0].replace("Game ", "").parse::<u8>().unwrap();
    let reveals_data = parts[1].split("; ");

    let mut reveals = Vec::new();
    for reveal in reveals_data {
        let mut red_cube_count = 0;
        let mut green_cube_count = 0;
        let mut blue_cube_count = 0;

        for colour_data in reveal.split(", ") {
            let colour_parts: Vec<&str> = colour_data.split_whitespace().collect();
            let count = colour_parts[0].parse::<u8>().unwrap();
            match colour_parts[1] {
                "red" => red_cube_count = count,
                "green" => green_cube_count = count,
                "blue" => blue_cube_count = count,
                _ => {}
            }
        }
        let mut cubes = Reveals::new();
        cubes.add_cubes(Colour::Red, red_cube_count);
        cubes.add_cubes(Colour::Green, green_cube_count);
        cubes.add_cubes(Colour::Blue, blue_cube_count);
        reveals.push(cubes);
    }
    Game {
        game_number,
        reveals,
    }
}

// Compares each game with the actual number of cubes in a bag to see if it was possible or not
fn compare_game_with_actual_number_of_cubes(input_game: &Game) -> (Vec<String>,  bool) {
    // The bag contained 12 red cubes, 13 green cubes and 14 blue cubes 
    let mut actual_cubes = Reveals::new();
    actual_cubes.add_cubes(Colour::Red, 12);
    actual_cubes.add_cubes(Colour::Green, 13);
    actual_cubes.add_cubes(Colour::Blue, 14);

    let mut is_this_possible: bool = true;

    println!("Actual red cube count is {:?} ", actual_cubes.count(Colour::Red));

    // initisling the vector containing the messages
    let mut messages = Vec::new();
    messages.push(format!("Hello!"));

    for (idx, reveal) in input_game.reveals.iter().enumerate(){
        let reveal_number = idx+1;

        if 
    }
    // for (idx, reveal) in input_game.reveals.iter().enumerate(){
    //     let reveal_number = idx+1;
    //     let red_cube_count = reveal.red_cube_count;
    //     let green_cube_count = reveal.green_cube_count;
    //     let blue_cube_count = reveal.blue_cube_count;

    //     if red_cube_count > actual_red_cube_count {
    //         is_this_possible = false;
    //         messages.push(format!("the reveal number {} showed there were more red cubes than there were red cubes in the actual bag ({} > {})", reveal_number, red_cube_count, actual_red_cube_count));
    //     }
    //     if green_cube_count > actual_green_cube_count {
    //         is_this_possible = false;
    //         messages.push(format!("the reveal number {} showed there were more green cubes than there were green cubes in the actual bag ({} > {})", reveal_number, green_cube_count, actual_green_cube_count));
    //     }
    //     if blue_cube_count > actual_blue_cube_count {
    //         is_this_possible = false;
    //         messages.push(format!("the reveal number {} showed there were more blue cubes than there were blue cubes in the actual bag ({} > {})", reveal_number, blue_cube_count, actual_blue_cube_count));
    //     }
    // };
    return (messages, is_this_possible);
}

// fn how_fewer_cubes_to_make_the_game_possible(input_game: Game) -> {

// }

fn main(){
    let mut sum_of_game_ids_that_are_possible: u16 = 0;
    for line in read_lines(){
        let game = parse_game_data(&line);
        let (_messages, is_this_possible) = compare_game_with_actual_number_of_cubes(&game);
        if is_this_possible == false {
            // for message in messages{
            //     println!("In the game #{}, {}.", game.game_number, message);
            // }
            //println!("The number #{} is impossible!", game.game_number);
        } else {
            //println!("The number #{} is possible!", game.game_number);
            sum_of_game_ids_that_are_possible = sum_of_game_ids_that_are_possible + u16::from(game.game_number);
        }
    }
    //println!("{}", sum_of_game_ids_that_are_possible);

    // let mut cubes = Reveals::new();
    // cubes.add_cubes(Colour::Red, 1);
    // cubes.add_cubes(Colour::Green, 12);
    // cubes.add_cubes(Colour::Blue, 4);

    
    // let mut reveals = Vec::new();
    // reveals.push(cubes);

    // let game_1 =  Game {
    //     game_number: 1,
    //     reveals: reveals,
    // };

    // for reveal in game_1.reveals {
    //     for (colour, count) in reveal.cubes {
    //         println!("{:?}: {}", colour, count);
    //     }
    // }

}