use std::collections::{HashSet, HashMap};
use std::time::Instant;

use aoc2023::reader;

fn main() {
    if let Ok(lines) = reader::read_lines("./inputs/4.txt".to_owned()) {
        part_1(lines.clone());
        part_2(lines.clone());
    } else {
        println!("oopsie~!");
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Scratchcard {
    winning: HashSet<usize>,
    actual: Vec<usize>,
    copies: usize,
}

fn read_scratchcard(card: String) -> (usize, Scratchcard) {
    // Remove the "Game X:" from the front
    let (mut game, numbers) = card.split_once(':').unwrap();
    game = game.strip_prefix("Card ").unwrap().trim();
    let game = game.parse::<usize>().unwrap();
    let (winning_nums, actual_nums) = numbers.split_once('|').unwrap();

    // Read winning numbers
    let mut winning = HashSet::new();
    for num in read_numlist(winning_nums.trim().to_string()) {
        winning.insert(num);
    }
    // Read actual numbers
    let actual = read_numlist(actual_nums.trim().to_string());

    (game, Scratchcard { winning, actual, copies: 1 })
}

fn read_numlist(list: String) -> Vec<usize> {
    list.split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect()
}

impl Scratchcard {
    fn count_wins(&self) -> usize {
        let mut matches = 0;
        for num in &self.actual {
            if self.winning.contains(&num) {
                matches += 1;
            }
        }
        matches
    }

    fn calculate_score(&self) -> usize {
        match self.count_wins() {
            0 => 0,
            1 => 1,
            _ => 1 << self.count_wins() - 1,
        }
    }

    fn set_copies(&mut self, num: usize) {
        self.copies = num;
    }

    fn add_copies(&mut self, num: usize) {
        self.copies += num;
    }
}

fn part_1(lines: Vec<String>) {
    let start = Instant::now();
    println!("Part 1");

    // Scratch cards
    // Card X: <numbers> | <numbers>
    // first set of numbers is winning numbers
    // second set are numbers you have
    // first match == 1 point
    // each subsequent match doubles the point value of the card
    // GOAL: sum each card's point totals
    let mut total = 0;
    for line in lines {
        let (_, sc) = read_scratchcard(line);
        total += sc.calculate_score();
    }

    println!("Total: {}", total);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    println!();
}

fn part_2(lines: Vec<String>) {
    let start = Instant::now();
    println!("Part 1");

    let mut total = 0;

    let mut cards: Vec<Scratchcard> = Vec::new();
    for line in lines {
        let (_, mut v) = read_scratchcard(line);
        v.set_copies(1);
        cards.push(v);
    }

    let len = cards.len();
    for idx in 0..cards.len() {
        let card = cards[idx].clone();
        total += card.copies;
        let wins = card.count_wins();
        for i in 1..=wins {
            if idx+i >= len {
                break;
            }
            cards[idx+i].add_copies(card.copies);
        }
    }


    println!("Total: {}", total);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_score() {
        let card1 = Scratchcard {
            winning: HashSet::from([41, 48, 83, 86, 17]),
            actual: vec![83, 86, 6, 31, 17, 9, 48, 53],
            copies: 0,
        };
        let card2 = Scratchcard {
            winning: HashSet::from([13, 32, 20, 16, 61]),
            actual: vec![61, 30, 68, 82, 17, 32, 24, 19],
            copies: 0,
        };
        let card3 = Scratchcard {
            winning: HashSet::from([41, 92, 73, 84, 69]),
            actual: vec![59, 84, 76, 51, 58, 5, 53, 83],
            copies: 0,
        };

        assert_eq!(8, card1.calculate_score());
        assert_eq!(2, card2.calculate_score());
        assert_eq!(1, card3.calculate_score());
    }

    #[test]
    fn test_read_scratchcard() {
        let card1 = "Card 1: 41 48 83 86 17 | 83 86 6 31 17 9 48 53".to_string();
        let sc1 = Scratchcard {
            winning: HashSet::from([41, 48, 83, 86, 17]),
            actual: vec![83, 86, 6, 31, 17, 9, 48, 53],
            copies: 0,
        };

        assert_eq!(read_scratchcard(card1), (1, sc1));
    }

    #[test]
    fn test_read_numlist() {
        let numlist = "41 48 83 86 17".to_string();

        assert_eq!(read_numlist(numlist), vec![41, 48, 83, 86, 17]);
    }
}
