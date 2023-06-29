extern crate num;

use num::Zero;

use std::ops::AddAssign;

#[derive(Debug, Default, Clone)]
pub struct SegmentTree<T>
where
    T: AddAssign + Zero + Copy,
{
    size: usize,
    data: Vec<T>,
}

impl<T> SegmentTree<T>
where
    T: AddAssign + Zero + Copy,
{
    /// O(1)
    pub fn with_capacity(capacity: usize) -> Self {
        let size = capacity.next_power_of_two();
        let data = vec![T::zero(); size * 2];
        Self { size, data }
    }

    /// O(n) time
    ///
    /// O(n) memory
    ///
    /// Where n is array.len().next_power_of_two()
    pub fn build(array: &[T]) -> Self {
        let size = array.len().next_power_of_two();
        let mut data = vec![T::zero(); size * 2];

        for i in 0..array.len() {
            data[size + i] = array[i];
        }

        for i in (size - 1)..1 {
            data[i] = data[i * 2] + data[i * 2 + 1];
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
        let mut ans: T = T::zero();
        left += self.size;
        right += self.size;

        while left <= right {
            if left % 2 == 1 {
                ans += self.data[left];
            }

            if right % 2 == 0 {
                ans += self.data[right];
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
        self.data[index] = value;

        while index > 0 {
            index /= 2;
            self.data[index] = self.data[index * 2] + self.data[index * 2 + 1];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_tree_unit_1() {
        let mut st = SegmentTree::build(&[0; 5]);

        st.update(2, 2);
        st.update(3, 1);
        st.update(4, 2);

        assert_eq!(st.sum(1, 1), 0);
        assert_eq!(st.sum(2, 2), 2);
        assert_eq!(st.sum(3, 3), 1);
        assert_eq!(st.sum(4, 4), 2);
        assert_eq!(st.sum(5, 5), 0);
        assert_eq!(st.sum(1, 5), 5);
    }
}
