use crate::{frame::Drawable, NUM_ROWS};

pub enum minotype {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

pub struct Tetrimino {
    pub xy: Vec<(usize, usize)>,
    pub moving: bool,
}

impl Tetrimino {
    pub fn new(minotype: minotype) -> Self {
        Self {
            xy: Self::generate_mino(minotype),
            moving: true,
        }
    }

    fn generate_mino(minotype: minotype) -> Vec<(usize, usize)> {
        let left_x = (NUM_ROWS) / 2 - 1;
        let top_y = 0;
        match minotype {
            minotype::I => vec![
                (left_x, top_y),
                (left_x, top_y + 1),
                (left_x, top_y + 2),
                (left_x, top_y + 3),
            ],
            minotype::O => vec![
                (left_x, top_y),
                (left_x + 1, top_y),
                (left_x, top_y + 1),
                (left_x + 1, top_y + 1),
            ],
            minotype::S => vec![
                (left_x + 1, top_y),
                (left_x + 2, top_y),
                (left_x, top_y + 1),
                (left_x + 1, top_y + 1),
            ],
            minotype::Z => vec![
                (left_x, top_y),
                (left_x + 1, top_y),
                (left_x + 1, top_y + 1),
                (left_x + 2, top_y + 1),
            ],
            minotype::J => vec![
                (left_x + 1, top_y),
                (left_x + 1, top_y + 1),
                (left_x + 1, top_y + 2),
                (left_x, top_y + 2),
            ],
            minotype::L => vec![
                (left_x, top_y),
                (left_x, top_y + 1),
                (left_x, top_y + 2),
                (left_x + 1, top_y + 2),
            ],
            minotype::T => vec![
                (left_x + 1, top_y),
                (left_x, top_y + 1),
                (left_x + 1, top_y + 1),
                (left_x + 2, top_y + 1),
            ],
        }
    }
}

impl Drawable for Tetrimino {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for (x, y) in self.xy.iter() {
            frame[*x][*y] = "@";
        }
    }
}
