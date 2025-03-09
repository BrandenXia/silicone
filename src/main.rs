use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;

use anyhow::Result;

use silicone::browser::Browser;
use silicone::image::display_img;

fn main() -> Result<()> {
    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )?;

    let buf = Arc::new(RwLock::new(Vec::<u8>::new()));
    let started = Arc::new((Mutex::new(false), Condvar::new()));
    let ended = Arc::new(Mutex::new(false));

    let writer_buf = Arc::clone(&buf);
    let writer_started = Arc::clone(&started);
    let writer_ended = Arc::clone(&ended);

    let signal_ended = Arc::clone(&ended);
    ctrlc::set_handler(move || {
        let mut ended = signal_ended.lock().unwrap();
        *ended = true;
    })?;

    thread::spawn(move || {
        let browser = Browser::new().unwrap();
        let tab = browser.new_tab().unwrap();
        tab.navigate_to("https://www.rust-lang.org").unwrap();

        if let (Ok(data), Ok(mut buf)) = (tab.capture_screenshot(), writer_buf.write()) {
            *buf = data;

            let (lock, cvar) = &*writer_started;
            let mut started = lock.lock().unwrap();
            *started = true;
            cvar.notify_one();
        }

        while let (Ok(data), Ok(mut buf)) = (tab.capture_screenshot(), writer_buf.write()) {
            *buf = data;
            thread::sleep(std::time::Duration::from_secs(1));

            if *writer_ended.lock().unwrap() {
                break;
            }
        }
    });

    let (lock, cvar) = &*started;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    while let Ok(data) = buf.read() {
        display_img(data.as_slice())?;

        if *ended.lock().unwrap() {
            break;
        }
    }

    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::cursor::Show
    )?;

    Ok(())
}
