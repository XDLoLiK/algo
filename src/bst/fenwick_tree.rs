extern crate num;

use num::Zero;

use std::ops::{AddAssign, Sub};

#[derive(Debug, Default, Clone)]
pub struct FenwickTree<T>
where
    T: Sub<Output = T> + AddAssign + Zero + Copy,
{
    size: usize,
    data: Vec<T>,
}

impl<T> FenwickTree<T>
where
    T: Sub<Output = T> + AddAssign + Zero + Copy,
{
    fn op(index: isize) -> isize {
        index & (index + 1)
    }

    /// O(1) time\
    /// O(n) space
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            size: capacity,
            data: vec![T::zero(); capacity],
        }
    }

    /// O(n) time\
    /// O(n) space
    pub fn build(array: &[T]) -> Self {
        let size = array.len();
        let mut data = vec![T::zero(); size];

        for i in 0..size {
            data[i] += array[i];
            let next = i | (i + 1);

            if next < size {
                let diff = data[i];
                data[next] += diff;
            }
        }

        Self { size, data }
    }

    // O(log n) time
    fn prefix_sum(&self, mut right: isize) -> T {
        let mut ans = T::zero();

        while right >= 0 {
            ans += self.data[right as usize];
            right = Self::op(right) - 1;
        }

        ans
    }

    // O(log n) time
    pub fn sum(&self, left: usize, right: usize) -> T {
        assert!(left < self.size);
        assert!(right < self.size);

        self.prefix_sum(right as isize) - self.prefix_sum(left as isize - 1)
    }

    // O(log n) time
    fn add(&mut self, mut index: usize, value: T) {
        while index < self.size {
            self.data[index] += value;
            index = index | (index + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fenwick_tree_unit_1() {
        let mut ft = FenwickTree::build(&[0; 5]);

        ft.add(1, 1);
        ft.add(2, 2);
        ft.add(3, 1);

        assert_eq!(ft.sum(0, 0), 0);
        assert_eq!(ft.sum(1, 1), 1);
        assert_eq!(ft.sum(2, 2), 2);
        assert_eq!(ft.sum(3, 3), 1);
        assert_eq!(ft.sum(4, 4), 0);
        assert_eq!(ft.sum(0, 4), 4);
    }
}
