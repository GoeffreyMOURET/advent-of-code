use crate::aoc2025::input::day2_input_file::INPUT_FILE_DAY2;
use crate::structures::day_trait::{Day, InputFile};
use std::collections::HashSet;

pub struct Day2 {}
impl Day for Day2 {
    fn get_description(&self) -> String {
        String::from("Année 2025, Jour 2")
    }

    fn executer_partie1(&self, input: &str) -> i128 {
        let ranges: Vec<Range> = Self::parser_input(input);
        let max = ranges.iter().map(|r| r.fin).max().unwrap();
        let valeurs_incorrectes =
            generateur::GenerateurValeursIncorrectes::recuperer_valeurs_incorrectes(max, true);
        Self::sommer_valeurs_incorrectes_presentes(ranges, valeurs_incorrectes)
    }

    fn executer_partie2(&self, input: &str) -> i128 {
        let ranges: Vec<Range> = Self::parser_input(input);
        let max = ranges.iter().map(|r| r.fin).max().unwrap();
        let valeurs_incorrectes =
            generateur::GenerateurValeursIncorrectes::recuperer_valeurs_incorrectes(max, false);
        Self::sommer_valeurs_incorrectes_presentes(ranges, valeurs_incorrectes)
    }
    fn recuperer_input_file(&self) -> InputFile {
        INPUT_FILE_DAY2
    }
}

impl Day2 {
    fn sommer_valeurs_incorrectes_presentes(ranges: Vec<Range>, valeurs_incorrectes: HashSet<i128>) -> i128 {
        ranges.iter()
            .map(|range| {
                valeurs_incorrectes.iter()
                    .filter(|v_incorr| { **v_incorr >= range.debut && **v_incorr <= range.fin })
                    .sum::<i128>()
            })
            .sum::<i128>()
    }

    fn parser_input(input: &str) -> Vec<Range> {
        input.split(',')
            .map(Range::parse)
            .filter_map(Result::ok)
            .collect()
    }
}

struct Range {
    debut: i128,
    fin: i128,
}

impl Range {
    fn parse(input: &str) -> Result<Range, String> {
        let parts: Vec<&str> = input.split('-').collect();
        let [debut, fin] = parts.as_slice() else {
            return Err(format!("Invalid input: {}", input));
        };
        Ok(
            Range{
                debut: debut.parse().map_err(|_| { "Erreur dans la conversion du début".to_string() })?,
                fin: fin.parse().map_err(|_| { "Erreur dans la conversion de la fin".to_string() })?,
            }
        )
    }
}

mod generateur {
    use std::collections::HashSet;

    pub struct GenerateurValeursIncorrectes {
        etat_actuel: i128,
        valeur_incorrecte_doublon: i128
    }

    impl GenerateurValeursIncorrectes {
        fn new() -> GenerateurValeursIncorrectes {
            GenerateurValeursIncorrectes {
                etat_actuel: 1,
                valeur_incorrecte_doublon: 11
            }
        }

        pub fn recuperer_valeurs_incorrectes(
            borne_superieure: i128,
            compter_uniquement_doublon: bool
        ) -> HashSet<i128> {
            let mut resultat: HashSet<i128> = HashSet::new();
            let mut generateur = GenerateurValeursIncorrectes::new();
            while generateur.valeur_incorrecte_doublon <= borne_superieure {
                let mut valeur_incorrecte_a_ajouter = generateur.valeur_incorrecte_doublon;
                while valeur_incorrecte_a_ajouter <= borne_superieure {
                    resultat.insert(valeur_incorrecte_a_ajouter);
                    if compter_uniquement_doublon { break }
                    valeur_incorrecte_a_ajouter =
                        format!("{}{}", valeur_incorrecte_a_ajouter, generateur.etat_actuel).parse().unwrap();
                }
                generateur.etat_actuel += 1;
                generateur.valeur_incorrecte_doublon =
                    format!("{}{}", generateur.etat_actuel, generateur.etat_actuel).parse().unwrap();
            }
            resultat
        }
    }
}


