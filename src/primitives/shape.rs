// Created by Sean L. on Jun. 22.
// Last Updated by Sean L. on Jun. 23.
// 
// overture.rs
// src/primitives/rectangle.rs
// 
// Makabaka1880, 2025. All rights reserved.

//! Shape Definitions for Terminal Rendering
//!
//! This module provides basic geometric shape primitives for terminal-based rendering,
//! including rectangles and soft-edged boxes. All shapes are defined in terms of
//! discrete 2D coordinates and implement the [`Renderable`] trait, allowing them to
//! be drawn using Unicode box-drawing characters.
//!
//! # Overview
//!
//! - [`Rectangle`]: A rectangular box with sharp corners, rendered using standard box-drawing characters.
//! - [`SoftBox`]: A rectangular box with rounded (soft) corners, rendered using soft box-drawing characters.
//!
//! All shapes are internally normalized so that their `pos` field always represents
//! the top-left corner and `corner` the bottom-right corner, regardless of the order
//! of points provided to their constructors.
//!
//! # Usage
//!
//! Shapes can be constructed from any two points and will automatically normalize
//! their coordinates. They can then be rendered to a terminal UI by collecting their
//! pixels via the [`Renderable::pixels`] or, more idiomatically, [`Renderable::rasterize`] method.
//!
//! # Examples
//!
//! ```rust
//! use overture::primitives::shape::{Rectangle, SoftBox};
//! use overture::interfaces::rendering::Renderable;    // Import `Renderable` to use `rasterize()`
//! use overture::interfaces::geometry::DiscreteCoord;
//!
//! let rect = Rectangle::new(DiscreteCoord::new(1, 2), DiscreteCoord::new(5, 6));
//! let soft = SoftBox::new(DiscreteCoord::new(3, 4), DiscreteCoord::new(7, 8));
//! let rect_pixels = rect.rasterize();
//! let soft_pixels = soft.rasterize();
//! ```
//!
//! # See Also
//!
//! - [`Renderable`]: Trait for objects that can be rendered as a collection of pixels.
//! - [`DiscreteCoord`]: Discrete 2D coordinate type used for shape positioning.
//! - [`Pixel`]: Represents a single drawable cell in the terminal UI.

use crate::{
    ioopts::box_drawing::box_drawing,
    interfaces::{
        geometry::DiscreteCoord,
        rendering::Renderable,
        pixels::Pixel,
    }
};

/// A rectangular box primitive defined by two points in 2D space.
///
/// Internally normalizes corner coordinates so that `pos` is the top-left corner
/// and `corner` is the bottom-right corner, no matter the order of constructor arguments.
///
/// # Examples
///
/// ```rust
/// use overture::primitives::shape::Rectangle;
/// use overture::interfaces::geometry::DiscreteCoord;
///
/// let rect = Rectangle::new(
///     DiscreteCoord::new(2, 3),
///     DiscreteCoord::new(10, 8),
/// );
///
/// assert_eq!(rect.pos(), DiscreteCoord::new(2, 3));
/// assert_eq!(rect.corner(), DiscreteCoord::new(10, 8));
/// ```
pub struct Rectangle {
    pos: DiscreteCoord,
    corner: DiscreteCoord
}

impl Rectangle {
    /// Returns the top-left corner of the rectangle.
    ///
    /// This is always the minimum x and y coordinate of the two corners.
    pub fn pos(&self) -> DiscreteCoord { self.pos }

    /// Returns the bottom-right corner of the rectangle.
    ///
    /// This is always the maximum x and y coordinate of the two corners.
    pub fn corner(&self) -> DiscreteCoord { self.corner }

