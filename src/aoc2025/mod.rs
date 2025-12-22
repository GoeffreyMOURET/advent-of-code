use crate::aoc2025::day6::Day6;
use crate::aoc2025::day6_input_file::{INPUT_FILE_DAY6};
use crate::day_trait::Day;
use crate::execution_jour_service::ExecutionJourService;

mod day6;
mod day6_input_file;

pub fn executer_jour(numero_jour: i8) {
    ExecutionJourService::executer_jour(
        recuperer_jour(numero_jour),
        INPUT_FILE_DAY6
    );
}

fn recuperer_jour(numero_jour: i8) -> impl Day {
    match numero_jour {
        6 => Day6{},
        _ => panic!("Non implémenté"),
    }
}