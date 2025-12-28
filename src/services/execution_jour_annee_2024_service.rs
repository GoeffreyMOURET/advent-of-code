use crate::aoc2024::day::day1::Day1;
use crate::structures::day_trait::{Day, ExecutionAnnee};

pub struct ExecutionJourAnnee2024Service;
impl ExecutionAnnee for ExecutionJourAnnee2024Service {
    fn recuperer_jour(numero_jour: u8) -> Box<dyn Day> {
        match numero_jour {
            1 => Box::new(Day1{}),
            _ => panic!("Non implémenté"),
        }
    }
}