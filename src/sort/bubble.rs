use std::cmp::Ord;

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
    fn bubble_sort_unit_test_1() {
        let mut array = [5, 1, 2, 345, 35, 12, 1];
        bubble_sort(&mut array);

        assert_eq!(array, [1, 1, 2, 5, 12, 35, 345]);
    }
}
