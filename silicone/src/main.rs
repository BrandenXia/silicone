use std::sync::{Arc, mpsc};
use std::thread;

use anyhow::Result;

use silicone::handlers;
use silicone::state::{Event, Handlers, State};

fn main() -> Result<()> {
    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )?;

    let mut handlers = Handlers::new();
    let state = Arc::new(State::new());
    let (tx, rx) = mpsc::channel::<Event>();

    handlers.add_handler::<handlers::RenderHandler>();
    handlers.add_handler::<handlers::InputHandler>();

    // tx.send(Event::Start).expect("Failed to send start event");
    tx.send(Event::RefreshScreen)
        .expect("Failed to send refresh event");

    while let Ok(msg) = rx.recv() {
        if msg == Event::End {
            break;
        }

        let hs = handlers.get_handlers(msg);
        for h in hs {
            let sender = tx.clone();
            let thread_state = Arc::clone(&state);
            thread::spawn({
                let handler = Arc::clone(h);
                move || handler.thread(thread_state, sender)
            });
        }
    }

    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::cursor::Show
    )?;

    Ok(())
}
