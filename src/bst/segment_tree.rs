extern crate num;

use num::{Bounded, One, Zero};

use std::{
    cmp::{max, min},
    ops::{AddAssign, MulAssign},
};

#[derive(Debug, Default, Clone)]
struct Node<T>
where
    T: AddAssign + Zero + MulAssign + One + Ord + Bounded + Copy,
{
    sum: T,
    product: T,
    min: T,
    max: T,
}

impl<T> Node<T>
where
    T: AddAssign + Zero + MulAssign + One + Ord + Bounded + Copy,
{
    fn neutral() -> Self {
        Self {
            sum: T::zero(),
            product: T::one(),
            min: T::min_value(),
            max: T::max_value(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct SegmentTree<T>
where
    T: AddAssign + Zero + MulAssign + One + Ord + Bounded + Copy,
{
    size: usize,
    data: Vec<Node<T>>,
}

impl<T> SegmentTree<T>
where
    T: AddAssign + Zero + MulAssign + One + Ord + Bounded + Copy,
{
    fn update_single(data: &mut Vec<Node<T>>, index: usize, value: T) {
        data[index].sum = value;
        data[index].product = value;
        data[index].max = value;
        data[index].min = value;
    }

    fn propagate(data: &mut Vec<Node<T>>, index: usize) {
        data[index].sum = data[index * 2].sum + data[index * 2 + 1].sum;
        data[index].product = data[index * 2].product * data[index * 2 + 1].product;
        data[index].min = min(data[index * 2].min, data[index * 2 + 1].min);
        data[index].max = max(data[index * 2].max, data[index * 2 + 1].max);
    }

    /// O(1)
    pub fn with_capacity(capacity: usize) -> Self {
        let size = capacity.next_power_of_two();
        let data = vec![Node::<T>::neutral(); size * 2];

        Self { size, data }
    }

    /// O(n) time
    ///
    /// O(n) memory
    ///
    /// Where n is array.len().next_power_of_two()
    pub fn build(array: &[T]) -> Self {
        let size = array.len().next_power_of_two();
        let mut data = vec![Node::<T>::neutral(); size * 2];

        for i in 0..array.len() {
            Self::update_single(&mut data, size + i, array[i]);
        }

        for i in (size - 1)..1 {
            Self::propagate(&mut data, i);
        }

        Self { size, data }
    }

    /// O(1)
    pub fn size(&self) -> usize {
        self.size
    }

    /// O(log n) time
    ///
    /// Where n is self.size
    pub fn sum(&self, mut left: usize, mut right: usize) -> T {
        assert!(left < self.size);
        assert!(right < self.size);

        let mut ans: T = T::zero();
        left += self.size;
        right += self.size;

        while left <= right {
            if left % 2 == 1 {
                ans += self.data[left].sum;
            }

            if right % 2 == 0 {
                ans += self.data[right].sum;
            }

            left = (left + 1) / 2;
            right = (right - 1) / 2;
        }

        ans
    }

    /// O(log n) time
    ///
    /// Where n is self.size
    pub fn update(&mut self, mut index: usize, value: T) {
        assert!(index < self.size);

        index += self.size;
        Self::update_single(&mut self.data, index, value);

        while index > 0 {
            index /= 2;
            Self::propagate(&mut self.data, index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_tree_unit_1() {
        let mut st = SegmentTree::build(&[0; 5]);

        st.update(1, 2);
        st.update(2, 1);
        st.update(3, 2);

        assert_eq!(st.sum(0, 0), 0);
        assert_eq!(st.sum(1, 1), 2);
        assert_eq!(st.sum(2, 2), 1);
        assert_eq!(st.sum(3, 3), 2);
        assert_eq!(st.sum(4, 4), 0);
        assert_eq!(st.sum(0, 4), 5);
    }
}
