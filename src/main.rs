use std::sync::Arc;
use std::thread;

use anyhow::Result;

use silicone::browser::browser_thread;
use silicone::image::display_img;
use silicone::state::State;

fn main() -> Result<()> {
    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )?;

    let state = State::new();

    let signal_state = Arc::clone(&state);
    ctrlc::set_handler(move || {
        let mut ended = signal_state.ended.lock().unwrap();
        *ended = true;
    })?;

    let browser_state = Arc::clone(&state);
    thread::spawn(move || {
        browser_thread(browser_state).unwrap();
    });

    let (lock, cvar) = &state.started;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    while let Ok(data) = state.buf.read() {
        display_img(data.as_slice())?;
        thread::sleep(std::time::Duration::from_secs(1));

        if *state.ended.lock().unwrap() {
            break;
        }
    }

    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::cursor::Show
    )?;

    Ok(())
}
