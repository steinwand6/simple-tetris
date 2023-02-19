use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<&'static str>>;

pub fn init_frame() -> Frame {
    let mut rows = Vec::with_capacity(NUM_ROWS);
    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_COLS);
        for _ in 0..NUM_ROWS {
            col.push(" ");
        }
        rows.push(col);
    }
    rows
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
