use std::{
    error::Error,
    io,
    sync::mpsc,
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use crossterm::{
    cursor::Hide,
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use simple_tetris::{
    fixed_block::FixedBlock,
    frame::{init_frame, Drawable, Frame},
    render::render,
    tetrimino::Tetrimino,
};

fn main() -> Result<(), Box<dyn Error>> {
    // terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let (tx, rx) = mpsc::channel();
    let render_handler = render_hander(rx);
    game_play(tx)?;

    // cleanup
    terminal::disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;
    drop(render_handler);
    Ok(())
}

fn game_play(tx: mpsc::Sender<Frame>) -> Result<(), Box<dyn Error>> {
    let mut mino = Tetrimino::new();
    let mut instant = Instant::now();
    let mut fixed_block: FixedBlock = vec![];

    loop {
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = init_frame();
        if !mino.moving {
            fixed_block.append(&mut mino.xy);
            mino = Tetrimino::new();
        }
        fixed_block.draw(&mut curr_frame);

        // input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Down => mino.go_down(&curr_frame),
                    KeyCode::Right => mino.go_right(&curr_frame),
                    KeyCode::Left => mino.go_left(&curr_frame),
                    KeyCode::Esc | KeyCode::Char('q') => {
                        return Ok(());
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
}

fn render_hander(rx: mpsc::Receiver<Frame>) -> JoinHandle<()> {
    thread::spawn(move || {
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
    })
}
