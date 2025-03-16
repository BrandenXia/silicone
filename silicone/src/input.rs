use std::sync::{Arc, mpsc::Sender};

use crossterm::event;
use crossterm::event::EnableMouseCapture;
use crossterm::event::Event;

use crate::handler_default_new;
use crate::state;
use crate::state::{Handler, State};

pub struct InputHandler;

impl Handler for InputHandler {
    handler_default_new!();

    fn deps(&self) -> &[state::Event] {
        &[state::Event::Start]
    }

    fn thread(&self, state: Arc<State>, tx: Sender<state::Event>) -> anyhow::Result<()> {
        crossterm::execute!(std::io::stdout(), EnableMouseCapture)?;

        let browser = &state.browser;
        loop {
            match event::read()? {
                Event::Key(k) => browser.handle_key(k)?,
                _ => continue,
            }

            tx.send(state::Event::RefreshScreen)
                .expect("Failed to send refresh event");
        }
    }
}
