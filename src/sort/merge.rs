use std::cmp::{Ord, Ordering};

pub fn merge_sort<T>(array: &mut [T])
where
    T: Ord + Default,
{
    merge_sort_helper(array, 0, array.len());
}

fn merge_sort_helper<T>(array: &mut [T], left: usize, right: usize)
where
    T: Ord + Default,
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
    T: Ord + Default,
{
    let mut first_iter = 0;
    let mut second_iter = 0;
    let mut result = vec![T::default(); right - left];

    while left + first_iter < mid && mid + second_iter < right {
        match array[left + first_iter].cmp(array[mid + second_iter]) {
            Ordering::Less => {
                result[first_iter + second_iter] = array[left + first_iter];
                first_iter += 1;
            },
            _ => {
                result[first_iter + second_iter] = array[mid + second_iter];
                second_iter += 1;
            },
        }
    }

    while left + first_iter < mid {
        result[first_iter + second_iter] = array[left + first_iter];
        first_iter += 1;
    }

    while right + second_iter < right {
        result[first_iter + second_iter] = array[right + second_iter];
        second_iter += 1;
    }

    for i in 0..(first_iter + second_iter) {
        array[left + i] = result[i];
    }
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
