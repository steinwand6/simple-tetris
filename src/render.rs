use std::io::{Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, curr_frame: &Frame, last_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }
    for (y, col) in curr_frame.iter().enumerate() {
        for (x, s) in col.iter().enumerate() {
            if *s != last_frame[y][x] || force {
                stdout.queue(MoveTo(y as u16, x as u16)).unwrap();
                print!("{}", *s);
            }
            if *s == "@" {
                stdout.queue(MoveTo(y as u16, x as u16)).unwrap();
                print!("{}", *s);
            }
        }
    }
    stdout.flush().unwrap();
}
