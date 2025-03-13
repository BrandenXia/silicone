use std::sync::{Arc, mpsc::Sender};

use crate::image::display_img;
use crate::state::{Event, Handler, State};

pub struct RenderHandler;

impl Handler for RenderHandler {
    fn new() -> Self {
        Self
    }

    fn thread(&self, state: Arc<State>, _: Sender<Event>) -> anyhow::Result<()> {
        if let Ok(data) = state.buf.read() {
            display_img(data.as_slice())?;
        }

        Ok(())
    }

    fn deps(&self) -> &[Event] {
        &[Event::RefreshScreen]
    }
}
