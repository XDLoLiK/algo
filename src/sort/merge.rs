use std::cmp::{Ord, Ordering};

pub fn merge_sort<T>(array: &mut [T])
where
    T: Ord + Default + Copy,
{
    merge_sort_helper(array, 0, array.len() - 1);
}

fn merge_sort_helper<T>(array: &mut [T], left: usize, right: usize)
where
    T: Ord + Default + Copy,
{
    if left < right {
        let mid = (left + right) / 2;
        merge_sort_helper(array, left, mid);
        merge_sort_helper(array, mid + 1, right);
        merge(array, left, mid, right);
    }
}

fn merge<T>(array: &mut [T], left: usize, mid: usize, right: usize)
where
    T: Ord + Default + Copy,
{
    let mut first_iter = 0;
    let mut second_iter = 0;
    let size = right - left + 1;
    let mut result = vec![T::default(); size];

    while left + first_iter <= mid && mid + 1 + second_iter <= right {
        match array[left + first_iter].cmp(&array[mid + 1 + second_iter]) {
            Ordering::Less => {
                result[first_iter + second_iter] = array[left + first_iter];
                first_iter += 1;
            }
            _ => {
                result[first_iter + second_iter] = array[mid + 1 + second_iter];
                second_iter += 1;
            }
        }
    }

    while left + first_iter <= mid {
        result[first_iter + second_iter] = array[left + first_iter];
        first_iter += 1;
    }

    while mid + 1 + second_iter <= right {
        result[first_iter + second_iter] = array[mid + 1 + second_iter];
        second_iter += 1;
    }

    array[left..(left + size)].copy_from_slice(&result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_sort_numbers_test_1() {
        let mut array = [5, 1, 2, 345, 35, 12, 1];
        merge_sort(&mut array);

        assert_eq!(array, [1, 1, 2, 5, 12, 35, 345]);
    }

    #[test]
    fn merge_sort_string_test_1() {
        let mut array = ["aa", "bbc", "bab", "ca", "bac", "bac", "a"];
        merge_sort(&mut array);

        assert_eq!(array, ["a", "aa", "bab", "bac", "bac", "bbc", "ca"]);
    }
}
