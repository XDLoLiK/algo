use std::cmp::Ord;

/// O(n^2) time (worst)\
/// O(n) time (best)\
/// O(n^2) time (average)\
/// O(1) spacce\
/// Stable
pub fn bubble_sort<T>(array: &mut [T])
where
    T: Ord,
{
    let size = array.len();

    for i in 0..size {
        for j in (i + 1)..size {
            if array[i] > array[j] {
                array.swap(i, j)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_numbers_test_1() {
        let mut array = [5, 1, 2, 345, 35, 12, 1];
        bubble_sort(&mut array);

        assert_eq!(array, [1, 1, 2, 5, 12, 35, 345]);
    }

    #[test]
    fn bubble_sort_string_test_1() {
        let mut array = ["aa", "bbc", "bab", "ca", "bac", "bac", "a"];
        bubble_sort(&mut array);

        assert_eq!(array, ["a", "aa", "bab", "bac", "bac", "bbc", "ca"]);
    }
}
