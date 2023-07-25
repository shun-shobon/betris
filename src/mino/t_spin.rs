use crate::{field::Field, pos, position::Position};

use super::{shape::Shape, Mino};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TSpin {
    #[default]
    None,
    Mini,
    Full,
}

impl TSpin {
    pub fn update(&mut self, mino: &Mino, field: &Field, delta: Position) {
        *self = if Self::is_t_spin(mino, field) {
            if Self::is_t_spin_mini(mino, field, delta) {
                Self::Mini
            } else {
                Self::Full
            }
        } else {
            Self::None
        };
    }

    // Tミノであり，Tミノの四隅が3箇所以上埋まっているとT-Spin
    // 壁や床は埋まっている扱い
    fn is_t_spin(mino: &Mino, field: &Field) -> bool {
        if mino.shape != Shape::T {
            return false;
        }

        let fullfilled = T_SPIN_CHECK_POSITIONS
            .iter()
            .map(|&pos| pos + mino.pos)
            .filter(|&pos| {
                field
                    .blocks
                    .get(pos)
                    .map_or(true, |block| block.is_filled())
            })
            .count();

        fullfilled >= 3
    }

    // T-Spinであり，回転補正が(±1, ±2)ではなく，Tミノの凸側の隅2箇所が埋まっていないとT-Spin Mini
    fn is_t_spin_mini(mino: &Mino, field: &Field, delta: Position) -> bool {
        if delta.x.abs() == 1 && delta.y.abs() == 2 {
            false
        } else {
            let angle_idx: usize = mino.angle.into();

            !T_SPIN_MINI_CHECK_POSITIONS[angle_idx]
                .iter()
                .map(|&pos| pos + mino.pos)
                .all(|pos| {
                    field
                        .blocks
                        .get(pos)
                        .map_or(true, |block| block.is_filled())
                })
        }
    }
}

static T_SPIN_CHECK_POSITIONS: [Position; 4] = pos![(0, 0), (2, 0), (0, 2), (2, 2)];
static T_SPIN_MINI_CHECK_POSITIONS: [[Position; 2]; 4] = [
    pos![(0, 2), (2, 2)],
    pos![(2, 2), (2, 0)],
    pos![(2, 0), (0, 0)],
    pos![(0, 0), (0, 2)],
];
