//! 1984 Retro-Station color palette and style helpers for the MASH TUI.
//! Transformation from Cyberpunk to BBC Micro/UNIX station aesthetic.

use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::BorderType;

// ── Base palette (Phosphor Glow) ─────────────────────────────────────────────

pub const BLACK: Color = Color::Black;
pub const GREEN: Color = Color::Rgb(0, 255, 0); // Primary Phosphor
pub const AMBER: Color = Color::Rgb(255, 191, 0); // Secondary Phosphor
pub const RED: Color = Color::Rgb(255, 0, 0); // Fatal Error
pub const DIM_GREEN: Color = Color::Rgb(0, 100, 0);

// ── Style helpers ────────────────────────────────────────────────────────────

pub fn default_style() -> Style {
    Style::default().fg(GREEN).bg(BLACK)
}

pub fn border_style() -> Style {
    Style::default().fg(GREEN).bg(BLACK)
}

pub fn title_style() -> Style {
    Style::default()
        .fg(BLACK)
        .bg(GREEN)
        .add_modifier(Modifier::BOLD)
}

pub fn selected_style() -> Style {
    Style::default()
        .fg(BLACK)
        .bg(AMBER)
        .add_modifier(Modifier::BOLD)
}

pub fn success_style() -> Style {
    Style::default().fg(GREEN).add_modifier(Modifier::BOLD)
}

pub fn warning_style() -> Style {
    Style::default().fg(AMBER)
}

pub fn error_style() -> Style {
    Style::default().fg(RED).add_modifier(Modifier::REVERSED)
}

pub fn dim_style() -> Style {
    Style::default().fg(DIM_GREEN)
}

pub fn accent_style() -> Style {
    Style::default().fg(AMBER)
}

// ── Border helpers ───────────────────────────────────────────────────────────

pub fn outer_border_type() -> BorderType {
    BorderType::Thick
}
