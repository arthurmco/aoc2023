#[allow(dead_code)]

use crate::util::{read_file_as_text};
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Card {
    pub number: usize,
    winning: Vec<u32>,
    numbers: Vec<u32>
}

impl Card {
    fn get_card_number(line: &str) -> usize {
        // Card<SPACE><NUMBER>
        line[4..].trim().parse().unwrap()
    }

    fn get_won_number_count(&self) -> usize {
        self.numbers.iter().filter(|n| self.winning.contains(n)).count()
    }
    
    fn get_won_cards(&self, total_cards: usize, won_numbers: usize) -> Vec<usize> {
        (1..=won_numbers).map(|n| n+self.number).filter(|&n| n <= total_cards).collect()
    }

    fn get_point_count(won_numbers: usize) -> usize {
        let base: usize = 2;
        match won_numbers {
            0 => 0,
            x => base.pow((x-1) as u32)
        }
    }

    fn parse_numbers(line: &str) -> Vec<u32>{
        line.split(' ').filter_map(|num| num.trim().parse().ok()).collect()
    }

    fn get_winning_and_received_numbers(line: &str) -> (Vec<u32>, Vec<u32>) {
        let mut parts = line.split('|');
        let winning = Card::parse_numbers(parts.next().unwrap());
        let received = Card::parse_numbers(parts.next().unwrap());

        (winning, received)        
    }
    
    fn from_line(line: &str) -> Self {
        let mut parts = line.split(':');
        let number = Card::get_card_number(parts.next().unwrap());
        let (winning, numbers) = Card::get_winning_and_received_numbers(parts.next().unwrap());
        
        Card {
            number,
            winning,
            numbers
        }
    }
}


pub fn _day4t1() {
    let game_file = read_file_as_text("./inputs/day4real.txt");
    //let game_file = read_file_as_text("./inputs/day4test.txt");
    let card_total: usize = game_file.lines().map(|line| Card::from_line(&line.unwrap())).inspect(|c| print!("Cards: {:?}", c)).map(|c| Card::get_point_count(c.get_won_number_count())).inspect(|c| println!(" count {}", c)).sum();

    println!("{}", card_total);
}

fn process_card_retrieve_won_cards(total_cards: usize, cards_received: usize, c: &Card) -> Vec<(usize, usize)> {
    let card_count = c.get_won_number_count();
    let card_numbers = c.get_won_cards(total_cards, card_count);

    card_numbers.iter().map(|cnum| (*cnum, cards_received)).collect()
}

fn process_all_cards(cards: &[Card]) -> HashMap<usize, usize> {
    let mut count_list : HashMap<usize, usize> = cards.iter().map(|c| (c.number, 1)).collect();
    
    for c in cards {
        let card_count = *count_list.get(&c.number).unwrap();
        process_card_retrieve_won_cards(cards.len(), card_count, c).into_iter().for_each(|(cnum, ccount)| {
            count_list.entry(cnum).and_modify(|v| *v += ccount);
        });
    }

    count_list
}


pub fn day4() {
    let game_file = read_file_as_text("./inputs/day4real.txt");
    //let game_file = read_file_as_text("./inputs/day4test.txt");
    let cards: Vec<Card> = game_file.lines().map(|line| Card::from_line(&line.unwrap())).collect();
    let card_counts = process_all_cards(&cards);
    let card_sum: usize = card_counts.values().sum();
    
    println!("{:?}", card_counts);
    println!("{}", card_sum);
}
