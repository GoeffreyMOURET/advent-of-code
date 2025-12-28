use advent_of_code::services::execution_service::ExecutionService;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut jour: u8 = 1;
    let mut annee: u16 = 2025;
    if args.len() > 2 && let Ok(a) = args[1].parse::<u16>() && let Ok(j) = args[2].parse::<u8>() {
        jour = j;
        annee = a;

    }
    else if args.len() > 1 && let Ok(n) = args[1].parse::<u16>() {
        if n > 100 { annee = n }
        else { jour = n as u8; }
    }
    ExecutionService::executer_aoc(annee, jour);
}
