use crate::aoc2025::input::day3_input_file::INPUT_FILE_DAY3;
use crate::structures::day_trait::{Day, InputFile};

pub struct Day3 {}
impl Day for Day3 {
    fn get_description(&self) -> String {
        String::from("Année 2025, Jour 3")
    }

    fn executer_partie1(&self, input: &str) -> i128 {
        input.split("\n")
            .map(Banck::parse)
            .map(|banck: Banck| { banck.calculer_output(2)})
            .sum()
    }

    fn executer_partie2(&self, input: &str) -> i128 {
        input.split("\n")
            .map(Banck::parse)
            .map(|banck: Banck| { banck.calculer_output(12) })
            .sum()
    }
    fn recuperer_input_file(&self) -> InputFile {
        INPUT_FILE_DAY3
    }

}

struct Banck {
    joltages: Vec<u32>,
}

impl Banck {
    fn parse(ligne: &str) -> Banck {
        Banck {
            joltages: ligne.chars()
                .filter_map(|c| c.to_digit(10))
                .collect()
        }
    }

    fn calculer_output(&self, nb_digit: usize) -> i128 {
        let mut resultat: Vec<u32> = Vec::with_capacity(nb_digit);
        let mut index_chiffre_suivant: usize = 0;

        for i in 0..nb_digit {
            let chiffre = &self.joltages[index_chiffre_suivant..&self.joltages.len() - nb_digit + i + 1]
                .iter().max().unwrap();
            resultat.push(**chiffre);
            let index_chiffre = &self.joltages[..&self.joltages.len() - nb_digit + i + 1]
                .iter().enumerate()
                .find(|(index, joltage)| { *index >= index_chiffre_suivant && joltage == chiffre })
                .map(|(index, _)| index).unwrap();
            index_chiffre_suivant = *index_chiffre + 1;
        }

        resultat.iter().rev().enumerate()
            .map(|(index, r)| { (*r as i128) * 10_i128.pow(index as u32) })
            .sum()
    }
}

