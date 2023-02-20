use crate::{frame::Drawable, NUM_COLS, NUM_ROWS};

pub type FixedBlock = Vec<(usize, usize)>;

pub fn clear_row(block: FixedBlock, target_row: usize) -> FixedBlock {
    block
        .into_iter()
        .filter(|(_, y)| *y != target_row)
        .map(|(x, y)| if y < target_row { (x, y + 1) } else { (x, y) })
        .collect()
}

pub fn update_block(block: &mut FixedBlock) {
    for i in 0..NUM_ROWS {
        if block.iter().filter(|(_, y)| *y == i).count() == NUM_COLS {
            *block = clear_row(block.to_vec(), i);
        }
    }
}

impl Drawable for FixedBlock {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for (x, y) in self.iter() {
            frame[*x][*y] = "@"
        }
    }
}
