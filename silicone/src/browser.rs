use std::sync::Arc;

use anyhow::Result;
use crossterm::event::{KeyEvent, KeyModifiers};
use headless_chrome::Tab;
use headless_chrome::browser::tab::ModifierKey;
use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;

use crate::terminal::get_terminal_size;

pub(crate) struct Browser {
    inner: headless_chrome::Browser,
    current_tab: Arc<Tab>,
}

fn key_event_2_key(k: KeyEvent) -> String {
    match k.code {
        crossterm::event::KeyCode::Char(c) => c.to_string(),
        crossterm::event::KeyCode::Enter => "Enter".to_string(),
        crossterm::event::KeyCode::Backspace => "Backspace".to_string(),
        crossterm::event::KeyCode::Left => "ArrowLeft".to_string(),
        crossterm::event::KeyCode::Right => "ArrowRight".to_string(),
        crossterm::event::KeyCode::Up => "ArrowUp".to_string(),
        crossterm::event::KeyCode::Down => "ArrowDown".to_string(),
        _ => String::new(),
    }
}

fn key_event_2_modifiers(k: KeyEvent) -> Vec<ModifierKey> {
    let mut modifiers = Vec::new();

    if k.modifiers.contains(KeyModifiers::CONTROL) {
        modifiers.push(ModifierKey::Ctrl);
    } else if k.modifiers.contains(KeyModifiers::SHIFT) {
        modifiers.push(ModifierKey::Shift);
    } else if k.modifiers.contains(KeyModifiers::ALT) {
        modifiers.push(ModifierKey::Alt);
    } else if k.modifiers.contains(KeyModifiers::META) {
        modifiers.push(ModifierKey::Meta);
    }

    modifiers
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
        let modifiers = key_event_2_modifiers(k);
        self.current_tab.press_key_with_modifiers(
            key_event_2_key(k).as_str(),
            if modifiers.is_empty() {
                None
            } else {
                Some(modifiers.as_slice())
            },
        )?;

        Ok(())
    }
}
