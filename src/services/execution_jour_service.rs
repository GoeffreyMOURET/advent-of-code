use crate::aoc2025::day1::Day1;
use crate::aoc2025::day10::Day10;
use crate::aoc2025::day12::Day12;
use crate::aoc2025::day2::Day2;
use crate::aoc2025::day3::Day3;
use crate::aoc2025::day4::Day4;
use crate::aoc2025::day6::Day6;
use crate::aoc2025::day7::Day7;
use crate::aoc2025::day8::Day8;
use crate::aoc2025::day9::Day9;
use crate::structures::day_trait::Day;
use crate::structures::resultat_jour::ResultatJour;

pub struct ExecutionJourService;

impl ExecutionJourService {

    fn recuperer_jour(numero_jour: i8) -> Box<dyn Day> {
        match numero_jour {
            1 => Box::new(Day1{}),
            2 => Box::new(Day2{}),
            3 => Box::new(Day3{}),
            4 => Box::new(Day4{}),
            6 => Box::new(Day6{}),
            7 => Box::new(Day7{}),
            8 => Box::new(Day8{}),
            9 => Box::new(Day9{}),
            10 => Box::new(Day10{}),
            12 => Box::new(Day12{}),
            _ => panic!("Non implémenté"),
        }
    }

    pub fn executer_jour(
        numero_jour: i8
    ) {
        println!("Exécution de l'advent of code : {}", numero_jour);

        let jour = Self::recuperer_jour(numero_jour);

        println!("Executor Partie 1 : {}", jour.get_description());
        let input_file = jour.recuperer_input_file();
        let resultat = ResultatJour {
            exemple: jour.executer_partie1(input_file.exemple),
            inconnu: jour.executer_partie1(input_file.inconnu),
        };
        println!("{resultat}");

        println!("Executor Partie 2 : {}", jour.get_description());
        let input_file = jour.recuperer_input_file_partie2();
        let resultat = ResultatJour {
            exemple: jour.executer_partie2(input_file.exemple),
            inconnu: jour.executer_partie2(input_file.inconnu),
        };
        println!("{resultat}");
    }
}