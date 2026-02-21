//! Cyberpunk color palette and style helpers for the MASH TUI.

use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::BorderType;

// ── Base palette ────────────────────────────────────────────────────────────

pub const BLACK: Color = Color::Rgb(10, 10, 20);
pub const WHITE: Color = Color::White;
pub const CYAN: Color = Color::Rgb(0, 255, 255);
pub const MAGENTA: Color = Color::Rgb(255, 0, 255);
pub const GREEN: Color = Color::Rgb(0, 255, 65); // matrix green
pub const YELLOW: Color = Color::Rgb(255, 215, 0);
pub const RED: Color = Color::Rgb(255, 49, 49);
pub const DIM_GRAY: Color = Color::Rgb(80, 80, 100);

// ── Style helpers ────────────────────────────────────────────────────────────

pub fn default_style() -> Style {
    Style::default().fg(WHITE).bg(BLACK)
}

pub fn border_style() -> Style {
    Style::default().fg(CYAN).bg(BLACK)
}

pub fn title_style() -> Style {
    Style::default()
        .fg(CYAN)
        .bg(BLACK)
        .add_modifier(Modifier::BOLD)
}

pub fn selected_style() -> Style {
    Style::default()
        .fg(MAGENTA)
        .bg(BLACK)
        .add_modifier(Modifier::BOLD)
}

pub fn success_style() -> Style {
    Style::default().fg(GREEN)
}

pub fn warning_style() -> Style {
    Style::default().fg(YELLOW)
}

pub fn error_style() -> Style {
    Style::default().fg(RED)
}

pub fn dim_style() -> Style {
    Style::default().fg(DIM_GRAY)
}

pub fn accent_style() -> Style {
    Style::default().fg(MAGENTA)
}

pub fn bbs_style() -> Style {
    Style::default()
        .fg(MAGENTA)
        .bg(BLACK)
        .add_modifier(Modifier::BOLD | Modifier::ITALIC)
}

pub fn progress_filled_style() -> Style {
    Style::default().fg(CYAN)
}

// ── Border helpers ───────────────────────────────────────────────────────────

pub fn inner_border_type() -> BorderType {
    BorderType::Rounded
}

pub fn outer_border_type() -> BorderType {
    BorderType::Double
}
