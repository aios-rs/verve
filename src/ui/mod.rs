//! User interface components and views.
//!
//! This module contains all the UI components for the Verve application.

pub mod themes;
pub mod app;
pub mod request_panel;

use gpui::*;

/// The main Verve application UI.
pub struct VerveApp {
    window: Window,
}

impl VerveApp {
    /// Create a new Verve application instance.
    pub fn new(window: Window, _cx: &mut Context<Self>) -> Self {
        Self { window }
    }
}

impl Render for VerveApp {
    fn render(&mut mut self, _cx: &mut Context<Self>) -> impl IntoElement {
        // TODO: Implement the actual UI rendering
        div()
    }
}
