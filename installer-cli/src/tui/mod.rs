//! Ratatui TUI for MASH Installer — cyberpunk hacker terminal.

pub mod app;
pub mod bbs;
pub mod menus;
pub mod observer;
pub mod render;
pub mod sysinfo_poller;
pub mod theme;

pub use app::run;
