mod browser;
mod image;
mod input;
mod macros;
mod terminal;
mod ui;

pub mod state;
pub mod handlers {
    pub use crate::browser::BrowserHandler;
    pub use crate::ui::RenderHandler;
}
