use std::time::Duration;

use rusty_time::Timer;

use crate::{
    frame::{Drawable, Frame},
    NUM_COLS, NUM_ROWS,
};

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
    timer: Timer,
}

impl Tetrimino {
    pub fn new(minotype: minotype) -> Self {
        Self {
            xy: Self::generate_mino(minotype),
            moving: true,
            timer: Timer::from_millis(1000),
        }
    }

    pub fn update(&mut self, frame: &Frame, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready {
            self.timer.reset();
            self.go_down(frame);
        }
    }

    pub fn go_down(&mut self, frame: &Frame) {
        if self.moving {
            let new_xy: Vec<(usize, usize)> = self.xy.iter().map(|(x, y)| (*x, *y + 1)).collect();
            let bottom = new_xy.iter().map(|(_, y)| *y).max().unwrap_or(0);
            if bottom > NUM_ROWS - 1 {
                self.moving = false;
                return;
            }
            if new_xy.iter().any(|(x, y)| frame[*x][*y] == "@") {
                return;
            }
            self.xy = new_xy;
        }
    }

    pub fn go_right(&mut self, frame: &Frame) {
        if self.moving {
            let new_xy: Vec<(usize, usize)> = self.xy.iter().map(|(x, y)| (*x + 1, *y)).collect();

            let right = self.xy.iter().map(|(x, _)| *x).max().unwrap_or(0);
            if right >= NUM_COLS - 1 {
                return;
            }
            if new_xy.iter().any(|(x, y)| frame[*x][*y] == "@") {
                return;
            }

            self.xy = new_xy;
        }
    }
    pub fn go_left(&mut self, frame: &Frame) {
        if self.moving {
            let left = self.xy.iter().map(|(x, _)| *x).min().unwrap_or(0);
            if left == 0 {
                return;
            }
            if self.xy.iter().any(|(x, y)| frame[*x][*y] == "@") {
                return;
            }
            let new_xy: Vec<(usize, usize)> = self.xy.iter().map(|(x, y)| (*x - 1, *y)).collect();

            self.xy = new_xy;
        }
    }

    fn generate_mino(minotype: minotype) -> Vec<(usize, usize)> {
        let left_x = (NUM_COLS) / 2 - 1;
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
