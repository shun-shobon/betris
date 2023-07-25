use crate::mino::shape::Shape;
use rand::prelude::*;

pub struct RandomBag(Vec<Shape>);

impl RandomBag {
    pub fn new() -> Self {
        let mut bag = Self(Vec::with_capacity(Shape::COUNT));
        bag.fill();

        bag
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

impl Iterator for RandomBag {
    type Item = Shape;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            self.fill();
        }

        self.0.pop()
    }
}
