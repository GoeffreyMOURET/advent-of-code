use crate::aoc2025::day6::Day6;
use crate::aoc2025::day7::Day7;
use crate::aoc2025::day8::Day8;
use crate::aoc2025::day9::Day9;
use crate::day_trait::Day;
use crate::execution_jour_service::ExecutionJourService;

mod day6;
mod day6_input_file;
mod day7_input_file;
mod day7;
mod day8;
mod day8_input_file;
mod day9;
mod day9_input_file;

pub fn executer_jour(jour: i8) {
    println!("Exécution de l'advent of code : {}", jour);
    ExecutionJourService::executer_jour(
        recuperer_jour(jour)
    );
}

fn recuperer_jour(numero_jour: i8) -> Box<dyn Day> {
    match numero_jour {
        6 => Box::new(Day6{}),
        7 => Box::new(Day7{}),
        8 => Box::new(Day8{}),
        9 => Box::new(Day9{}),
        _ => panic!("Non implémenté"),
    }
}