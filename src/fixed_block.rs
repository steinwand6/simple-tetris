use crate::frame::Drawable;

pub type FixedBlock = Vec<(usize, usize)>;

impl Drawable for FixedBlock {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for (x, y) in self.iter() {
            frame[*x][*y] = "@"
        }
    }
}
