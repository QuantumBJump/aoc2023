use std::time::Instant;

use aoc2023::reader;

fn main() {
    if let Ok(lines) = reader::read_lines("./inputs/3.txt".to_owned()) {
        part_1(lines.clone());
        part_2(lines.clone());
    } else {
        println!("oopsie~!");
    }
}

#[derive(Clone, Debug)]
struct Coordinate {
    x: isize,
    y: isize,
}

#[derive(Clone, Debug)]
struct BoundingBox {
    start: Coordinate,
    end: Coordinate,
}

#[derive(Clone, Debug)]
struct Number {
    value: usize,
    bounding_box: BoundingBox,
}

#[derive(Clone)]
struct StateMachine {
    coord: Coordinate,
    current_number: Option<Number>,
}


fn part_1(lines: Vec<String>) {
    let start = Instant::now();
    println!("Part 1");

    // bounding box
    // - find number, create bounding box w/ extra padding
    //
    // work out bounding box for each number
    //   123 at x, y => bounding_box(x-1, y-1, x+3, y+1)
    // scaling: numbers * symbols

    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Coordinate> = Vec::new();
    let mut chompy = StateMachine{
        coord: Coordinate{
            x: 0,
            y: 0,
        },
        current_number: None,
    };
    for (y, line) in lines.into_iter().enumerate() {
        chompy.coord.x = 0;
        chompy.coord.y = y as isize;
        // True == there is a number
        // False == no number
        for char in line.chars() {

            let num = chompy.current_number.clone();
            match (char, num) {
                ('.', Some(num)) => {
                    // push into numbers, clear number, advance
                    numbers.push(num);
                    chompy.current_number = None;
                },
                ('.', None) => {},
                (c, None) if c.is_ascii_digit() => {
                    // number.start = x-1, y-1
                    // number.end = x+1, y+1
                    // number.value = char.parse()
                    let number = Number{
                        value: c.to_digit(10).unwrap() as usize,
                        bounding_box: BoundingBox{
                            start: Coordinate{
                                x: chompy.coord.x-1,
                                y: chompy.coord.y-1,
                            },
                            end: Coordinate {
                                x: chompy.coord.x+1,
                                y: chompy.coord.y+1,
                            }
                        }
                    };
                    chompy.current_number = Some(number);
                },
                (c, Some(num)) if c.is_ascii_digit() => {
                    let mut new_num = num.clone();
                    new_num.bounding_box.end.x += 1;
                    new_num.value *= 10;
                    new_num.value += c.to_digit(10).unwrap() as usize;
                    chompy.current_number = Some(new_num);
                },
                (_, num) => {
                    if num.is_some() {
                        numbers.push(num.unwrap());
                        chompy.current_number = None;
                    }
                    let coord = Coordinate{
                        x: chompy.coord.x,
                        y: chompy.coord.y,
                    };
                    symbols.push(coord);
                },
            }
            chompy.coord.x += 1;
        }
        if chompy.current_number.is_some() {
            numbers.push(chompy.current_number.clone().unwrap());
            chompy.current_number = None;
        }
    }

    let mut total = 0;
    for num in numbers {
        for symbol in symbols.clone() {
            if symbol.x >= num.bounding_box.start.x
                && symbol.x <= num.bounding_box.end.x
                && symbol.y >= num.bounding_box.start.y
                && symbol.y <= num.bounding_box.end.y {
                    total += num.value;
                    break;
                }
        }
    }
    println!("{}", total);

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    println!();
}

#[allow(dead_code, unused_variables)]
fn part_2(lines: Vec<String>) {
    let start = Instant::now();
    println!("Part 1");

    // Find gears (* symbols which are adjacent to EXACTLY TWO numbers)
    // gear ratio is the product of the two numbers
    // sum all gear ratios

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    println!();
}
