use once_cell::sync::Lazy;

use crate::board::Position;

pub(super) static I_SHAPE: Lazy<[[Position; 4]; 4]> = Lazy::new(|| {
    [
        [
            Position { x: 0, y: 1 },
            Position { x: 1, y: 1 },
            Position { x: 2, y: 1 },
            Position { x: 3, y: 1 },
        ],
        [
            Position { x: 2, y: 0 },
            Position { x: 2, y: 1 },
            Position { x: 2, y: 2 },
            Position { x: 2, y: 3 },
        ],
        [
            Position { x: 0, y: 2 },
            Position { x: 1, y: 2 },
            Position { x: 2, y: 2 },
            Position { x: 3, y: 2 },
        ],
        [
            Position { x: 1, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 1, y: 2 },
            Position { x: 1, y: 3 },
        ],
    ]
});

pub(super) static J_SHAPE: Lazy<[[Position; 4]; 4]> = Lazy::new(|| {
    [
        [
            Position { x: 0, y: 0 },
            Position { x: 0, y: 1 },
            Position { x: 1, y: 1 },
            Position { x: 2, y: 1 },
        ],
        [
            Position { x: 1, y: 0 },
            Position { x: 2, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 1, y: 2 },
        ],
        [
            Position { x: 0, y: 1 },
            Position { x: 1, y: 1 },
            Position { x: 2, y: 1 },
            Position { x: 2, y: 2 },
        ],
        [
            Position { x: 1, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 0, y: 2 },
            Position { x: 1, y: 2 },
        ],
    ]
});

pub(super) static L_SHAPE: Lazy<[[Position; 4]; 4]> = Lazy::new(|| {
    [
        [
            Position { x: 2, y: 0 },
            Position { x: 0, y: 1 },
            Position { x: 1, y: 1 },
            Position { x: 2, y: 1 },
        ],
        [
            Position { x: 1, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 1, y: 2 },
            Position { x: 2, y: 2 },
        ],
        [
            Position { x: 0, y: 1 },
            Position { x: 1, y: 1 },
            Position { x: 2, y: 1 },
            Position { x: 0, y: 2 },
        ],
        [
            Position { x: 0, y: 0 },
            Position { x: 1, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 1, y: 2 },
        ],
    ]
});

pub(super) static O_SHAPE: Lazy<[[Position; 4]; 4]> = Lazy::new(|| {
    [
        [
            Position { x: 1, y: 0 },
            Position { x: 2, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 2, y: 1 },
        ],
        [
            Position { x: 1, y: 0 },
            Position { x: 2, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 2, y: 1 },
        ],
        [
            Position { x: 1, y: 0 },
            Position { x: 2, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 2, y: 1 },
        ],
        [
            Position { x: 1, y: 0 },
            Position { x: 2, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 2, y: 1 },
        ],
    ]
});

pub(super) static S_SHAPE: Lazy<[[Position; 4]; 4]> = Lazy::new(|| {
    [
        [
            Position { x: 1, y: 0 },
            Position { x: 2, y: 0 },
            Position { x: 0, y: 1 },
            Position { x: 1, y: 1 },
        ],
        [
            Position { x: 1, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 2, y: 1 },
            Position { x: 2, y: 2 },
        ],
        [
            Position { x: 1, y: 1 },
            Position { x: 2, y: 1 },
            Position { x: 0, y: 2 },
            Position { x: 1, y: 2 },
        ],
        [
            Position { x: 0, y: 0 },
            Position { x: 0, y: 1 },
            Position { x: 1, y: 1 },
            Position { x: 1, y: 2 },
        ],
    ]
});

pub(super) static T_SHAPE: Lazy<[[Position; 4]; 4]> = Lazy::new(|| {
    [
        [
            Position { x: 1, y: 0 },
            Position { x: 0, y: 1 },
            Position { x: 1, y: 1 },
            Position { x: 2, y: 1 },
        ],
        [
            Position { x: 1, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 2, y: 1 },
            Position { x: 1, y: 2 },
        ],
        [
            Position { x: 0, y: 1 },
            Position { x: 1, y: 1 },
            Position { x: 2, y: 1 },
            Position { x: 1, y: 2 },
        ],
        [
            Position { x: 1, y: 0 },
            Position { x: 0, y: 1 },
            Position { x: 1, y: 1 },
            Position { x: 1, y: 2 },
        ],
    ]
});

pub(super) static Z_SHAPE: Lazy<[[Position; 4]; 4]> = Lazy::new(|| {
    [
        [
            Position { x: 0, y: 0 },
            Position { x: 1, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 2, y: 1 },
        ],
        [
            Position { x: 2, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 2, y: 1 },
            Position { x: 1, y: 2 },
        ],
        [
            Position { x: 0, y: 1 },
            Position { x: 1, y: 1 },
            Position { x: 1, y: 2 },
            Position { x: 2, y: 2 },
        ],
        [
            Position { x: 1, y: 0 },
            Position { x: 0, y: 1 },
            Position { x: 1, y: 1 },
            Position { x: 0, y: 2 },
        ],
    ]
});
