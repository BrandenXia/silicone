use std::sync::{Arc, Condvar, Mutex, RwLock};

use crate::browser::Browser;

pub struct State {
    pub(crate) browser: Browser,
    pub(crate) started: (Mutex<bool>, Condvar),
    pub(crate) ended: Mutex<bool>,
    pub(crate) buf: RwLock<Vec<u8>>,
}

impl State {
    pub fn new() -> Arc<Self> {
        let browser = Browser::new().unwrap();
        let buf = RwLock::new(Vec::<u8>::new());
        let started = (Mutex::new(false), Condvar::new());
        let ended = Mutex::new(false);

        Arc::new(Self {
            browser,
            buf,
            started,
            ended,
        })
    }
}

pub type StateRef = Arc<State>;

pub trait StateProc {
    fn thread(state: StateRef) -> anyhow::Result<()>;

    fn spawn(state: StateRef) -> std::thread::JoinHandle<anyhow::Result<()>> {
        std::thread::spawn(move || Self::thread(state))
    }
}

#[macro_export]
macro_rules! spawn {
    ($type:ty, $state:expr, $handles:expr) => {
        silicone::assert_impl!(StateProc, $type);
        {
            let state = Arc::clone(&$state);
            $handles.push(<$type>::spawn(state));
        }
    };
}