    /// Creates a new `Rectangle` from two arbitrary points.
    ///
    /// The constructor normalizes the coordinates so that `pos` is always the
    /// top-left corner and `corner` is always the bottom-right corner.
    ///
    /// # Parameters
    ///
    /// - `p1`: One corner of the rectangle.
    /// - `p2`: The opposite corner of the rectangle.
    ///
    /// # Returns
    ///
    /// A `Rectangle` instance normalized for consistent rendering.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use overture::primitives::shape::Rectangle;
    /// use overture::interfaces::geometry::DiscreteCoord;
    ///
    /// let rect = Rectangle::new(DiscreteCoord::new(10, 5), DiscreteCoord::new(2, 8));
    /// assert_eq!(rect.pos(), DiscreteCoord::new(2, 5));
    /// assert_eq!(rect.corner(), DiscreteCoord::new(10, 8));
    /// ```
    pub fn new(p1: DiscreteCoord, p2: DiscreteCoord) -> Self {
        match (p1.x > p2.x, p1.y > p2.y) {
            (false, false) => Rectangle { pos: p1, corner: p2 },
            (true, false) => Rectangle { pos: DiscreteCoord { x: p2.x, y: p1.y }, corner: DiscreteCoord { x: p1.x, y: p2.y } },
            (false, true) => Rectangle { pos: DiscreteCoord { x: p1.x, y: p2.y }, corner: DiscreteCoord { x: p2.x, y: p1.y } },
            (true, true) => Rectangle { pos: p2, corner: p1 },
        }
    }
}

impl Default for Rectangle {
    /// Disabled default constructor. Use [`Rectangle::new`] instead.
    fn default() -> Self {
        panic!("Default constructor is disabled, use Rectangle::new() instead");
    }
}

impl Renderable for Rectangle {
    /// Returns the pixels representing the box outline using Unicode box-drawing characters.
    ///
    /// This includes four corners and horizontal/vertical edges.
    ///
    /// # Returns
    ///
    /// A `Vec<Pixel>` containing all the pixels needed to draw the rectangle's border.
    fn pixels(&self) -> Vec<Pixel> {
        let mut pixels = vec![];
        // Corners
        pixels.push(Pixel::new_with_char(box_drawing::LU_CORNER, self.pos, false));
        pixels.push(Pixel::new_with_char(box_drawing::RD_CORNER, self.corner, false));
        pixels.push(Pixel::new_with_char(box_drawing::LD_CORNER, DiscreteCoord::new(self.pos.x, self.corner.y), false));
        pixels.push(Pixel::new_with_char(box_drawing::RU_CORNER, DiscreteCoord::new(self.corner.x, self.pos.y), false));
        // Top and bottom edges
        for x in (self.pos.x + 1)..self.corner.x {
            pixels.push(Pixel::new_with_char(box_drawing::H_LINE, DiscreteCoord::new(x, self.pos.y), false));
            pixels.push(Pixel::new_with_char(box_drawing::H_LINE, DiscreteCoord::new(x, self.corner.y), false));
        }
        // Left and right edges
        for y in (self.pos.y + 1)..self.corner.y {
            pixels.push(Pixel::new_with_char(box_drawing::V_LINE, DiscreteCoord::new(self.pos.x, y), false));
            pixels.push(Pixel::new_with_char(box_drawing::V_LINE, DiscreteCoord::new(self.corner.x, y),false));
        }
        pixels
    }

    /// Returns the dimensions of the rectangle as a `DiscreteCoord` representing width and height.
    fn dim(&self) -> DiscreteCoord {
        self.corner - self.pos
    }
}

/// A soft-edged rectangular box primitive defined by two points in 2D space.
///
/// This is similar to [`Rectangle`], but uses Unicode box-drawing characters
/// with rounded corners for a softer visual style.
///
/// Internally normalizes corner coordinates so that `pos` is the top-left corner
/// and `corner` is the bottom-right corner, regardless of the order of points given.
///
/// # Examples
///
/// ```rust
/// use overture::primitives::shape::SoftBox;
/// use overture::interfaces::geometry::DiscreteCoord;
///
/// let soft_box = SoftBox::new(
///     DiscreteCoord::new(5, 5),
///     DiscreteCoord::new(15, 10),
/// );
///
/// assert_eq!(soft_box.pos(), DiscreteCoord::new(5, 5));
/// assert_eq!(soft_box.corner(), DiscreteCoord::new(15, 10));
/// ```
pub struct SoftBox {
    pos: DiscreteCoord,
    corner: DiscreteCoord,
}

