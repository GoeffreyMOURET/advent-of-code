use crate::aoc2024::day::day1::Day1;
use crate::aoc2024::day::day2::Day2;
use crate::structures::day_trait::{Day, ExecutionAnnee};

pub struct ExecutionJourAnnee2024Service;
impl ExecutionAnnee for ExecutionJourAnnee2024Service {
    fn recuperer_jour(numero_jour: u8) -> Box<dyn Day> {
        match numero_jour {
            1 => Box::new(Day1{}),
            2 => Box::new(Day2{}),
            _ => panic!("Non implémenté"),
        }
    }
}