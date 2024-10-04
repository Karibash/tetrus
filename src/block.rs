use crate::color::Color;

struct Block {
    color: Color,
    shape: [[bool; 4]; 4],
}

impl Block {
    fn i_block() -> Self {
        Self {
            color: Color::Cyan,
            shape: [
                [false, false, false, false],
                [false, false, false, false],
                [true, true, true, true],
                [false, false, false, false],
            ],
        }
    }

    fn j_block() -> Self {
        Self {
            color: Color::Blue,
            shape: [
                [false, false, false, false],
                [true, false, false, false],
                [true, true, true, false],
                [false, false, false, false],
            ],
        }
    }

    fn l_block() -> Self {
        Self {
            color: Color::Orange,
            shape: [
                [false, false, false, false],
                [false, false, true, false],
                [true, true, true, false],
                [false, false, false, false],
            ],
        }
    }

    fn o_block() -> Self {
        Self {
            color: Color::Yellow,
            shape: [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
        }
    }

    fn s_block() -> Self {
        Self {
            color: Color::Green,
            shape: [
                [false, false, false, false],
                [false, true, true, false],
                [true, true, false, false],
                [false, false, false, false],
            ],
        }
    }

    fn t_block() -> Self {
        Self {
            color: Color::Purple,
            shape: [
                [false, false, false, false],
                [false, true, false, false],
                [true, true, true, false],
                [false, false, false, false],
            ],
        }
    }

    fn z_block() -> Self {
        Self {
            color: Color::Red,
            shape: [
                [false, false, false, false],
                [true, true, false, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
        }
    }
}
