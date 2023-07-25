use std::collections::VecDeque;

use crate::mino::shape::Shape;
use rand::prelude::*;

const QUEUE_SIZE: usize = 6;

pub struct NextQueue {
    queue: VecDeque<Shape>,
    bag: RandomBag,
}

impl Default for NextQueue {
    fn default() -> Self {
        let mut bag = RandomBag::new();
        let queue = (0..QUEUE_SIZE).map(|_| bag.pop()).collect();

        Self { queue, bag }
    }
}

impl NextQueue {
    pub fn pop(&mut self) -> Shape {
        self.queue.push_back(self.bag.pop());

        self.queue.pop_front().unwrap()
    }

    pub fn queue(&self) -> &VecDeque<Shape> {
        &self.queue
    }
}

struct RandomBag(Vec<Shape>);

impl RandomBag {
    pub fn new() -> Self {
        let mut bag = Self(Vec::with_capacity(Shape::COUNT));
        bag.fill();

        bag
    }

    pub fn pop(&mut self) -> Shape {
        if self.0.is_empty() {
            self.fill();
        }

        self.0.pop().unwrap()
    }

    fn fill(&mut self) {
        let mut rng = thread_rng();

        self.0 = vec![
            Shape::I,
            Shape::J,
            Shape::L,
            Shape::O,
            Shape::S,
            Shape::T,
            Shape::Z,
        ];
        self.0.shuffle(&mut rng);
    }
}

impl Default for RandomBag {
    fn default() -> Self {
        Self::new()
    }
}
