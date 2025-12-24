pub fn quick_sort_by<T, F, K>(arr: &mut [T], key: &F)
where
    F: Fn(&T) -> K,
    K: Ord + Clone,
{
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let pivot_index = partition_by(arr, key);
    quick_sort_by(&mut arr[0..pivot_index], key);
    quick_sort_by(&mut arr[pivot_index + 1..len], key);
}

fn partition_by<T, F, K>(arr: &mut [T], key: &F) -> usize
where
    F: Fn(&T) -> K,
    K: Ord + Clone,
{
    let len = arr.len();
    let pivot_index = len - 1;
    let pivot = key(&arr[pivot_index]).clone();

    let mut i = 0;
    for j in 0..pivot_index {
        if key(&arr[j]) <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_index);
    i
}
