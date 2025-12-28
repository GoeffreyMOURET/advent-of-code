use crate::aoc2024::input::day1_input_file::INPUT_FILE_DAY1;
use crate::structures::day_trait::{Day, InputFile};
use std::collections::HashMap;

pub struct Day1 {}
impl Day for Day1 {
    fn get_description(&self) -> String {
        String::from("Année 2024, Jour 1")
    }

    fn executer_partie1(&self, input: &str) -> i128 {
       let mut input = InputPart1::parse(input);
        input.ordonner_liste();
        input.liste1.iter()
            .zip(input.liste2.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }

    fn executer_partie2(&self, input: &str) -> i128 {
        let input = InputPart2::parse(input);
        input.liste1.iter()
            .map(|(&cle,&valeur)| {
                input.liste2.get(&cle).unwrap_or(&0) * valeur * cle
            })
            .sum()
    }
    fn recuperer_input_file(&self) -> InputFile {
        INPUT_FILE_DAY1
    }
}

struct InputPart2 {
    liste1: HashMap<i128, i128>,
    liste2: HashMap<i128, i128>,
}

impl InputPart2 {
    fn parse(input: &str) -> InputPart2 {
        let mut liste1: HashMap<i128, i128> = HashMap::new();
        let mut liste2: HashMap<i128, i128> = HashMap::new();

        for line in input.lines() {
            line.split_whitespace()
                .filter(|s| !s.is_empty())
                .filter_map(|s| s.parse::<i128>().ok())
                .enumerate()
                .for_each(|(index, valeur)| {
                    let liste = match index {
                        0 => &mut liste1,
                        1 => &mut liste2,
                        _ => panic!("Unexpected index {}", index),
                    };
                    *liste.entry(valeur).or_insert(0) += 1;
                })
        }

        InputPart2{liste1, liste2}
    }
}

struct InputPart1 {
    liste1: Vec<i128>,
    liste2: Vec<i128>,
}

impl InputPart1 {
    fn parse(input: &str) -> InputPart1 {
        let mut liste1: Vec<i128> = Vec::new();
        let mut liste2: Vec<i128> = Vec::new();

        for line in input.lines() {
            line.split_whitespace()
                .filter(|s| !s.is_empty())
                .filter_map(|s| s.parse::<i128>().ok())
                .enumerate()
                .for_each(|(index, valeur)| {
                    match index {
                        0 => liste1.push(valeur),
                        1 => liste2.push(valeur),
                        _ => panic!("Unexpected index {}", index),
                    };
                })
        }
        InputPart1 { liste1, liste2 }
    }

    fn ordonner_liste(&mut self) {
        self.liste1.sort();
        self.liste2.sort();
    }
}
