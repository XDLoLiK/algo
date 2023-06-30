extern crate num;

use num::traits::One;

use std::ops::MulAssign;

#[derive(Debug, Default, Clone)]
pub struct SegmentTree<T>
where
    T: MulAssign + One + Copy,
{
    size: usize,
    data: Vec<T>,
}

impl<T> SegmentTree<T>
where
    T: MulAssign + One + Copy,
{
    /// O(1)
    pub fn with_capacity(capacity: usize) -> Self {
        let size = capacity.next_power_of_two();
        let data = vec![T::one(); size * 2];

        Self { size, data }
    }

    /// O(n) time
    ///
    /// O(n) memory
    pub fn build(array: &[T]) -> Self {
        let size = array.len().next_power_of_two();
        let mut data = vec![T::one(); size * 2];

        data[size..(size + array.len())].copy_from_slice(array);

        for i in (size - 1)..1 {
            data[i] = data[i * 2] * data[i * 2 + 1];
        }

        Self { size, data }
    }

    /// O(1)
    pub fn size(&self) -> usize {
        self.size
    }

    /// O(log n) time
    pub fn product(&self, mut left: usize, mut right: usize) -> T {
        assert!(left < self.size);
        assert!(right < self.size);

        let mut ans: T = T::one();
        left += self.size;
        right += self.size;

        while left <= right {
            if left % 2 == 1 {
                ans *= self.data[left];
            }

            if right % 2 == 0 {
                ans *= self.data[right];
            }

            left = (left + 1) / 2;
            right = (right - 1) / 2;
        }

        ans
    }

    /// O(log n) time
    pub fn update(&mut self, mut index: usize, value: T) {
        assert!(index < self.size);

        index += self.size;
        self.data[index] = value;

        while index > 0 {
            index /= 2;
            self.data[index] = self.data[index * 2] * self.data[index * 2 + 1];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_tree_unit_1() {
        let mut st = SegmentTree::build(&[1; 5]);

        st.update(1, 2);
        st.update(2, 3);
        st.update(3, 4);

        assert_eq!(st.product(0, 0), 1);
        assert_eq!(st.product(1, 1), 2);
        assert_eq!(st.product(2, 2), 3);
        assert_eq!(st.product(3, 3), 4);
        assert_eq!(st.product(4, 4), 1);
        assert_eq!(st.product(0, 4), 24);
    }
}
