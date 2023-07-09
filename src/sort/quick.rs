use std::cmp::{Ord, Ordering};

pub fn quick_sort<T>(array: &mut [T])
where
    T: Ord + Copy,
{
    quick_sort_helper(array, 0, array.len() - 1);
}

fn quick_sort_helper<T>(array: &mut [T], left: usize, right: usize)
where
    T: Ord + Copy,
{
    if left < right {
        let pivot = find_pivot(array, left, right);
        let (new_left, new_right) = find_partition(array, pivot, left, right);
        quick_sort_helper(array, left, new_left.saturating_sub(1));
        quick_sort_helper(array, new_right, right);
    }
}

fn find_pivot<T>(array: &[T], left: usize, right: usize) -> T
where
    T: Ord + Copy,
{
    array[(left + right) / 2]
}

fn find_partition<T>(array: &mut [T], pivot: T, mut left: usize, mut right: usize) -> (usize, usize)
where
    T: Ord + Copy,
{
    let mut upper_bound = right;
    right = left;

    while right <= upper_bound {
        match pivot.cmp(&array[right]) {
            Ordering::Less => {
                array.swap(right, upper_bound);
                // Can't overflow
                upper_bound -= 1;
            }
            Ordering::Greater => {
                array.swap(left, right);
                left += 1;
                right += 1;
            }
            Ordering::Equal => right += 1,
        }
    }

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_sort_unit_test_1() {
        let mut array = [5, 1, 2, 345, 35, 12, 1];
        quick_sort(&mut array);

        assert_eq!(array, [1, 1, 2, 5, 12, 35, 345]);
    }
}