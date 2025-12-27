use crate::aoc2025::input::day7_input_file::INPUT_FILE_DAY7;
use crate::structures::day_trait::{Day, InputFile};
use std::collections::{HashMap, HashSet};

const SPLITTER: char = '^';
const START: char = 'S';
pub struct Day7 {}
impl Day for Day7 {
    fn get_description(&self) -> String {
        String::from("Année 2025, Jour 7")
    }

    fn executer_partie1(&self, input: &str) -> i128 {
        Self::calculer_splitters_atteints(input, false).len() as i128
    }

    fn executer_partie2(&self, input: &str) -> i128 {
        Self::calculer_splitters_atteints(input, true).iter()
            .map(|(_, info)| info.nb_rayons * ((2 - info.nb_enfants) as i128))
            .sum()

    }

    fn recuperer_input_file(&self) -> InputFile {
        INPUT_FILE_DAY7
    }
}

impl Day7 {
    fn mettre_a_jour_splitters_atteints(
        splitters_atteints: &mut HashMap<Coordonnees, InfoSplitter>,
        coordonnees_actuelles: Coordonnees,
        coordonnees_parent: Coordonnees,
    ) {
        let info_precedente = splitters_atteints
            .get(&coordonnees_actuelles)
            .cloned()
            .unwrap_or(InfoSplitter { nb_enfants: 0, nb_rayons: 0 });

        let info_parent = splitters_atteints
            .get(&coordonnees_parent)
            .cloned()
            .unwrap_or(InfoSplitter { nb_enfants: 0, nb_rayons: 1 });

        splitters_atteints.insert(
            coordonnees_actuelles,
            InfoSplitter {
                nb_rayons: info_precedente.nb_rayons + info_parent.nb_rayons,
                nb_enfants: 0,
            },
        );

        if splitters_atteints.contains_key(&coordonnees_parent) {
            splitters_atteints.insert(
                coordonnees_parent,
                InfoSplitter {
                    nb_enfants: info_parent.nb_enfants + 1,
                    nb_rayons: info_parent.nb_rayons,
                },
            );
        }
    }


    fn calculer_splitters_atteints(
        input: &str,
        parcourir_toute_profondeur: bool
    ) -> HashMap<Coordonnees, InfoSplitter> {
        let mut splitters_atteints: HashMap<Coordonnees, InfoSplitter> = HashMap::new();
        let mut splitters: HashSet<Coordonnees> = HashSet::new();

        let mut start: Coordonnees = Coordonnees { x: 0, y: 0 };
        for (index_y, line) in input.lines().enumerate() {
            for (index_x, char) in line.chars().enumerate() {
                match char {
                    START => {
                        start.x = index_x;
                        start.y = index_y;
                    },
                    SPLITTER => {
                        splitters.insert(Coordonnees { x: index_x, y: index_y });
                        let coordonnees_actuelles = Coordonnees { x: index_x, y: index_y };
                        for y in (0..index_y - 1).rev() {
                            let mut doit_break = false;
                            if start.x == index_x && start.y == y {
                                Self::mettre_a_jour_splitters_atteints(
                                    &mut splitters_atteints,
                                    coordonnees_actuelles,
                                    start
                                );
                                doit_break = !parcourir_toute_profondeur;
                            }

                            let voisins_valides: Vec<_> = [
                                Coordonnees { x: index_x - 1, y },
                                Coordonnees { x: index_x + 1, y },
                            ]
                                .into_iter()
                                .filter(|v| splitters_atteints.contains_key(v))
                                .collect();

                            for voisin in voisins_valides {
                                Self::mettre_a_jour_splitters_atteints(
                                    &mut splitters_atteints,
                                    coordonnees_actuelles,
                                    voisin,
                                );
                                doit_break = !parcourir_toute_profondeur;
                            }

                            if doit_break || splitters.contains(&Coordonnees { x: index_x, y }) { break }
                        }
                    },
                    _ => continue,
                }
            }
        };
        splitters_atteints
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Copy, Clone)]
struct Coordonnees {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct InfoSplitter {
    nb_enfants: i8,
    nb_rayons: i128,
}