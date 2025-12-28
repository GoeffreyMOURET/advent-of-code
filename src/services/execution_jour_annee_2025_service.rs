use crate::aoc2025::day::day1::Day1;
use crate::aoc2025::day::day10::Day10;
use crate::aoc2025::day::day11::Day11;
use crate::aoc2025::day::day12::Day12;
use crate::aoc2025::day::day2::Day2;
use crate::aoc2025::day::day3::Day3;
use crate::aoc2025::day::day4::Day4;
use crate::aoc2025::day::day5::Day5;
use crate::aoc2025::day::day6::Day6;
use crate::aoc2025::day::day7::Day7;
use crate::aoc2025::day::day8::Day8;
use crate::aoc2025::day::day9::Day9;
use crate::structures::day_trait::{Day, ExecutionAnnee};

pub(crate) struct ExecutionJourAnnee2025Service;

impl ExecutionAnnee for ExecutionJourAnnee2025Service {
    fn recuperer_jour(numero_jour: u8) -> Box<dyn Day> {
        match numero_jour {
            1 => Box::new(Day1{}),
            2 => Box::new(Day2{}),
            3 => Box::new(Day3{}),
            4 => Box::new(Day4{}),
            5 => Box::new(Day5{}),
            6 => Box::new(Day6{}),
            7 => Box::new(Day7{}),
            8 => Box::new(Day8{}),
            9 => Box::new(Day9{}),
            10 => Box::new(Day10{}),
            11 => Box::new(Day11{}),
            12 => Box::new(Day12{}),
            _ => panic!("Non implémenté"),
        }
    }
}