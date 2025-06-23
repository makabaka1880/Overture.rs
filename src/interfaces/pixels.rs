// Created by Sean L. on Jun. 23.
// Last Updated by Sean L. on Jun. 24.
// 
// overture.rs
// src/interfaces/bitmap.rs
// 
// Makabaka1880, 2025. All rights reserved.

//! Defines the `Pixel` struct representing a styled character at a specific position on the terminal canvas.
//!
//! A `Pixel` bundles a [`RenderChar`] (character + style) with its absolute position ([`DiscreteCoord`]), 
//! serving as the fundamental unit of rendering in the engine.

use std::os::unix::process;

use crate::interfaces::{rendering::RenderChar, geometry::DiscreteCoord};

/// Represents a single drawable unit on the terminal screen.
/// 
/// A `Pixel` consists of the character and its style (wrapped in [`RenderChar`]) along with its position
/// on the grid (`DiscreteCoord`), making it the atomic element for rasterization and rendering.
#[derive(Clone, Debug)]
pub struct Pixel {
    /// The styled character to be displayed.
    pub content: RenderChar,
    /// The pixel's position in discrete terminal coordinates.
    pub position: DiscreteCoord,
    /// Whether or not the pixel is prunable
    pub protected: bool,
}

impl Pixel {
    /// Creates a new `Pixel` from a [`RenderChar`] and position.
    ///
    /// # Arguments
    ///
    /// - `content` - The styled character to display.
    /// - `position` - The position of the pixel on the terminal grid.
    /// - `protected` - The prune protection flag of this pixel.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use overture::interfaces::{pixels::Pixel, rendering::RenderChar, styling::RenderStyle, geometry::DiscreteCoord};
    /// 
    /// let pixel = Pixel::new(RenderChar::new('A', RenderStyle::Plain), DiscreteCoord::new(10, 5), false);
    /// ```
    pub fn new(content: RenderChar, position: DiscreteCoord, protected: bool) -> Self {
        Pixel { content, position, protected }
    }

    /// Convenience constructor to create a `Pixel` from a plain character and position.
    ///
    /// The created `Pixel` will have the default plain style.
    ///
    /// # Arguments
    ///
    /// - `content` - The plain character to display.
    /// - `position` - The position of the pixel on the terminal grid.
    /// - `protected` - The prune protection flag of this pixel.
    /// 
    /// # Examples
    ///
    /// ```rust
    /// use overture::interfaces::{pixels::Pixel, geometry::DiscreteCoord};
    /// 
    /// let pixel = Pixel::new_with_char('A', DiscreteCoord::new(10, 5), false);
    /// ```
    pub fn new_with_char(content: char, position: DiscreteCoord, protected: bool) -> Self {
        Pixel::new(RenderChar::new_plain(content), position, protected)
    }
}
