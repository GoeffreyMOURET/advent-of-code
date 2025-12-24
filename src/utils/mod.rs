use crate::utils::array_utils::quick_sort_by;

mod array_utils;

pub struct Utils;

impl Utils {
    pub fn quick_sort_by<T, F, K>(arr: &mut [T], key: &F)
    where
        F: Fn(&T) -> K,
        K: Ord + Clone,
    {
        quick_sort_by(arr, key)
    }
}