// Created by Sean L. on Jun. 23.
// Last Updated by Sean L. on Jun. 23.
// 
// overture.rs
// src/ioopts/mod.rs
// 
// Makabaka1880, 2025. All rights reserved.

//! IO Options Module
//!
//! This module provides utilities for input/output options, including support for ANSI escape sequences
//! and box drawing symbols. These features are useful for enhancing terminal output with colors,
//! formatting, and graphical elements.
//!
//! # Submodules
//! - [`ansi`]: Utilities for working with ANSI escape sequences (e.g., colors, styles).
//! - [`box_drawing`]: Functions and constants for rendering box drawing symbols in terminal UIs.


pub mod ansi;
pub mod box_drawing;