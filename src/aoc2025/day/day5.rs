use crate::aoc2025::input::day5_input_file::INPUT_FILE_DAY5;
use crate::structures::day_trait::{Day, InputFile};

pub struct Day5 {}
impl Day for Day5 {
    fn get_description(&self) -> String {
        String::from("Année 2025, Jour 5")
    }

    fn executer_partie1(&self, input: &str) -> i128 {
        let input = Input::parse(input).unwrap();
        input.ingredients.iter()
            .filter(|ingredient| {
                input.ranges.iter().any(|range| { range.min <= **ingredient && **ingredient <= range.max })
            })
            .count() as i128
    }

    fn executer_partie2(&self, input: &str) -> i128 {
        Self::fusionner_ranges(Input::parse(input).unwrap().ranges).iter()
            .map(|range| { range.max - range.min + 1})
            .sum()
    }
    fn recuperer_input_file(&self) -> InputFile {
        INPUT_FILE_DAY5
    }
}



impl Day5 {
    fn fusionner_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
        ranges.sort_by_key(|r| r.min);

        let mut resultat: Vec<Range> = Vec::new();
        for r in ranges {
            if let Some(last) = resultat.last_mut() {
                if r.min <= last.max + 1 {
                    last.max = last.max.max(r.max);
                    continue;
                }
            }
            resultat.push(r);
        }

        resultat
    }

}


#[derive(PartialEq, Eq, Hash, Debug)]
#[derive(Clone)]
struct Range{
    min: i128,
    max: i128,
}
impl Range {
    fn parse(input: &str) -> Result<Range, String> {
        let range: Vec<_> = input
            .split('-')
            .filter_map(|s| { s.parse::<i128>().ok() })
            .collect();

        let [min, max] = range.as_slice() else {
            return Err(format!("Range invalide: {}", input));
        };
        Ok(
            Range { min: *min, max: *max }
        )
    }
}

struct Input {
    ranges: Vec<Range>,
    ingredients: Vec<i128>,
}

impl Input {
    fn parse(input: &str) -> Result<Input, String> {
        let mut ingredients = Vec::new();
        let mut ranges = Vec::new();

        for line in input.lines() {
            if line.contains('-') {
                ranges.push(Range::parse(line)?);
            } else if !line.is_empty() {
                ingredients.push(
                    line
                        .parse::<i128>()
                        .map_err(|_| format!("Ingredient invalide: {}", line))?
                );
            }
        }

        Ok(Input { ranges, ingredients })
    }
}
