use crate::aoc2024::input::day3_input_file::{INPUT_FILE_DAY3, INPUT_FILE_DAY3_PART2};
use crate::structures::day_trait::{Day, InputFile};
use regex::Regex;

pub struct Day3 {}
impl Day for Day3 {
    fn get_description(&self) -> String {
        String::from("Année 2024, Jour 3")
    }

    fn executer_partie1(&self, input: &str) -> i128 {
        let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let mut resultat: i128 = 0;
        for caps in regex.captures_iter(input) {
            let (_, [debut, fin]) = caps.extract();
            resultat += debut.parse::<i128>().unwrap() * fin.parse::<i128>().unwrap();
        }
        resultat
    }

    fn executer_partie2(&self, input: &str) -> i128 {
        let input_traite = input
            .split("don't()")
            .enumerate()
            .map(|(index, part)| {
                if index == 0 {
                    part.to_string()
                } else {
                    part.split("do()")
                        .skip(1)
                        .collect::<String>()
                }
            })
            .collect::<String>();

        self.executer_partie1(&input_traite)
    }
    fn recuperer_input_file(&self) -> InputFile {
        INPUT_FILE_DAY3
    }

    fn recuperer_input_file_partie2(&self) -> InputFile {
        INPUT_FILE_DAY3_PART2
    }
}

