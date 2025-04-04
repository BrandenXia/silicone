use std::sync::{Arc, RwLock, mpsc::Sender};

use anyhow::Result;
use crossterm::terminal::window_size;
use silicone_macro::EnumCount;

use crate::{browser::Browser, terminal::get_terminal_size};

#[derive(Debug, Clone, Copy, PartialEq, EnumCount)]
pub enum Event {
    Start,
    End,
    RefreshScreen,
}

pub struct State {
    pub(crate) browser: Browser,
    pub(crate) cr_size: RwLock<(u16, u16)>,
    pub(crate) term_size: (u32, u32),
}

impl State {
    pub fn new(initial_url: &str) -> Result<Self> {
        let size = window_size()?;
        Ok(Self {
            browser: Browser::new(initial_url)?,
            cr_size: RwLock::new((size.columns, size.rows)),
            term_size: get_terminal_size()?,
        })
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new("https://google.com").expect("Failed to create state")
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
