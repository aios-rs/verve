//! Persistent storage for workspace data.
//!
//! This module handles loading and saving workspace configuration to local files.

use serde::{Deserialize, Serialize};

/// Workspace layout and UI preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceLayout {
    /// The currently selected locale (e.g., "zh-CN", "en").
    pub locale: Option<String>,
    // Additional layout settings would go here
}

/// Load the workspace layout from persistent storage.
pub fn load_layout() -> Option<WorkspaceLayout> {
    // TODO: Implement actual loading from ~/.verve/workspace.json
    None
}

/// Save the workspace layout to persistent storage.
pub fn save_layout(layout: &WorkspaceLayout) -> anyhow::Result<()> {
    // TODO: Implement actual saving to ~/.verve/workspace.json
    Ok(())
}
