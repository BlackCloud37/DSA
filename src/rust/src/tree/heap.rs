use std::fmt::Debug;

use super::BinaryHeap;

/// Implementation binary heap with vec
pub struct VecHeap<T> {
    vec: Vec<T>,
}

impl<T: Ord + Copy + Debug> BinaryHeap<T> for VecHeap<T> {
    fn new() -> Self {
        Self { vec: Vec::new() }
    }

    fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self.vec[0])
        }
    }

    fn len(&self) -> usize {
        self.vec.len()
    }

    fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let result = Some(self.vec[0]);
        let mut curr = 0;
        while curr < self.len() {
            let lc = curr * 2 + 1;
            let rc = curr * 2 + 2;
            if lc >= self.len() && rc >= self.len() {
                break;
            } else if lc < self.len() && rc >= self.len() {
                self.vec[curr] = self.vec[lc];
                curr = lc;
            } else {
                if self.vec[lc] > self.vec[rc] {
                    self.vec[curr] = self.vec[lc];
                    curr = lc;
                } else {
                    self.vec[curr] = self.vec[rc];
                    curr = rc;
                }
            }
        }
        self.vec.remove(curr);
        result
    }

    fn from_vec(vec: Vec<T>) -> Self {
        let mut vec = vec;
        for i in (1..vec.len()).rev() {
            let mut curr = i;
            while curr >= 1 {
                let parent = (curr - 1) / 2;
                if vec[parent] < vec[curr] {
                    let tmp = vec[parent];
                    vec[parent] = vec[curr];
                    vec[curr] = tmp;
                }
                curr = parent;
            }
        }
        println!("from vec: {:?}", vec);
        Self { vec }
    }

    fn push(&mut self, val: T) {
        self.vec.push(val);
        let mut i = self.vec.len() - 1;
        while i >= 1 {
            let parent = (i - 1) / 2;
            if self.vec[parent] < self.vec[i] {
                let tmp = self.vec[parent];
                self.vec[parent] = self.vec[i];
                self.vec[i] = tmp;
            }
            i = parent;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::{heap::VecHeap, BinaryHeap};

    #[test]
    fn vec_heap() {
        let mut heap = VecHeap::new();
        assert_eq!(heap.peek(), None);
        heap.push(1);
        heap.push(5);
        heap.push(2);
        // Now peek shows the most important item in the heap.
        assert_eq!(heap.peek(), Some(&5));
        // We can check the length of a heap.
        assert_eq!(heap.len(), 3);
        // If we instead pop these scores, they should come back in order.
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), None);
    }
}
