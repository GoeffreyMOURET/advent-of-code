use crate::aoc2025::input::day4_input_file::INPUT_FILE_DAY4;
use crate::structures::day_trait::{Day, InputFile};
use std::cmp::PartialEq;

const ROULEAU_PAPIER: char = '@';
const SEUIL_NB_ROULEAU: usize = 4;
pub struct Day4 {}
impl Day for Day4 {
    fn get_description(&self) -> String {
        String::from("Année 2025, Jour 4")
    }

    fn executer_partie1(&self, input: &str) -> i128 {
        Grid::parse(input).calculer_etape_suivante() as i128
    }

    fn executer_partie2(&self, input: &str) -> i128 {
        let mut grid = Grid::parse(input);
        let mut nb_rouleaux_retires = 0;

        loop {
            let nouveau_nb_rouleaux_retires = grid.calculer_etape_suivante();
            if nouveau_nb_rouleaux_retires == nb_rouleaux_retires {
                return nouveau_nb_rouleaux_retires as i128;
            }
            nb_rouleaux_retires = nouveau_nb_rouleaux_retires;
        }

    }
    fn recuperer_input_file(&self) -> InputFile {
        INPUT_FILE_DAY4
    }
}

#[derive(PartialEq, Clone, Copy)]

enum StatutEmplacement {
    Vide,
    Rouleau,
    RouleauRetire
}
struct Grid {
    grid: Vec<Vec<StatutEmplacement>>,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let grid: Vec<Vec<StatutEmplacement>> = input.lines()
            .map(|l| l.chars()
                .map(|c| if c == ROULEAU_PAPIER { StatutEmplacement::Rouleau} else { StatutEmplacement::Vide })
                .collect())
            .collect();
        Grid { grid }
    }

    fn get_voisins(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut resultat = Vec::new();
        let (longueur_max, largeur_max) = self.get_dimensions();
        if i>0 {
            resultat.push((i - 1, j));
            if j>0 { resultat.push((i - 1, j - 1)); }
            if j<largeur_max { resultat.push((i - 1, j + 1)); }
        }
        if i < longueur_max {
            resultat.push((i + 1, j));
            if j>0 { resultat.push((i + 1, j - 1)); }
            if j<largeur_max { resultat.push((i + 1, j + 1)); }
        }
        if j>0 { resultat.push((i, j - 1)); }
        if j<largeur_max { resultat.push((i, j + 1)); }
        resultat
    }

    fn get_dimensions(&self) -> (usize, usize) {
        (self.grid.len() - 1, self.grid[0].len() - 1)
    }

    fn est_rouleau(&self, i: usize, j: usize) -> bool {
        self.grid[i][j] == StatutEmplacement::Rouleau
    }

    fn get_nb_rouleaux_papier_voisin(&self, i: usize, j: usize) -> usize {
        self.get_voisins(i, j).iter()
            .filter(|&(i, j)| self.grid[*i][*j] == StatutEmplacement::Rouleau)
        .count()
    }

    fn calculer_etape_suivante(&mut self) -> usize {
        let mut nouvelle_grid: Vec<Vec<StatutEmplacement>> = self.grid.clone();

        let (longueur_max, largeur_max) = self.get_dimensions();
        for i in 0..longueur_max+1 {
            for j in 0..largeur_max+1 {
                if self.est_rouleau(i, j) && self.get_nb_rouleaux_papier_voisin(i, j) < SEUIL_NB_ROULEAU {
                    nouvelle_grid[i][j] = StatutEmplacement::RouleauRetire;
                }
            }
        }

        let nb_rouleaux_retires = nouvelle_grid.iter()
            .map(|row| {
                row.into_iter()
                    .filter(|statut_emplacement| { **statut_emplacement==StatutEmplacement::RouleauRetire })
                    .count()
            })
            .sum();

        self.grid = nouvelle_grid;

        nb_rouleaux_retires
    }
}

