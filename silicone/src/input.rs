use std::sync::{Arc, mpsc::Sender};
use std::thread;
use std::time::Duration;

use crossterm::event;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, poll};

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

        loop {
            if poll(Duration::from_millis(1000))? {
                match event::read()? {
                    Event::Key(k) => {
                        if let event::KeyCode::Char(c) = k.code {
                            if c == 'c' && k.modifiers == event::KeyModifiers::CONTROL {
                                tx.send(state::Event::End)?;
                                break;
                            }
                        }
                        thread::spawn({
                            let s = Arc::clone(&state);
                            move || s.browser.handle_key(k)
                        });
                    }

                    Event::Mouse(m) => {
                        let size = state.cr_size.read().unwrap();
                        let size = (size.0, size.1);
                        let term_size = state.term_size;
                        thread::spawn({
                            let s = Arc::clone(&state);
                            move || s.browser.handle_mouse(m, size, term_size)
                        });
                    }

                    Event::Resize(c, r) => *state.cr_size.write().unwrap() = (c, r),

                    _ => {}
                }
            }

            tx.send(state::Event::RefreshScreen)?;
        }

        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(std::io::stdout(), DisableMouseCapture)?;

        Ok(())
    }
}
