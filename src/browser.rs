use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::Result;

use crate::state::{StateProc, StateRef};
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

pub struct BrowserThread;

impl StateProc for BrowserThread {
    fn thread(state: StateRef) -> Result<()> {
        let browser = &state.browser;
        let tab = browser.new_tab()?;
        tab.navigate_to("https://google.com")?;

        if let (Ok(data), Ok(mut buf)) = (tab.capture_screenshot(), state.buf.write()) {
            *buf = data;

            let (lock, cvar) = &state.started;
            let mut started = lock.lock().unwrap();
            *started = true;
            cvar.notify_one();
        }

        while let (Ok(data), Ok(mut buf)) = (tab.capture_screenshot(), state.buf.write()) {
            *buf = data;
            thread::sleep(std::time::Duration::from_secs(1));

            if *state.ended.lock().unwrap() {
                break;
            }
        }

        Ok(())
    }
}
