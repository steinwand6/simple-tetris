use std::{error::Error, io, time::Duration};

use crossterm::{
    cursor::Hide,
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use simple_tetris::{frame::init_frame, render::render};

fn main() -> Result<(), Box<dyn Error>> {
    // terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let frame = init_frame();
    render(&mut stdout, &frame, &frame, true);
    'game_play: loop {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'game_play;
                    }
                    _ => (),
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;
    Ok(())
}
