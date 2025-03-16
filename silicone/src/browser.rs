use std::sync::Arc;

use anyhow::Result;
use crossterm::event::KeyEvent;
use headless_chrome::Tab;
use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;

use crate::terminal::get_terminal_size;

pub(crate) struct Browser {
    inner: headless_chrome::Browser,
    current_tab: Arc<Tab>,
}

fn key_event_to_key(k: KeyEvent) -> String {
    match k.code {
        crossterm::event::KeyCode::Char(c) => c.to_string(),
        _ => String::new(),
    }
}

impl Browser {
    pub(crate) fn new() -> Result<Self> {
        let win_size = get_terminal_size()?;

        let options = headless_chrome::LaunchOptions::default_builder()
            .window_size(Some(win_size))
            .build()?;
        let browser = headless_chrome::Browser::new(options)?;

        let current_tab = browser.new_tab()?;
        current_tab
            .navigate_to("https://www.google.com")?
            .wait_until_navigated()?;

        Ok(Self {
            inner: browser,
            current_tab,
        })
    }

    pub(crate) fn capture_screenshot(&self) -> Result<Vec<u8>> {
        self.current_tab
            .capture_screenshot(CaptureScreenshotFormatOption::Png, None, None, true)
    }

    pub(crate) fn handle_key(&self, k: KeyEvent) -> Result<()> {
        self.current_tab
            .press_key_with_modifiers(key_event_to_key(k).as_str(), None)?;

        Ok(())
    }
}
