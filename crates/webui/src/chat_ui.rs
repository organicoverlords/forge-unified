//! Bundled single-page chat UI for the Forge MVP.
//!
//! The browser UI is kept in `chat_ui.html` so session-turn layout,
//! tool-card lifecycle rendering, and proof hooks stay reviewable without
//! hiding the product UI inside a long Rust raw string.

pub const CHAT_HTML: &str = include_str!("chat_ui.html");
