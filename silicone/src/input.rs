use std::sync::{Arc, mpsc::Sender};

use crossterm::event;
use crossterm::event::Event;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};

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
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stdout(), EnableMouseCapture)?;

        let browser = &state.browser;
        loop {
            match event::read()? {
                Event::Key(k) => {
                    if let event::KeyCode::Char(c) = k.code {
                        if c == 'c' && k.modifiers == event::KeyModifiers::CONTROL {
                            tx.send(state::Event::End)?;
                            break;
                        }
                    }
                    browser.handle_key(k)?;
                }

                _ => continue,
            }

            tx.send(state::Event::RefreshScreen)?;
        }

        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(std::io::stdout(), DisableMouseCapture)?;

        Ok(())
    }
}
