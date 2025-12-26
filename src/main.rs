use std::env;
use advent_of_code::services::execution_jour_service::ExecutionJourService;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut jour: i8 = 6;
    if args.len() > 1 && let Ok(n) = args[1].parse::<i8>() {
        jour = n;
    }
    ExecutionJourService::executer_jour(jour);
}
