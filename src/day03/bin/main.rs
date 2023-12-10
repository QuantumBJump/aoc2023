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

#[derive(Clone, Debug)]
struct Symbol {
    ch: char,
    coord: Coordinate,
}

#[derive(Clone)]
struct StateMachine {
    coord: Coordinate,
    current_number: Option<Number>,
}

impl StateMachine {
    fn consume_number(&mut self, ch: char) -> (Option<Number>, Option<Symbol>) {
        match (ch, self.current_number.clone()) {
            ('.', Some(num)) => {
                // End of a number; update.
                self.current_number = None;
                (Some(num), None)
            },
            ('.', None) => (None, None), // Not in a number
            (c, None) if c.is_ascii_digit() => {
                // Start of a number
                let number = Number{
                    value: c.to_digit(10).unwrap() as usize,
                    bounding_box: BoundingBox{
                        start: Coordinate{
                            x: self.coord.x-1,
                            y: self.coord.y-1,
                        },
                        end: Coordinate {
                            x: self.coord.x+1,
                            y: self.coord.y+1,
                        }
                    }
                };
                self.current_number = Some(number);
                (None, None)
            },
            (c, Some(num)) if c.is_ascii_digit() => {
                // Continuing number
                let mut new_num = num.clone();
                new_num.bounding_box.end.x += 1;
                new_num.value *= 10;
                new_num.value += c.to_digit(10).unwrap() as usize;
                self.current_number = Some(new_num);
                (None, None)
            },
            (_, num) => {
                let symb = Symbol {
                    ch,
                    coord: Coordinate {
                        x: self.coord.x,
                        y: self.coord.y,
                    }
                };
                if num.is_some() {
                    self.current_number = None;
                    (Some(num.unwrap()), Some(symb))
                } else {
                    (None, Some(symb))
                }
            },

        }
    }
}

fn collision(bb: &BoundingBox, coord: &Coordinate) -> bool {
    coord.x >= bb.start.x &&
        coord.x <= bb.end.x &&
        coord.y >= bb.start.y &&
        coord.y <= bb.end.y
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
    let mut symbols: Vec<Symbol> = Vec::new();
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
        for char in line.chars() {

            let (num, symb) = chompy.consume_number(char);
            if let Some(num) = num {
                numbers.push(num);
            }
            if let Some(symb) = symb {
                symbols.push(symb);
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
            if collision(&num.bounding_box, &symbol.coord) {
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
    
    // chomp
    // go through symbols
    // if the symbol isn't a *, skip
    // if it is, check each number

    let mut symbols: Vec<Symbol> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();
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
        for char in line.chars() {
            let (num, symb) = chompy.consume_number(char);
            if let Some(num) = num {
                numbers.push(num);
            }
            if let Some(symb) = symb {
                symbols.push(symb);
            }
            chompy.coord.x += 1;
        }
        if chompy.current_number.is_some() {
            numbers.push(chompy.current_number.clone().unwrap());
            chompy.current_number = None;
        }
    }

    let mut total = 0;

    'symb: for symb in symbols {
        if symb.ch != '*' {
            continue;
        }
        let mut adjacent: Vec<Number> = Vec::new();

        for num in numbers.clone() {
            if collision(&num.bounding_box, &symb.coord) {
                adjacent.push(num);
            }
            if adjacent.len() >= 3 {
                continue 'symb;
            }
        }
        if adjacent.len() == 2 {
            total += adjacent[0].value * adjacent[1].value;
        }
    }
    println!("{}", total);

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    println!();
}
