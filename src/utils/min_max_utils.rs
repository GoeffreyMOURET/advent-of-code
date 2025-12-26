pub fn get_max_min_by<T, K, F>(
    input1: T,
    input2: T,
    key: &F
) -> (T, T)
where F: Fn(&T) -> &K,
      K: Ord {
    if key(&input1) > key(&input2) {
        (input1, input2)
    } else {
        (input2, input1)
    }
}

pub fn get_max_min<T>(
    input1: T,
    input2: T
) -> (T, T)
where T: Ord {
    get_max_min_by(input1, input2, &|x| x)
}