use std::cmp::Ord;

use num::range_inclusive;

/// O(n log n) time (worst)\
/// O(n) time (best)\
/// O(n log n) time (average)\
/// O(1) space\
/// Not stable
pub fn heap_sort<T>(array: &mut [T])
where
    T: Ord,
{
    let size = array.len();

    for i in range_inclusive(0, size / 2 - 1).rev() {
        heapify(array, size, i);
    }

    for i in range_inclusive(0, size - 1).rev() {
        array.swap(0, i);
        heapify(array, i, 0);
    }
}

fn heapify<T>(array: &mut [T], size: usize, index: usize)
where
    T: Ord,
{
    let mut largest = index;
    let left = index * 2 + 1;
    let right = index * 2 + 2;

    if left < size && array[left] > array[largest] {
        largest = left;
    }

    if right < size && array[right] > array[largest] {
        largest = right;
    }

    if largest != index {
        array.swap(index, largest);
        heapify(array, size, largest);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_sort_numbers_test_1() {
        let mut array = [5, 1, 2, 345, 35, 12, 1];
        heap_sort(&mut array);

        assert_eq!(array, [1, 1, 2, 5, 12, 35, 345]);
    }

    #[test]
    fn heap_sort_string_test_1() {
        let mut array = ["aa", "bbc", "bab", "ca", "bac", "bac", "a"];
        heap_sort(&mut array);

        assert_eq!(array, ["a", "aa", "bab", "bac", "bac", "bbc", "ca"]);
    }
}
