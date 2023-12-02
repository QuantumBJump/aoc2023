use std::time::Instant;

use aoc2023::reader;

fn main() {
    if let Ok(lines) = reader::read_lines("./inputs/1.txt".to_owned()) {
        part_1(lines.clone());
        part_2(lines.clone());
    } else {
        println!("oops!");
    }
}

fn part_1(lines: Vec<String>) {
    let start = Instant::now();
    println!("Part 1");
    let mut total: usize = 0;
    for line in lines {
        let mut calibration: usize = 0;
        let chars = line.chars().into_iter();
        for n in chars.clone() {
            if n.is_ascii_digit() {
                let digit = n.to_digit(10).unwrap() as usize * 10;
                // println!("digit 1: {}", digit);
                calibration += digit;
                break;
            }
        }
        for n in chars.rev() {
            if n.is_ascii_digit() {
                let digit = n.to_digit(10).unwrap() as usize;
                // println!("digit 2: {}", digit);
                calibration += digit;
                break;
            }
        }
        // println!("\tcalibration: {}", calibration);
        total += calibration;
    }
    println!("{}", total);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    println!();
}
const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
];

fn part_2(lines: Vec<String>) {
    let start = Instant::now();
    println!("Part 2");

    let mut total: usize = 0;
    for line in lines {
        let mut calibration: usize = 0;
        let mut idx = 0;
        let linechars: Vec<u8> = line.bytes().collect();
        // println!("line");
        while idx < line.len() {
            let mut found = false;
            for (num, val) in NUMBERS.iter().zip(1..) {
                if linechars[idx..].starts_with(num.as_bytes()) {
                    // println!("\tdigit 1: {}", val * 10); 
                    calibration += val * 10;
                    found = true;
                    break;
                }
                else if linechars[idx].is_ascii_digit() {
                    let digit = (linechars[idx] - b'0') as usize * 10;
                    // println!("\tdigit 1: {}", digit); 
                    calibration += digit;
                    found = true;
                    break;
                }
            }
            if found { break; }
            idx += 1;
        }

        idx = line.len();
        loop {
            let mut found = false;
            for (num, val) in NUMBERS.iter().zip(1..) {
                if linechars[..idx].ends_with(num.as_bytes()) {
                    calibration += val;
                    found = true;
                    break;
                }
                else if linechars[idx-1].is_ascii_digit() {
                    let digit = (linechars[idx-1] - b'0') as usize;
                    calibration += digit;
                    found = true;
                    break;
                }
            }
            if found { break; }

            if idx == 0 {
                break;
            }
            idx -= 1;
        }
        total += calibration;
    }

    let duration = start.elapsed();
    println!("{}", total);
    println!("Time elapsed: {:?}", duration);
    println!();
}
