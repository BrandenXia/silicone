use std::sync::{Arc, mpsc::Sender};

use crate::handler_default_new;
use crate::image::display_img;
use crate::state::{Event, Handler, State};

pub struct RenderHandler;

impl Handler for RenderHandler {
    handler_default_new!();

    fn thread(&self, state: Arc<State>, _: Sender<Event>) -> anyhow::Result<()> {
        let browser = &state.browser;

        if let Ok(data) = browser.capture_screenshot() {
            display_img(&data)?;
        }

        Ok(())
    }

    fn deps(&self) -> &[Event] {
        &[Event::RefreshScreen]
    }
}
