use crate::aoc2025::input::day8_input_file::INPUT_FILE_DAY8;
use crate::structures::day_trait::{Day, InputFile};
pub struct Day8 {}
impl Day for Day8 {
    fn get_description(&self) -> String {
        String::from("Année 2025, Jour 8")
    }

    fn executer_partie1(&self, input: &str) -> i128 {
        let points = Day8::parse_points(input);

        let mut distance = CouplePoint::construire_produit_cartesien(points.as_ref());
        distance.sort_by_key(&|c: &CouplePoint| c.distance);

        // Trick pour savoir si on est dans le cas de l'exemple ou si on est dans le vrai cas
        let nb_repetitions = if points.len() < 100 { 10 } else { 1000 };
        if nb_repetitions > distance.len() { panic!("nb_repetitions > distance.len() = {}", nb_repetitions); }

        let mut jonctions: Vec<HashSet<Point3D>> = Vec::new();
        for i in 0..nb_repetitions {
            let distance = distance.get(i).unwrap();
            Self::merge_conections(&mut jonctions, distance.point1, distance.point2)
        }

        Self::calculer_resultat_partie_1(jonctions)
    }

    fn executer_partie2(&self, input: &str) -> i128 {
        let points = Day8::parse_points(input);

        let mut distance = CouplePoint::construire_produit_cartesien(points.as_ref());
        distance.sort_by_key(&|c: &CouplePoint| c.distance);

        let mut jonctions: Vec<HashSet<Point3D>> = Vec::new();

        let mut index_prochaine_jonction = 0;
        while Self::est_en_cours_de_jonction(&points, &mut jonctions) {
            let distance = distance.get(index_prochaine_jonction).unwrap();
            Self::merge_conections(&mut jonctions, distance.point1, distance.point2);
            index_prochaine_jonction += 1;
        }

        let derniere_jonction = distance.get(index_prochaine_jonction - 1).unwrap();
        derniere_jonction.point1.x * derniere_jonction.point2.x
    }
    fn recuperer_input_file(&self) -> InputFile {
        INPUT_FILE_DAY8
    }
}

impl Day8 {

    fn parse_points(input: &str) -> Vec<Point3D> {
        input
            .lines() // découpe en lignes
            .filter_map(|line| Point3D::parse(line))
            .collect()
    }

    fn merge_conections<'a>(
        connections: &'a mut Vec<HashSet<Point3D>>,
        point1: &Point3D,
        point2: &Point3D
    ) {

        let mut connections_a_merge: Vec<HashSet<Point3D>> = Vec::new();
        let mut indexes_a_retirer: Vec<usize> = Vec::new();
        for (index, connection) in connections.iter().enumerate() {
            let contient_un_des_points = connection.iter()
                .find(|point| { *point == point1 || *point == point2 })
                .is_some();
            if contient_un_des_points {
                indexes_a_retirer.push(index);
                connections_a_merge.push(connection.clone());
            }
        }

        let mut nouveau_hashset = HashSet::new();
        nouveau_hashset.insert(*point1);
        nouveau_hashset.insert(*point2);
        for point in connections_a_merge.into_iter().flat_map(|c| c.into_iter()) {
            nouveau_hashset.insert(point);
        }

        for index in indexes_a_retirer.iter().rev() {
            connections.remove(*index);
        }

        connections.push(nouveau_hashset);

    }

    fn calculer_resultat_partie_1(mut connections: Vec<HashSet<Point3D>>) -> i128 {
        connections.sort_by_key(|set| set.len());
        connections = connections.iter().rev().cloned().collect();

        connections.get(0).unwrap().len() as i128
            * connections.get(1).unwrap().len() as i128
            * connections.get(2).unwrap().len() as i128
    }

    fn est_en_cours_de_jonction(
        points: &Vec<Point3D>,
        connections: &mut Vec<HashSet<Point3D>>
    ) -> bool {
        connections.len() != 1 || connections.get(0).unwrap().len() != points.len()
    }
}

use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Point3D {
    x: i128,
    y: i128,
    z: i128,
}


impl Point3D {
    fn parse(line: &str) -> Option<Point3D> {
        let coords: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        if coords.len() == 3 {
            // conversion en f64
            let x = coords[0].parse::<i128>().ok()?;
            let y = coords[1].parse::<i128>().ok()?;
            let z = coords[2].parse::<i128>().ok()?;
            Some(Point3D { x, y, z })
        } else {
            None
        }
    }

    fn calculer_distance_euclidean_carree(p1: &Point3D, p2: &Point3D) -> i128 {
        (p1.x - p2.x).pow(2)
            + (p1.y - p2.y).pow(2)
            + (p1.z - p2.z).pow(2)
    }
}


#[derive(Debug)]
struct CouplePoint<'a> {
    point1: &'a Point3D,
    point2: &'a Point3D,
    distance: i128,
}
impl CouplePoint<'_> {
    fn construire_produit_cartesien(points: &'_ Vec<Point3D>) -> Vec<CouplePoint<'_>> {
        points.iter()
            .flat_map(|point1| {
                points.iter().filter_map(|point2| {
                    if point1.x < point2.x
                        || (point1.x == point2.x && point1.y < point2.y)
                        || (point1.x == point2.x && point1.y == point2.y && point1.z <= point2.z) {
                        None
                    } else {
                        Some(CouplePoint { point1, point2, distance: Point3D::calculer_distance_euclidean_carree(point1, point2) })
                    }
                })
            })
            .collect()
    }
}

