//! Verve — an offline-first API co-development platform built with gpui-component.
//!
//! This is the core library that provides the main functionality for the Verve application.

// Roadmap features and helpers produce dead-code warnings until wired up.
#![allow(dead_code)]
// The GPUI element-builder style produces some complex closures.
#![allow(clippy::type_complexity)]
// Some serde data structs are fine to derive-impl via macros.
#![allow(clippy::derivable_impls)]

pub mod state;
pub mod ui;
