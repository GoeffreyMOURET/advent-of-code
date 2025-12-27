use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn calculer_nb_chemins_par_dfs<T>(
    input: &HashMap<&T, &HashSet<&T>>,
    depart: &T,
    arrivee: &T,
) -> u128 where T: Eq + Hash + ?Sized {
    let mut cache: HashMap<&T, u128> = HashMap::new();
    dfs(input, &mut cache, depart, arrivee)
}

fn dfs<'a, T>(
    input: &HashMap<&'a T, &HashSet<&'a T>>,
    cache: &mut HashMap<&'a T, u128>,
    node: &'a T,
    end: &'a T
) -> u128 where T: PartialEq + ?Sized, T: Eq + Hash {
    if node == end { return 1; }
    if let Some(resultat_cache) = cache.get(&node) {
        return *resultat_cache;
    } else {
        let result = input.get(&node).unwrap_or(&&HashSet::new()).iter()
            .map(|&next| dfs(input, cache, next, end))
            .sum();
        cache.insert(node, result);
        result
    }
}

