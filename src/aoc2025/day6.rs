use crate::aoc2025::day6_input_file::INPUT_FILE_DAY6;
use crate::day_trait::{Day, InputFile};

pub struct Day6 {}
impl Day for Day6 {
    fn get_description(&self) -> String {
        String::from("Année 2025, Jour 6")
    }

    fn executer_partie1(&self, input: &str) -> i128 {
        preparer_input_partie1(input).iter()
            .map(Colonne::effectuer_operation)
            .sum()
    }

    fn executer_partie2(&self, input: &str) -> i128 {
        preparer_input_partie2(input).iter()
            .map(Colonne::effectuer_operation)
            .sum()
    }
    fn recuperer_input_file(&self) -> InputFile {
        INPUT_FILE_DAY6
    }
}

#[derive(Clone)]
enum Operation {
    ADDITION,
    MULTIPLICATION,
}

struct Colonne {
    operation: Operation,
    nombres: Vec<i128>
}
impl Colonne {
    fn effectuer_operation(&self) -> i128 {
        match self.operation {
            Operation::ADDITION => self.nombres.iter().sum(),
            Operation::MULTIPLICATION => self.nombres.iter().product()
        }
    }
}

fn preparer_input_partie1(input: &str) -> Vec<Colonne> {
    let mut nombres: Vec<Vec<i128>> = Vec::new();
    let (lignes_nombre, ligne_operation) = extraire_info_input(input);

    for ligne in lignes_nombre.iter() {
        ligne
            .split(".")
            .filter(|l| *l != "")
            .map(|s| s.parse::<i128>().expect("L'input n'a pas le format attendu"))
            .enumerate()
            .for_each(|(i, nombre)|{
                ajouter_nombre_dans_resultat(&mut nombres, i, nombre)
            });

    }
    construire_liste_colonnes(
        &nombres,
        &construire_vecteur_operations(ligne_operation)
    )
}

fn preparer_input_partie2(input: &str) -> Vec<Colonne> {
    let mut nombres: Vec<Vec<i128>> = Vec::new();
    let nb_colonnes = input.lines().last().map_or(0, str::len);

    let (lignes_nombre, ligne_operation) = extraire_info_input(input);

    let mut numero_colonne_a_utiliser = 0;
    for col in 0..nb_colonnes+1 {
        let mut doit_passer_acompte_suivant = true;
        let mut colonne_aenregistrer: i128 = 0;
        for ligne in &lignes_nombre {
            let caractere = ligne.get(col..col+1).unwrap_or(".");
            if let Ok(n) = caractere.parse::<i128>() {
                colonne_aenregistrer = 10 * colonne_aenregistrer + n;
                doit_passer_acompte_suivant = false;
            }
        }
        if doit_passer_acompte_suivant {
            numero_colonne_a_utiliser += 1;
        } else {
            ajouter_nombre_dans_resultat(
                &mut nombres,
                numero_colonne_a_utiliser,
                colonne_aenregistrer
            );
        }
    }

    construire_liste_colonnes(
        &nombres,
        &construire_vecteur_operations(ligne_operation)
    )
}

fn ajouter_nombre_dans_resultat(
    nombres: &mut Vec<Vec<i128>>,
    numero_colonne_a_utiliser: usize,
    nombre_a_enregistrer: i128
) {
    if nombres.len() <= numero_colonne_a_utiliser {
        nombres.push(Vec::new());
    }
    nombres[numero_colonne_a_utiliser].push(nombre_a_enregistrer);
}

fn extraire_info_input(input: &str) -> (Vec<&str>, &str) {
    let mut lignes_nombre: Vec<&str> = input.lines().collect();
    let ligne_operation = lignes_nombre.pop().expect("Mauvais format input");
    (lignes_nombre, ligne_operation)
}

fn construire_vecteur_operations(ligne_operation: &str) -> Vec<Operation> {
    let mut operations: Vec<Operation> = Vec::new();
    ligne_operation
        .split(".")
        .filter(|l| *l != "")
        .for_each(|operation| {
            match operation {
                "+" => operations.push(Operation::ADDITION),
                "*" => operations.push(Operation::MULTIPLICATION),
                _ => panic!("Invalid operation")
            }
        });
    operations
}

fn construire_liste_colonnes(
    nombres: &Vec<Vec<i128>>,
    operations: &Vec<Operation>
) -> Vec<Colonne> {
    operations.iter()
        .zip(nombres)
        .map(|(operation, nombres)| Colonne {
            operation: operation.clone(),
            nombres: nombres.clone()
        })
        .collect()
}

