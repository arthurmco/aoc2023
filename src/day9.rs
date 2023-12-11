#![allow(dead_code, unused_mut, unused_variables)]

use crate::util::{read_file_as_text, split_numbers_by_space};
use std::io::prelude::*;

struct OASISSequenceIter {
    derivatives: Vec<isize>
}

impl OASISSequenceIter {
    fn new(last: usize, derivatives: Vec<Vec<isize>>) -> Self {
        let mut derivatives: Vec<isize> = derivatives.into_iter().filter_map(|v| v.last().cloned()).chain([last as isize]).collect();
        derivatives.sort();
        
        eprintln!("c {:?}", derivatives);
        Self {
            derivatives
        }
    }

    fn generate_next_values(&self) -> Vec<isize> {
        let mut ret = vec![];

        for v in &self.derivatives {
            let last_solved = ret.last().unwrap_or(&0);

            ret.push(last_solved + v);
        }
        
        return ret;
    }
}

impl Iterator for OASISSequenceIter {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let next_derivatives = self.generate_next_values();
        eprintln!("nn {:?}", next_derivatives);
        
        let ret = next_derivatives.last().cloned();

        self.derivatives = next_derivatives;
        
        ret
    }
}

#[derive(Debug)]
struct OASISSequence {
    start: Vec<isize>,
}

impl OASISSequence {
    fn new(elements: impl Iterator<Item=isize>) -> Self {
        Self {
            start: elements.collect()
        }
    }

    fn compute_difference_value(&self, last: usize, second_to_last: usize) -> isize {
        (last as isize) - (second_to_last as isize)
    }
   
    fn generate_single_derivative(input: &Vec<isize>) -> Vec<isize> {
        input.windows(2).map(|w| w[1] - w[0]).collect()
    }

    fn calculate_derivatives(start: &Vec<isize>) -> Vec<Vec<isize>> {
        let mut derivatives: Vec<Vec<isize>> = Vec::new();

        loop {
            let last_sequence = derivatives.last().unwrap_or(start);
            derivatives.push(OASISSequence::generate_single_derivative(&last_sequence));

            assert!(derivatives.last().is_some());
            if *derivatives.last().unwrap().last().unwrap_or(&0) == 0 {
                break;
            }
        }

        derivatives
    }

    fn calculate_previous_derivatives(start: &Vec<isize>) -> Vec<Vec<isize>> {
        let mut derivatives: Vec<Vec<isize>> = Vec::new();

        loop {
            let last_sequence = derivatives.last().unwrap_or(start);
            derivatives.push(OASISSequence::generate_single_derivative(&last_sequence));

            if *derivatives.last().unwrap().first().unwrap_or(&0) == 0 {
                break;
            }
        }

        derivatives
    }
    
    fn generate_derivative_iter(&self) -> OASISSequenceIter {
        let derivatives = OASISSequence::calculate_derivatives(&self.start);
        OASISSequenceIter::new(self.start.last().cloned().unwrap() as usize, derivatives)
    }

    fn generate_derivative_previous_iter(&self) -> OASISSequenceIter {
        let mut start_rev = self.start.clone();
        start_rev.reverse();
        
        let derivatives = OASISSequence::calculate_previous_derivatives(&start_rev); 
        OASISSequenceIter::new(self.start.first().cloned().unwrap() as usize, derivatives)
    }
}



pub fn day9() {
    //let game_file = read_file_as_text("./inputs/day9test.txt").lines();
    let game_file = read_file_as_text("./inputs/day9real.txt").lines();
    let sequences = game_file.map(|s| OASISSequence::new(split_numbers_by_space(&s.unwrap()).into_iter())).inspect(|s| println!("s {:?}", s));

    let derivatives = sequences.map(|s| (s.generate_derivative_previous_iter())).map(|mut d| d.next().unwrap());
    let next_sequences = derivatives.inspect(|v| eprintln!("n {:?}", v)).collect::<Vec<isize>>();
           
    println!("{}", next_sequences.into_iter().sum::<isize>());
}

pub fn day9t1() {
    //let game_file = read_file_as_text("./inputs/day9test.txt").lines();
    let game_file = read_file_as_text("./inputs/day9real.txt").lines();
    let sequences = game_file.map(|s| OASISSequence::new(split_numbers_by_space(&s.unwrap()).into_iter())).inspect(|s| println!("s {:?}", s));

    let derivatives = sequences.map(|s| (s.generate_derivative_iter())).map(|mut d| d.next().unwrap());
    let next_sequences = derivatives.inspect(|v| eprintln!("n {:?}", v)).collect::<Vec<isize>>();
           
    println!("{}", next_sequences.into_iter().sum::<isize>());
}
