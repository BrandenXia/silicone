use std::sync::{Arc, mpsc::Sender};

use crate::handler_default_new;
use crate::state::{Event, Handler, State};

pub struct InputHandler;

impl Handler for InputHandler {
    handler_default_new!();

    fn deps(&self) -> &[Event] {
        &[Event::Start]
    }

    fn thread(&self, state: Arc<State>, tx: Sender<Event>) -> anyhow::Result<()> {
        Ok(())
    }
}
