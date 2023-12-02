use core::fmt;
use std::error;
use std::{time::Instant, num::ParseIntError, error::Error};

use aoc2023::reader;

fn main() {
    if let Ok(lines) = reader::read_lines("./inputs/2.txt".to_owned()) {
        part_1(lines.clone());
        part_2(lines.clone());
    } else {
        println!("oopsie~!");
    }
}

#[derive(Debug, Clone)]
enum ParseError {
    Unknown,

    Parse(ParseIntError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::Unknown => write!(f, "failed to parse"),
            ParseError::Parse(..) => write!(f, "the provided string could not be parsed as an int"),
        }
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ParseError::Unknown => None,
            ParseError::Parse(ref e) => Some(e),
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> ParseError {
        ParseError::Parse(err)
    }
}

fn get_game_number(game: String) -> Result<usize, ParseError> {
    // Takes in a string of the form "Game X" and returns the integer X
    let number = game.trim().split_once(" ").ok_or(ParseError::Unknown)?.1;
    let parsed = parse_number(number);
    match parsed {
        Ok(p) => Ok(p),
        Err(e) => Err(ParseError::from(e)),
    }
}

fn parse_number(number: &str) -> Result<usize, ParseIntError> {
    number.parse::<usize>()
}

fn evaluate_game(game: String) -> bool {
    let game = game.replace(";", ",");
    let handfuls: Vec<_> = game.split(",").map(|x| x.trim()).collect();

    for handful in &handfuls {
        let (num, colour) = handful.split_once(" ").unwrap();
        let parsed_num = parse_number(num).expect("Failed to parse number");
        match colour {
            "blue" => {
                if parsed_num > 14 {
                    return false;
                }
            },
            "red" => {
                if parsed_num > 12 {
                    return false;
                }
            },
            "green" => {
                if parsed_num > 13 {
                    return false;
                }
            },
            _ => panic!("unexpected colour {}!", colour),
        }
    }
    return true;
}

fn part_1(lines: Vec<String>) {
    // small bag
    // cubes are either red green or blue
    // game:
    // - he hides a secret number of each color cube in the bag
    // - goal is to figure out info about no of cubes
    // - he reaches into bag & pulls out random handful each time
    // - puts cubes back in bag.
    // - game id, followed by :
    // - semicolon separated lists of handfuls
    // - comma separated numbers of cubes
    // GOAL: which games would have been possible if the bag contained ONLY 12 red, 13 gree, 14
    // blue cubes?
    // Add up the IDs of the possible games.
    
    // Notes
    // - game is only possible if it contains <= 12 red cubes, 13 green, 14 blue
    // - if ever we see > that number, we can instantly discount the game.
    // - data structure to store game info?
    // - no we don't need that yet. We can just look for /\d+\s(red|green|blue), parse number,
    // match against number needed.
    let start = Instant::now();
    println!("Part 1");

    let mut total = 0;
    for line in lines {
        let mut line_iter = line.split(":");
        let game = get_game_number(line_iter.next().unwrap().to_owned());
        if game.is_err() {
            println!("Error: {:?}", game);
            return;
        }
        let game = game.unwrap();
        let game_info = line_iter.next().unwrap().trim().to_owned();
        if evaluate_game(game_info) {
            total += game;
        }
    }

    println!("Total: {}", total);

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    println!();
}

fn evaluate_handful(handful: String) -> (usize, usize, usize) {
    // Returns the number of red, green, and blue cubes in a given handful of cubes
    let cubes = handful.split(",").map(|x| x.trim());
    let (mut red, mut green, mut blue) = (0, 0, 0);
    for cube in cubes {
        let (num, colour) = cube.split_once(" ").unwrap();
        let parsed_num = parse_number(num).unwrap();
        match colour {
            "red" => red += parsed_num,
            "green" => green += parsed_num,
            "blue" => blue += parsed_num,
            _ => panic!("unexpected colour {}!", colour),
        }
    }
    return (red, green, blue)
}

fn evaluate_game_2(game: String) -> usize {
    let (mut red, mut green, mut blue) = (0, 0, 0);
    let handfuls: Vec<_> = game.split(";").map(|x| x.trim()).collect();
    for handful in handfuls {
        let nums = evaluate_handful(handful.to_owned());
        if nums.0 > red { red = nums.0 };
        if nums.1 > green { green = nums.1 };
        if nums.2 > blue { blue = nums.2 };
    }
    return red * green * blue;
}

fn part_2(lines: Vec<String>) {
    let start = Instant::now();
    println!("Part 1");

    // What is the fewest number of cubes of each colour that could be in the bag?
    // - e.g. for 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // - the game could be played with as few as 4 red, 2 green, and 6 blue
    // - power = numbers of cubes multiplied together
    // therefore minimum power for game above would be 4 * 2 * 6 = 48
    //
    // GOAL: find minimum power for each game, sum all powers
    
    let mut total = 0;

    for line in lines {
        let game = line.split_once(":").unwrap().1.trim().to_owned();
        total += evaluate_game_2(game);
    }

    println!("Total: {}", total);

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    println!();
}
