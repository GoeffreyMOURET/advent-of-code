use crate::aoc2025::input::day9_input_file::INPUT_FILE_DAY9;
use crate::structures::day_trait::{Day, InputFile};
use crate::utils::min_max_utils::get_max_min;

pub struct Day9 {}
impl Day for Day9 {
    fn get_description(&self) -> String {
        String::from("Année 2025, Jour 9")
    }

    fn executer_partie1(&self, input: &str) -> i128 {
        let points = Self::parse_points(input);
        let rectangles = CouplePoint::construire_produit_cartesien(&points);

        rectangles
            .iter()
            .max_by(|r1, r2| { r1.surface.cmp(&r2.surface) })
            .unwrap().surface
    }

    fn executer_partie2(&self, input: &str) -> i128 {
        let points = Self::parse_points(input);
        let mut rectangles = CouplePoint::construire_produit_cartesien(&points);
        // On trie la liste des possibilités par surface décroissante. Ainsi, le premier résultat trouvé possible est le max !
        rectangles.sort_by_key(&|r: &CouplePoint| { -1 * r.surface });
        rectangles.iter().find(|couple | {

            let (xmax, xmin) = get_max_min(couple.point1.x, couple.point2.x);
            let (ymax, ymin) = get_max_min(couple.point1.y, couple.point2.y);

            points
                .iter()
                .zip(points.iter().cycle().skip(1).take(points.len()))
                .all(|(&point, &point_suivant)| {
                    if point.y == point_suivant.y {
                        point.y >= ymax
                            || point.y <= ymin
                            || (point.x <= xmin && point_suivant.x <= xmin)
                            || (point.x >= xmax && point_suivant.x >= xmax)
                    } else {
                        debug_assert!(point.x == point_suivant.x, "Les points n'ont pas été construits correctement");
                        point.x >= xmax
                            || point.x <= xmin
                            || (point.y <= ymin && point_suivant.y <= ymin)
                            || (point.y >= ymax && point_suivant.y >= ymax)
                    }
                })
        }).unwrap().surface
    }
    fn recuperer_input_file(&self) -> InputFile {
        INPUT_FILE_DAY9
    }
}

impl Day9 {
    fn parse_points(input: &str) -> Vec<Point> {
        input
            .lines()
            .enumerate()// découpe en lignes
            .filter_map(|(index, line)| Point::parse(line, index))
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Point {
    x: i128,
    y: i128,
    numero: usize,
}


impl Point {
    fn parse(line: &str, numero: usize) -> Option<Point> {
        let coords: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        if coords.len() == 2 {
            // conversion en f64
            let x = coords[0].parse::<i128>().ok()?;
            let y = coords[1].parse::<i128>().ok()?;
            Some(Point { x, y, numero })
        } else {
            None
        }
    }

    fn calculer_surface(p1: &Point, p2: &Point) -> i128 {
        ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1)
    }
}

#[derive(Debug)]
struct CouplePoint<'a> {
    point1: &'a Point,
    point2: &'a Point,
    surface: i128,
}
impl CouplePoint<'_> {
    fn construire_produit_cartesien(points: &'_ Vec<Point>) -> Vec<CouplePoint<'_>> {
        points.iter()
            .flat_map(|point1| {
                points.iter().filter_map(|point2| {
                    if point1.numero >= point2.numero {
                        None
                    } else {
                        Some(CouplePoint { point1, point2, surface: Point::calculer_surface(point1, point2) })
                    }
                })
            })
            .collect()
    }
}