impl SoftBox {
    /// Returns the top-left corner of the soft box.
    ///
    /// This is always the minimum x and y coordinate of the two corners.
    pub fn pos(&self) -> DiscreteCoord { self.pos }

    /// Returns the bottom-right corner of the soft box.
    ///
    /// This is always the maximum x and y coordinate of the two corners.
    pub fn corner(&self) -> DiscreteCoord { self.corner }

    /// Creates a new `SoftBox` from two arbitrary points.
    ///
    /// The constructor normalizes the coordinates so that `pos` is always the
    /// top-left corner and `corner` is always the bottom-right corner.
    ///
    /// # Parameters
    ///
    /// - `p1`: One corner of the box.
    /// - `p2`: The opposite corner of the box.
    ///
    /// # Returns
    ///
    /// A `SoftBox` instance normalized for consistent rendering.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use overture::primitives::shape::SoftBox;
    /// use overture::interfaces::geometry::DiscreteCoord;
    ///
    /// let soft_box = SoftBox::new(DiscreteCoord::new(10, 5), DiscreteCoord::new(2, 8));
    /// assert_eq!(soft_box.pos(), DiscreteCoord::new(2, 5));
    /// assert_eq!(soft_box.corner(), DiscreteCoord::new(10, 8));
    /// ```
    pub fn new(p1: DiscreteCoord, p2: DiscreteCoord) -> Self {
        match (p1.x > p2.x, p1.y > p2.y) {
            (false, false) => SoftBox { pos: p1, corner: p2 },
            (true, false) => SoftBox { pos: DiscreteCoord { x: p2.x, y: p1.y }, corner: DiscreteCoord { x: p1.x, y: p2.y } },
            (false, true) => SoftBox { pos: DiscreteCoord { x: p1.x, y: p2.y }, corner: DiscreteCoord { x: p2.x, y: p1.y } },
            (true, true) => SoftBox { pos: p2, corner: p1 },
        }
    }
}

impl Default for SoftBox {
    /// Disabled default constructor. Use [`SoftBox::new`] instead.
    fn default() -> Self {
        panic!("Default constructor is disabled, use SoftBox::new() instead");
    }
}

impl Renderable for SoftBox {
    /// Returns the pixels representing the soft box outline using Unicode rounded box-drawing characters.
    ///
    /// This includes four rounded corners and horizontal/vertical edges.
    ///
    /// # Returns
    ///
    /// A `Vec<Pixel>` containing all the pixels needed to draw the soft box's border.
    fn pixels(&self) -> Vec<Pixel> {
        let mut pixels = vec![];
        // Corners with soft rounded characters
        pixels.push(Pixel::new_with_char(box_drawing::LU_CORNER_SOFT, self.pos, false));
        pixels.push(Pixel::new_with_char(box_drawing::RD_CORNER_SOFT, self.corner, false));
        pixels.push(Pixel::new_with_char(box_drawing::LD_CORNER_SOFT, DiscreteCoord::new(self.pos.x, self.corner.y), false));
        pixels.push(Pixel::new_with_char(box_drawing::RU_CORNER_SOFT, DiscreteCoord::new(self.corner.x, self.pos.y), false));
        // Top and bottom edges
        for x in (self.pos.x + 1)..self.corner.x {
            pixels.push(Pixel::new_with_char(box_drawing::H_LINE, DiscreteCoord::new(x, self.pos.y), false));
            pixels.push(Pixel::new_with_char(box_drawing::H_LINE, DiscreteCoord::new(x, self.corner.y), false));
        }
        // Left and right edges
        for y in (self.pos.y + 1)..self.corner.y {
            pixels.push(Pixel::new_with_char(box_drawing::V_LINE, DiscreteCoord::new(self.pos.x, y), false));
            pixels.push(Pixel::new_with_char(box_drawing::V_LINE, DiscreteCoord::new(self.corner.x, y), false));
        }
        pixels
    }

    /// Returns the dimensions of the soft box as a `DiscreteCoord` representing width and height.
    fn dim(&self) -> DiscreteCoord {
        self.corner - self.pos
    }
}
