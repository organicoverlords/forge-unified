//! Bundled single-page chat UI for the Forge MVP.
//!
//! The browser UI is kept in `chat_ui.html` so session-turn layout,
//! tool-card lifecycle rendering, and proof hooks stay reviewable without
//! hiding the product UI inside a long Rust raw string. Small browser-only
//! enhancements live beside it so proof UX can evolve without rebuilding a
//! giant string literal.

pub const CHAT_HTML: &str = concat!(
    include_str!("chat_ui.html"),
    include_str!("chat_ui_enhancements.html"),
);
