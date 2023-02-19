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
        let left_x = NUM_ROWS / 2;
        let top_y = 0;
        Self {
            xy: match minotype {
                minotype::O => vec![
                    (left_x, top_y),
                    (left_x + 1, top_y),
                    (left_x, top_y + 1),
                    (left_x + 1, top_y + 1),
                ],
                _ => unreachable!(),
            },
            moving: true,
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
