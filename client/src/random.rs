use crate::mino::shape::MinoShape;
use rand::prelude::*;

pub struct RandomBag(Vec<MinoShape>);

impl RandomBag {
    pub fn new() -> Self {
        let mut bag = Self(Vec::with_capacity(MinoShape::COUNT));
        bag.fill();

        bag
    }

    fn fill(&mut self) {
        let mut rng = thread_rng();

        self.0 = vec![
            MinoShape::I,
            MinoShape::J,
            MinoShape::L,
            MinoShape::O,
            MinoShape::S,
            MinoShape::T,
            MinoShape::Z,
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
    type Item = MinoShape;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            self.fill();
        }

        self.0.pop()
    }
}
