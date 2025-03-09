use std::sync::{Arc, Condvar, Mutex, RwLock};

#[derive(Default)]
pub struct State {
    pub started: (Mutex<bool>, Condvar),
    pub ended: Mutex<bool>,
    pub buf: RwLock<Vec<u8>>,
}

impl State {
    pub fn new() -> Arc<Self> {
        let buf = RwLock::new(Vec::<u8>::new());
        let started = (Mutex::new(false), Condvar::new());
        let ended = Mutex::new(false);

        Arc::new(Self {
            buf,
            started,
            ended,
        })
    }
}

pub type StateRef = Arc<State>;
