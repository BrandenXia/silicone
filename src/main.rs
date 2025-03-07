use std::error::Error;

use headless_chrome::protocol::cdp::Page;
use headless_chrome::{Browser, LaunchOptions};

mod image;
mod terminal;

fn main() -> Result<(), Box<dyn Error>> {
    let win_size = terminal::get_terminal_size().unwrap();
    let win_size = (win_size.0 / 2, win_size.1 / 2);

    let options = LaunchOptions::default_builder()
        .window_size(Some(win_size))
        .build()
        .unwrap();
    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;
    tab.navigate_to("https://www.rust-lang.org")?;
    let data =
        tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, None, true)?;

    image::display_img(&data)?;

    Ok(())
}
