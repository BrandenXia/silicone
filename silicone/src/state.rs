use std::sync::{Arc, RwLock, mpsc::Sender};

use silicone_macro::EnumCount;

use crate::browser::Browser;

#[derive(Debug, Clone, Copy, EnumCount)]
pub enum Event {
    RefreshScreen,
}

pub struct State {
    pub(crate) browser: Browser,
    pub(crate) buf: RwLock<Vec<u8>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            browser: Browser::new().expect("Fail to initialize browser"),
            buf: RwLock::new(Vec::new()),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Handler: Send + Sync {
    fn new() -> Self
    where
        Self: Sized;
    fn deps(&self) -> &[Event];
    fn thread(&self, state: Arc<State>, tx: Sender<Event>) -> anyhow::Result<()>;
}

pub struct Handlers {
    _handlers: Vec<Arc<dyn Handler>>,
    handlers: [Vec<usize>; Event::COUNT],
}

impl Handlers {
    pub fn new() -> Self {
        Self {
            _handlers: Vec::new(),
            handlers: Default::default(),
        }
    }

    pub fn add_handler<H: Handler + 'static>(&mut self) {
        let h = H::new();
        h.deps().iter().for_each(|e| {
            self.handlers[*e as usize].push(self._handlers.len() - 1);
        });
        self._handlers.push(Arc::new(h));
    }

    pub fn get_handlers(&self, event: Event) -> Vec<&Arc<dyn Handler>> {
        self.handlers[event as usize]
            .iter()
            .map(|&i| &self._handlers[i])
            .collect()
    }
}

impl Default for Handlers {
    fn default() -> Self {
        Self::new()
    }
}
