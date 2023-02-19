use std::{error::Error, io, sync::mpsc, thread, time::Duration};

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

    let (tx, rx) = mpsc::channel();
    let render_handler = thread::spawn(move || {
        let mut last_frame = init_frame();
        let mut stdout = io::stdout();
        render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render(&mut stdout, &curr_frame, &last_frame, false);
            last_frame = curr_frame;
        }
    });

    'game_play: loop {
        let curr_frame = init_frame();

        // input
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
        let _ = tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    terminal::disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;
    drop(render_handler);
    Ok(())
}
