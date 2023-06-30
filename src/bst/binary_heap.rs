extern crate num;

use num::traits::Bounded;

use std::cmp::{max, min};

fn parent(pos: usize) -> usize {
    (pos - 1) / 2
}

fn left(pos: usize) -> usize {
    pos * 2 + 1
}

fn right(pos: usize) -> usize {
    pos * 2 + 2
}

pub struct MinHeap<T>
where
    T: Bounded + Ord + Copy,
{
    size: usize,
    capacity: usize,
    data: Vec<T>,
}

impl<T> MinHeap<T>
where
    T: Bounded + Ord + Copy,
{
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            size: 0,
            capacity,
            data: vec![T::max_value(); capacity],
        }
    }

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

    pub fn decrease_key(&mut self, mut index: usize, value: T) {
        assert!(value < self.data[index]);

        self.data[index] = value;

        while index != 0 && self.data[parent(index)] > self.data[index] {
            self.data.swap(index, parent(index));
            index = parent(index);
        }
    }

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

    pub fn delete_key(&mut self, index: usize) {
        self.decrease_key(index, T::min_value());
        self.extract_min();
    }

    pub fn heapify(&mut self, index: usize) {
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
}
