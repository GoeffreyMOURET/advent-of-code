use crate::aoc2025::input::day11_input_file::{INPUT_FILE_DAY11, INPUT_FILE_DAY11_PART2};
use crate::structures::day_trait::{Day, InputFile};
use crate::utils::dfs_utils::calculer_nb_chemins_par_dfs;
use std::collections::{HashMap, HashSet};

pub struct Day11 {}
impl Day for Day11 {
    fn get_description(&self) -> String {
        String::from("Année 2025, Jour 11")
    }

    fn executer_partie1(&self, input: &str) -> i128 {
        let etapes = Self::parse(input);
        Self::compter_nb_chemins(&etapes, "you", "out") as i128
    }

    fn executer_partie2(&self, input: &str) -> i128 {
        let etapes = Self::parse(input);
        (Self::compter_nb_chemins(&etapes, "svr", "dac")
            * Self::compter_nb_chemins(&etapes, "dac", "fft")
            * Self::compter_nb_chemins(&etapes, "fft", "out")
            + Self::compter_nb_chemins(&etapes, "svr", "fft")
            * Self::compter_nb_chemins(&etapes, "fft", "dac")
            * Self::compter_nb_chemins(&etapes, "dac", "out"))
            as i128
    }
    fn recuperer_input_file(&self) -> InputFile {
        INPUT_FILE_DAY11
    }

    fn recuperer_input_file_partie2(&self) -> InputFile {
        INPUT_FILE_DAY11_PART2
    }
}

impl Day11 {
    fn parse(input: &'_ str) -> Vec<Etape<'_>> {
        input.lines()
            .filter_map(|l| Etape::parse(l).ok())
            .collect()
    }


    fn compter_nb_chemins(etapes: &Vec<Etape>, depart: &str, arrivee: &str) -> u128 {
        let mut graph: HashMap<&str, &HashSet<&str>>  = HashMap::new();
        for etape in etapes {
            graph.insert(etape.depart, &etape.arrivees);
        }
        calculer_nb_chemins_par_dfs(
            &graph,
            depart,
            arrivee,
        )
    }

}

#[derive(Eq, PartialEq, Debug)]
struct Etape<'a> {
    depart: &'a str,
    arrivees: HashSet<&'a str>,
}

impl Etape<'_> {
    fn parse(input: &'_ str) -> Result<Etape<'_>, String> {
        let parts = input.split(':').collect::<Vec<&str>>();
        let [depart, arrives] = parts.as_slice() else {
            return Err(format!("Invalid input: {}", input));
        };
        let arrivees = arrives.split_whitespace()
            .filter(|&ligne| !ligne.is_empty())
            .collect::<HashSet<&str>>();
         Ok(Etape{ arrivees, depart, })
    }
}
