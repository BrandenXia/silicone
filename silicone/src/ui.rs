use std::sync::{Arc, mpsc::Sender};

use crate::image::display_img;
use crate::state::{Event, Handler, State};

pub struct RenderHandler;

impl Handler for RenderHandler {
    fn new() -> Self {
        Self
    }

    fn thread(&self, state: Arc<State>, _: Sender<Event>) -> anyhow::Result<()> {
        let browser = &state.browser;
        let mut tabs = browser.tabs();
        let tab = if !tabs.is_empty() {
            &tabs[0]
        } else {
            let new_tab = browser.new_tab()?;
            new_tab.navigate_to("https://www.google.com")?;
            tabs.push(new_tab);
            tabs.last().unwrap()
        };

        if let Ok(data) = tab.capture_screenshot() {
            display_img(&data)?;
        }

        Ok(())
    }

    fn deps(&self) -> &[Event] {
        &[Event::RefreshScreen]
    }
}
