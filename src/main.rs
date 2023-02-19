use std::{
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::Hide,
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use simple_tetris::{
    frame::{init_frame, Drawable},
    render::render,
    tetrimino::{minotype, Tetrimino},
};

fn main() -> Result<(), Box<dyn Error>> {
    // terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let (tx, rx) = mpsc::channel();

    let render_handler = thread::spawn(move || {
        let mut stdout = io::stdout();
        let mut last_frame = init_frame();
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

    let mut mino = Tetrimino::new(minotype::T);
    let mut instant = Instant::now();

    'game_play: loop {
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = init_frame();
        if !mino.moving {
            mino = Tetrimino::new(minotype::I);
        }

        // input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Down => mino.go_down(&curr_frame),
                    KeyCode::Right => mino.go_right(&curr_frame),
                    KeyCode::Left => mino.go_left(&curr_frame),
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'game_play;
                    }
                    _ => (),
                }
            }
        }
        mino.update(&curr_frame, delta);
        mino.draw(&mut curr_frame);
        let _ = tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    terminal::disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;
    drop(render_handler);
    Ok(())
}
