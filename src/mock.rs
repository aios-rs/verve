//! Mock HTTP server for API testing.
//!
//! This module provides a local mock server for testing API responses.

use std::collections::HashMap;
use std::sync::Arc;

/// Default port for the mock server.
pub const DEFAULT_PORT: u16 = 3000;

/// Create a rule map from the active project.
pub fn rule_map(_project: &crate::state::ProjectData) -> HashMap<String, String> {
    HashMap::new()
}

/// Start the mock server with the given rules.
pub fn serve(_port: u16, _rules: Arc<HashMap<String, String>>) {
    // TODO: Implement actual mock server
}
