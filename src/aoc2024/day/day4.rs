use crate::aoc2024::input::day4_input_file::INPUT_FILE_DAY4;
use crate::structures::day_trait::{Day, InputFile};
use std::collections::{HashMap, HashSet};

type Dictionnaire = HashMap<char, HashSet<(isize, isize)>>;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1,  0),
    ( 1,  0),
    ( 0, -1),
    ( 0,  1),
    (-1, -1),
    (-1,  1),
    ( 1, -1),
    ( 1,  1),
];

const POSITIONS_M: [[(isize, isize); 2]; 4] = [
    [(-1,  -1), (-1,  1)],
    [(-1,  -1), (1,  -1)],
    [(1,  1), (-1,  1)],
    [(1,  1), (1,  -1)],
];

pub struct Day4 {}
impl Day for Day4 {
    fn get_description(&self) -> String {
        String::from("Année 2024, Jour 4")
    }

    fn executer_partie1(&self, input: &str) -> i128 {
        let mut lettres: Dictionnaire = HashMap::new();

        input.lines().enumerate().for_each(|(i, ligne)| {
            ligne.chars().enumerate()
                .filter(|(_, lettre)| { *lettre == 'X' || *lettre == 'A' || *lettre == 'M' || *lettre == 'S' })
                .for_each(|(j, lettre)| {
                    lettres.entry(lettre).or_insert(HashSet::new()).insert((i as isize, j as isize));
                })
        });

        let mut resultat: i128 = 0;

        for &direction in DIRECTIONS.iter() {
            resultat += lettres.get(&'X').unwrap().iter()
                .filter(|(x, y)| {
                    let emplacement_m = (x + direction.0, y + direction.1);
                    let emplacement_a = (x + 2 * direction.0, y + 2 * direction.1);
                    let emplacement_s = (x + 3 * direction.0, y + 3 * direction.1);
                    lettres.get(&'M').unwrap().iter().find(|coord_m| { **coord_m == emplacement_m } ).is_some()
                        && lettres.get(&'A').unwrap().iter().find(|coord_a| { **coord_a == emplacement_a } ).is_some()
                        && lettres.get(&'S').unwrap().iter().find(|coord_s| { **coord_s == emplacement_s } ).is_some()
                }).count() as i128;
        }
        resultat
    }

    fn executer_partie2(&self, input: &str) -> i128 {
        let mut lettres: Dictionnaire = HashMap::new();

        input.lines().enumerate().for_each(|(i, ligne)| {
            ligne.chars().enumerate()
                .filter(|(_, lettre)| { *lettre == 'A' || *lettre == 'M' || *lettre == 'S' })
                .for_each(|(j, lettre)| {
                    lettres.entry(lettre).or_insert(HashSet::new()).insert((i as isize, j as isize));
                })
        });

        let mut resultat: i128 = 0;

        for &direction in POSITIONS_M.iter() {
            resultat += lettres.get(&'A').unwrap().iter()
                .filter(|(x, y)| {
                    let [direction1, direction2] = direction;
                    let emplacement_m1 = (x + direction1.0, y + direction1.1);
                    let emplacement_s1 = (x - direction1.0, y - direction1.1);
                    let emplacement_m2 = (x + direction2.0, y + direction2.1);
                    let emplacement_s2 = (x - direction2.0, y - direction2.1);
                    lettres.get(&'M').unwrap().iter().find(|coord_m| { **coord_m == emplacement_m1 } ).is_some()
                        && lettres.get(&'M').unwrap().iter().find(|coord_m| { **coord_m == emplacement_m2 } ).is_some()
                        && lettres.get(&'S').unwrap().iter().find(|coord_s| { **coord_s == emplacement_s1 } ).is_some()
                        && lettres.get(&'S').unwrap().iter().find(|coord_s| { **coord_s == emplacement_s2 } ).is_some()
                }).count() as i128;
        }
        resultat
    }


    fn recuperer_input_file(&self) -> InputFile {
        INPUT_FILE_DAY4
    }

}


