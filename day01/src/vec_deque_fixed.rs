use std::{collections::VecDeque, fmt::Debug};

#[derive(Debug)]
pub struct VecDequeFixed<T> {
    pub vec: VecDeque<T>,
    pub max_capacity: usize,
    pub push_front: bool,
}

impl<T> VecDequeFixed<T> {
    pub fn push(&mut self, item: T) {
        if self.vec.len() == self.max_capacity {
            if self.push_front {
                self.vec.pop_back();
            } else {
                self.vec.pop_front();
            }
        }
        if self.push_front {
            self.vec.push_front(item);
        } else {
            self.vec.push_back(item);
        }
    }
}
