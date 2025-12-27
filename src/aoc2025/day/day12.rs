use crate::aoc2025::input::day12_input_file::INPUT_FILE_DAY12;
use crate::structures::day_trait::{Day, InputFile};

const NB_FORMS_CADEAUX: usize = 6;
const BLOC_PLEIN: char = '#';
const BLOC_VIDE: char = '.';
pub struct Day12 {}
impl Day for Day12 {
    fn get_description(&self) -> String {
        String::from("Année 2025, Jour 12")
    }

    fn executer_partie1(&self, input: &str) -> i128 {
        let input_ligne: Vec<&str> = input.lines().collect();
        let cadeaux: Vec<TypeCadeau> = (0..NB_FORMS_CADEAUX).into_iter()
            .map(|index| {
                let forme_input = input_ligne[5*index..5*(index+1)].join("\n");
                TypeCadeau::parse(&forme_input)
            })
            .collect();

        let boites: Vec<Boite> = input_ligne[5*NB_FORMS_CADEAUX..].iter()
            .filter_map(|ligne| { Boite::parse(ligne).ok() })
            .collect();

        boites.iter()
            .filter(|boite| {
                let aire_necessaire: i128 = boite.cible_contenant.iter().enumerate()
                    .map(|(index_cadeau, nb_cadeaux)| { cadeaux[index_cadeau].calculer_aire() * (*nb_cadeaux as i128)})
                    .sum::<i128>();
                // A little magic trick !
                aire_necessaire < boite.largeur * (boite.hauteur - 1)
            })
            .count() as i128
    }

    fn executer_partie2(&self, input: &str) -> i128 {
        println!("Merry Christmas !");
        self.executer_partie1(input)
    }
    fn recuperer_input_file(&self) -> InputFile {
        INPUT_FILE_DAY12
    }
}

#[derive(Debug)]
struct TypeCadeau {
    forme: Vec<Vec<u8>>
}

impl TypeCadeau {
    fn parse(input: &str) -> TypeCadeau {
        TypeCadeau {
            forme: input.lines()
                .filter(|line| line.starts_with(BLOC_VIDE) || line.starts_with(BLOC_PLEIN))
                .map(|line| line.chars().map(|c| if c == BLOC_PLEIN { 1 } else { 0 }).collect())
                .collect()
        }
    }

    fn calculer_aire(&self) -> i128 {
        self.forme.iter()
            .flat_map(|line| line.iter())
            .sum::<u8>() as i128
    }
}

#[derive(Debug)]
struct Boite {
    hauteur: i128,
    largeur: i128,
    cible_contenant: Vec<usize>
}

impl Boite {
    fn parse(input: &str) -> Result<Boite, String> {
        let parts: Vec<_> = input.split(':').collect();

        let [dimension, contenu] = parts.as_slice() else {
            return Err(format!("Invalid input: {}", input));
        };

        let dims: Vec<_> = dimension.split('x').collect();

        let [hauteur, largeur] = dims.as_slice() else {
            return Err(format!("Invalid input: {}", input));
        };

        Ok(Boite {
            hauteur: hauteur.parse().unwrap(),
            largeur: largeur.parse().unwrap(),
            cible_contenant: contenu
                .split(' ')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect(),
        })

    }
}


