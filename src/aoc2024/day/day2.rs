use crate::aoc2024::input::day2_input_file::INPUT_FILE_DAY2;
use crate::structures::day_trait::{Day, InputFile};


pub struct Day2 {}
impl Day for Day2 {
    fn get_description(&self) -> String {
        String::from("Année 2024, Jour 2")
    }

    fn executer_partie1(&self, input: &str) -> i128 {
       Data::parse(input).rapports.iter()
           .filter(|&r| Rapport::est_valide(&r.valeur))
           .count() as i128
    }

    fn executer_partie2(&self, input: &str) -> i128 {
        Data::parse(input).rapports.iter()
            .filter(|&r| r.sous_rapports.iter().any(Rapport::est_valide))
            .count() as i128
    }
    fn recuperer_input_file(&self) -> InputFile {
        INPUT_FILE_DAY2
    }
}

struct Data {
    rapports: Vec<Rapport>
}

impl Data {
    fn parse(input: &str) -> Data {
        Data {
            rapports: input.lines().map(Rapport::parse).collect()
        }
    }
}

#[derive(PartialEq, Debug)]
enum Ordre {
    CROISSANT,
    DECROISSANT
}
struct Rapport {
    valeur: Vec<i128>,
    sous_rapports: Vec<Vec<i128>>,
}

impl Rapport {
    fn parse(input: &str) -> Rapport {
        let valeur: Vec<i128> = input.split_whitespace()
            .filter_map(|str| { str::parse::<i128>(str).ok() })
            .collect();
        let sous_rapports: Vec<Vec<i128>> = vec![valeur.clone(); valeur.len()].into_iter().enumerate()
            .map(|(index, mut sr)| {
                sr.remove(index);
                sr
            }).collect();

        Rapport { valeur, sous_rapports }
    }

    fn est_valide(valeur: &Vec<i128>) -> bool {
        debug_assert!(valeur.len() >= 2, "Input non prévu !");
        let ordre: Ordre = if valeur.first().unwrap() > valeur.last().unwrap() {
            Ordre::DECROISSANT
        } else {
            Ordre::CROISSANT
        };
        valeur.iter()
            .skip(if ordre == Ordre::DECROISSANT { 1 } else { 0 })
            .zip(valeur.iter().cycle().skip(if ordre == Ordre::CROISSANT { 1 } else { 0 }))
            .take(valeur.len() - 1)
            .all(|(&v1, &v2)| { v1 < v2 && (v1 - v2).abs() <= 3 })

    }
}