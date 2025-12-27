use crate::aoc2025::day::day1::dial::Roue;
use crate::aoc2025::input::day1_input_file::INPUT_FILE_DAY1;
use crate::structures::day_trait::{Day, InputFile};

pub struct Day1 {}
impl Day for Day1 {
    fn get_description(&self) -> String {
        String::from("Année 2025, Jour 1")
    }

    fn executer_partie1(&self, input: &str) -> i128 {
        Self::calculer_resultat(input).get_resultat_partie1()
    }

    fn executer_partie2(&self, input: &str) -> i128 {
        Self::calculer_resultat(input).get_resultat_partie2()
    }
    fn recuperer_input_file(&self) -> InputFile {
        INPUT_FILE_DAY1
    }
}

impl Day1 {
    fn calculer_resultat(input: &str) -> Roue {
        let mut roue = dial::Roue::new();
        input.lines()
            .filter_map(|line| { Rotation::parse(line).ok() })
            .for_each(|rotation| { roue.effectuer_rotation(rotation) });
        roue
    }
}

#[derive(PartialEq)]
enum Direction {
    DROITE,
    GAUCHE
}
struct Rotation {
    direction: Direction,
    angle: i128,
}
impl Rotation {
    fn parse(input: &str) -> Result<Rotation, String> {
        let direction = if input.starts_with('L') { Direction::GAUCHE } else { Direction::DROITE };
        let angle = input[1..].parse::<i128>()
            .map_err(|_| format!("Angle invalide dans '{input}'"))?;
        
        Ok(Rotation { direction, angle })
    }

}

mod dial {
    use crate::aoc2025::day::day1::{Direction, Rotation};

    pub struct Roue {
        etat: i128,
        nb_passage_via_zero: i128,
        nb_arrivee_zero: i128,
    }

    impl Roue {
        pub fn new() -> Roue {
            Roue { etat: 50, nb_arrivee_zero: 0, nb_passage_via_zero: 0 }
        }

        pub fn effectuer_rotation(&mut self, rotation: Rotation) {
            if rotation.direction == Direction::DROITE {
                self.nb_passage_via_zero += (self.etat + rotation.angle) / 100;
                self.etat = (self.etat + rotation.angle).rem_euclid(100);
            } else {
                let etat_retourne = (100 - self.etat) % 100;
                self.nb_passage_via_zero += (etat_retourne + rotation.angle) / 100;
                self.etat = (self.etat - rotation.angle).rem_euclid(100);
            }
            self.nb_arrivee_zero += i128::from(self.etat == 0);
        }

        pub fn get_resultat_partie1(&self) -> i128 {
            self.nb_arrivee_zero
        }

        pub fn get_resultat_partie2(&self) -> i128 {
            self.nb_passage_via_zero
        }

    }
}
