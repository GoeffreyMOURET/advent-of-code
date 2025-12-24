use std::env;

mod aoc2025;
mod day_trait;
mod resultat_jour;
mod execution_jour_service;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut jour: i8 = 6;
    if args.len() > 1 && let Ok(n) = args[1].parse::<i8>() {
        jour = n;
    }

    aoc2025::executer_jour(jour);
}
