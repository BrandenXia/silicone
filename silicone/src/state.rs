use std::sync::{Arc, mpsc::Sender};

use silicone_macro::EnumCount;

use crate::browser::Browser;

#[derive(Debug, Clone, Copy, PartialEq, EnumCount)]
pub enum Event {
    Start,
    End,
    RefreshScreen,
}

pub struct State {
    pub(crate) browser: Browser,
}

impl State {
    pub fn new() -> Self {
        Self {
            browser: Browser::new().expect("Fail to initialize browser"),
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

#[macro_export]
macro_rules! handler_default_new {
    () => {
        fn new() -> Self {
            Self
        }
    };
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
        h.deps()
            .iter()
            .for_each(|e| self.handlers[*e as usize].push(self._handlers.len()));
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
