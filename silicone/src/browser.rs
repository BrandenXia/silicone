use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::Result;

use crate::state::{Event, Handler, State};
use crate::terminal::get_terminal_size;

pub(crate) struct Browser {
    inner: headless_chrome::Browser,
}

impl Browser {
    pub(crate) fn new() -> Result<Self> {
        let win_size = get_terminal_size()?;

        let options = headless_chrome::LaunchOptions::default_builder()
            .window_size(Some(win_size))
            .build()?;
        let browser = headless_chrome::Browser::new(options)?;

        Ok(Self { inner: browser })
    }

    fn new_tab(&self) -> Result<Tab> {
        Ok(Tab::new(self.inner.new_tab()?))
    }

    fn tabs(&self) -> Vec<Tab> {
        Tab::from_tabs(self.inner.get_tabs())
    }
}

struct Tab {
    tab: Arc<headless_chrome::Tab>,
}

impl Tab {
    fn new(tab: Arc<headless_chrome::Tab>) -> Self {
        Self { tab }
    }

    fn from_tabs(tabs: &Arc<Mutex<Vec<Arc<headless_chrome::Tab>>>>) -> Vec<Self> {
        tabs.lock()
            .unwrap()
            .iter()
            .map(|tab| Self::new(tab.clone()))
            .collect()
    }

    fn navigate_to(&self, url: &str) -> Result<()> {
        self.tab.navigate_to(url)?.wait_until_navigated()?;
        Ok(())
    }

    fn capture_screenshot(&self) -> Result<Vec<u8>> {
        let data = self.tab.capture_screenshot(
            headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
            None,
            None,
            true,
        )?;
        Ok(data)
    }
}

pub struct BrowserHandler;

impl Handler for BrowserHandler {
    fn new() -> Self {
        Self
    }

    fn deps(&self) -> &[Event] {
        &[]
    }

    fn thread(&self, state: Arc<State>, tx: Sender<Event>) -> Result<()> {
        let browser = &state.browser;
        let tab = browser.new_tab()?;
        tab.navigate_to("https://google.com")?;

        if let (Ok(data), Ok(mut buf)) = (tab.capture_screenshot(), state.buf.write()) {
            *buf = data;
            tx.send(Event::RefreshScreen)
                .expect("Failed to send image ready event");
        }

        Ok(())
    }
}
