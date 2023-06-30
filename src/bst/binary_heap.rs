extern crate num;

use num::traits::Bounded;

use std::ops::{SubAssign, AddAssign};

/// O(1)
fn parent(pos: usize) -> usize {
    (pos - 1) / 2
}

/// O(1)
fn left(pos: usize) -> usize {
    pos * 2 + 1
}

/// O(1)
fn right(pos: usize) -> usize {
    pos * 2 + 2
}

pub struct MinHeap<T>
where
    T: SubAssign + Bounded + Ord + Copy,
{
    size: usize,
    capacity: usize,
    data: Vec<T>,
}

impl<T> MinHeap<T>
where
    T: SubAssign + Bounded + Ord + Copy,
{
    /// O(1) time
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            size: 0,
            capacity,
            data: vec![T::max_value(); capacity],
        }
    }

    /// O(n log n)
    pub fn build(array: &[T]) -> Self {
        let mut new = Self::with_capacity(array.len());

        for item in array {
            new.insert_key(*item);
        }

        new
    }

    /// O(log n) time
    pub fn insert_key(&mut self, key: T) {
        if self.size >= self.capacity {
            return;
        }

        let mut curr = self.size;
        self.data[curr] = key;
        self.size += 1;

        while curr != 0 && self.data[parent(curr)] > self.data[curr] {
            self.data.swap(curr, parent(curr));
            curr = parent(curr);
        }
    }

    /// O(log n) time
    pub fn decrease_key(&mut self, mut index: usize, value: T) {
        assert!(self.data[index] >= value);

        self.data[index] = value;

        while index != 0 && self.data[parent(index)] > self.data[index] {
            self.data.swap(index, parent(index));
            index = parent(index);
        }
    }

    /// O(log n) time
    fn heapify(&mut self, index: usize) {
        let left = left(index);
        let right = right(index);

        let mut smallest = index;

        if left < self.size && self.data[left] < self.data[index] {
            smallest = left;
        }

        if right < self.size && self.data[right] < self.data[smallest] {
            smallest = right;
        }

        if smallest != index {
            self.data.swap(index, smallest);
            self.heapify(smallest);
        }
    }

    /// O(1) time
    pub fn get_min(&self) -> T {
        self.data[0]
    }

    /// O(log n) time
    pub fn extract_min(&mut self) -> T {
        if self.size == 0 {
            return T::max_value();
        }

        let min = self.data[0];

        self.data[0] = self.data[self.size - 1];
        self.size -= 1;
        self.heapify(0);

        min
    }

    /// O(log n) time
    pub fn delete_key(&mut self, index: usize) {
        self.decrease_key(index, T::min_value());
        self.extract_min();
    }
}

pub struct MaxHeap<T>
where
    T: AddAssign + Bounded + Ord + Copy,
{
    size: usize,
    capacity: usize,
    data: Vec<T>,
}

impl<T> MaxHeap<T>
where
    T: AddAssign + Bounded + Ord + Copy,
{
    /// O(1) time
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            size: 0,
            capacity,
            data: vec![T::min_value(); capacity],
        }
    }

    /// O(n log n)
    pub fn build(array: &[T]) -> Self {
        let mut new = Self::with_capacity(array.len());

        for item in array {
            new.insert_key(*item);
        }

        new
    }

    /// O(log n) time
    pub fn insert_key(&mut self, key: T) {
        if self.size >= self.capacity {
            return;
        }

        let mut curr = self.size;
        self.data[curr] = key;
        self.size += 1;

        while curr != 0 && self.data[parent(curr)] < self.data[curr] {
            self.data.swap(curr, parent(curr));
            curr = parent(curr);
        }
    }

    /// O(log n) time
    pub fn increase_key(&mut self, mut index: usize, value: T) {
        assert!(self.data[index] <= value);

        self.data[index] = value;

        while index != 0 && self.data[parent(index)] < self.data[index] {
            self.data.swap(index, parent(index));
            index = parent(index);
        }
    }

    /// O(log n) time
    fn heapify(&mut self, index: usize) {
        let left = left(index);
        let right = right(index);

        let mut biggest = index;

        if left < self.size && self.data[left] > self.data[index] {
            biggest = left;
        }

        if right < self.size && self.data[right] > self.data[biggest] {
            biggest = right;
        }

        if biggest != index {
            self.data.swap(index, biggest);
            self.heapify(biggest);
        }
    }

    /// O(1) time
    pub fn get_max(&self) -> T {
        self.data[0]
    }

    /// O(log n) time
    pub fn extract_max(&mut self) -> T {
        if self.size == 0 {
            return T::min_value();
        }

        let max = self.data[0];

        self.data[0] = self.data[self.size - 1];
        self.size -= 1;
        self.heapify(0);

        max
    }

    /// O(log n) time
    pub fn delete_key(&mut self, index: usize) {
        self.increase_key(index, T::max_value());
        self.extract_max();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_heap_unit_1() {
        let mut heap = MinHeap::<usize>::with_capacity(11);

        heap.insert_key(3);
        heap.insert_key(2);
        heap.delete_key(1);
        heap.insert_key(15);
        heap.insert_key(5);
        heap.insert_key(4);
        heap.insert_key(45);

        assert_eq!(heap.extract_min(), 2);
        assert_eq!(heap.get_min(), 4);

        heap.decrease_key(2, 1);

        assert_eq!(heap.get_min(), 1);
    }

    #[test]
    fn max_heap_unit_1() {
        let mut heap = MaxHeap::<usize>::with_capacity(11);

        heap.insert_key(3);
        heap.insert_key(2);
        heap.delete_key(1);
        heap.insert_key(15);
        heap.insert_key(5);
        heap.insert_key(4);
        heap.insert_key(45);

        assert_eq!(heap.extract_max(), 45);
        assert_eq!(heap.get_max(), 15);

        heap.increase_key(2, 44);

        assert_eq!(heap.get_max(), 44);
    }
}
