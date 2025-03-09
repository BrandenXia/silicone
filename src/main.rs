use std::sync::Arc;

use anyhow::Result;

use silicone::browser::BrowserThread;
use silicone::spawn;
use silicone::state::{State, StateProc};
use silicone::ui::RenderThread;

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

    let mut handles = vec![];

    spawn!(BrowserThread, state, handles);
    spawn!(RenderThread, state, handles);

    for handle in handles {
        handle.join().unwrap()?;
    }

    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::cursor::Show
    )?;

    Ok(())
}
