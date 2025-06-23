// Created by Sean L. on Jun. 23.
// Last Updated by Sean L. on Jun. 23.
// 
// Overture.rs
// src/primitives/mod.rs
// 
// Makabaka1880, 2025. All rights reserved.

//! The `primitives` module provides fundamental building blocks for the Overture project.
//!
//! This module contains submodules for geometric shapes and text primitives, which can be
//! used throughout the application for rendering and layout purposes.
//!
//! # Submodules
//! - [`shape`]: Contains definitions and utilities for geometric shapes.
//! - [`text`]: Provides structures and functions for handling text primitives.
//!
//! # Re-exports
//! - [`Text`]: The main text primitive type, re-exported for convenience.
//!
//! # Examples
//! ```rust
//! use overture::primitives::Text;
//! // Create and use a Text primitive...
//! ```

pub mod shape;
pub mod text;

pub use crate::primitives::text::Text;