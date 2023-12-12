#![allow(dead_code, unused_mut, unused_variables)]

use crate::util::read_file_as_text;
use std::cmp::{Ordering, PartialOrd};
use std::io::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    A = 14,
    K = 13,
    Q = 12,
    T = 11,
    C9 = 9,
    C8 = 8,
    C7 = 7,
    C6 = 6,
    C5 = 5,
    C4 = 4,
    C3 = 3,
    C2 = 2,
    J = 1,
}

impl TryFrom<char> for Card {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::T),
            '2' => Ok(Card::C2),
            '3' => Ok(Card::C3),
            '4' => Ok(Card::C4),
            '5' => Ok(Card::C5),
            '6' => Ok(Card::C6),
            '7' => Ok(Card::C7),
            '8' => Ok(Card::C8),
            '9' => Ok(Card::C9),
            _ => Err(format!("Invalid char for card: {}", value)),
        }
    }
}

type CardHand = (Card, Card, Card, Card, Card);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

fn get_card_occurences_in_hand(hand: &CardHand) -> Vec<(usize, Card)> {
    let (h1, h2, h3, h4, h5) = hand;

    let mut cards = [h1, h2, h3, h4, h5];
    cards.sort();

    cards.into_iter().fold(vec![], |mut acc, card| {
        let index = acc
            .iter()
            .enumerate()
            .find(|(_, (count, c))| c == card)
            .map(|(index, _)| index);

        match index {
            Some(i) => {
                let (count, _) = acc[i];
                acc[i] = (count + 1, *card);
                acc
            }
            None => {
                acc.push((1, *card));
                acc
            }
        }
    })
}

fn fix_joker(mut occurences: Vec<(usize, Card)>) -> Vec<(usize, Card)> {
    // make counts attributed to the joker go to whatever card has the biggest occurence;

    let joker_data = occurences.iter().find(|(_, c)| *c == Card::J);

    match joker_data {
        None => occurences,
        Some((freq, _)) => {
            if occurences.len() == 1 {
                occurences
            } else {
                let biggest = occurences
                    .iter()
                    .enumerate()
                    .filter(|(_, (_, c))| *c != Card::J)
                    .max_by(|(_, (count_a, _)), (_, (count_b, _))| count_a.cmp(count_b))
                    .unwrap();

                let (b_index, (b_count, b_card)) = biggest;
                occurences[b_index] = (b_count + freq, *b_card);
                occurences
                    .into_iter()
                    .filter(|(_, c)| *c != Card::J)
                    .collect()
            }
        }
    }
}

fn get_hand_type(hand: &CardHand) -> HandType {
    let occurences = fix_joker(get_card_occurences_in_hand(hand));

    assert!(occurences.len() <= 5);

    match occurences.len() {
        1 => HandType::FiveOfKind,
        2 => {
            let card1 = occurences[0];
            let card2 = occurences[1];

            if (card1.0 == 4 && card2.0 == 1) || (card1.0 == 1 && card2.0 == 4) {
                HandType::FourOfKind
            } else {
                HandType::FullHouse
            }
        }
        3 => {
            let card1 = occurences[0];
            let card2 = occurences[1];
            let card3 = occurences[2];

            if card1.0 == 3 || card2.0 == 3 || card3.0 == 3 {
                HandType::ThreeKind
            } else {
                HandType::TwoPair
            }
        }
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!("It is forbidden to have >5 different cards in a hand"),
    }
}

fn order_card_hand(left: &CardHand, right: &CardHand) -> Ordering {
    match get_hand_type(left).cmp(&get_hand_type(right)) {
        Ordering::Equal => left.cmp(right),
        x => x,
    }
}

type CardLine = (CardHand, usize);

fn line_to_hand(line: &str) -> CardHand {
    let mut chariter = line.chars().take(5).filter_map(|c| Card::try_from(c).ok());

    (
        chariter.next().unwrap(),
        chariter.next().unwrap(),
        chariter.next().unwrap(),
        chariter.next().unwrap(),
        chariter.next().unwrap(),
    )
}

fn line_to_cardline(line: &str) -> CardLine {
    let mut parts = line.split(' ');

    (
        line_to_hand(parts.next().unwrap()),
        parts.next().unwrap().trim().parse::<usize>().unwrap(),
    )
}

pub fn day7() {
    let game_file = read_file_as_text("./inputs/day7real.txt").lines();
    //let game_file = read_file_as_text("./inputs/day7test.txt").lines();
    let mut hand_bids: Vec<CardLine> = game_file.map(|l| line_to_cardline(&l.unwrap())).collect();

    println!("{:?}", hand_bids);

    hand_bids.sort_by(|(a, _), (b, _)| order_card_hand(a, b));
    println!("{:?}", hand_bids);

    let winnings: usize = hand_bids
        .iter()
        .enumerate()
        .map(|(index, (_, bid))| (index + 1) * bid)
        .sum();
    //println!("{:?}", winnings.collect::<Vec<usize>>());

    println!("\n{}", winnings)
}
