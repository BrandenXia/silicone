use std::thread;

use crate::image::display_img;
use crate::state::StateProc;

pub struct RenderThread;

impl StateProc for RenderThread {
    fn thread(state: crate::state::StateRef) -> anyhow::Result<()> {
        let (lock, cvar) = &state.started;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }

        while let Ok(data) = state.buf.read() {
            display_img(data.as_slice())?;
            thread::sleep(std::time::Duration::from_secs(1));

            if *state.ended.lock().unwrap() {
                break;
            }
        }

        Ok(())
    }
}
