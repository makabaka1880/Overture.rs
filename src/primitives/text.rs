// Created by Sean L. on Jun. 22.
// Last Updated by Sean L. on Jun. 24.
// 
// overture.rs
// src/primitives/text.rs
// 
// Makabaka1880, 2025. All rights reserved.

//! Text Primitive for Terminal Rendering
//!
//! This module provides a simple `Text` struct representing a string
//! positioned at a discrete 2D coordinate in the terminal. It implements
//! the [`Renderable`] trait to convert the text into a sequence of pixels
//! for rendering.
//!
//! Additionally, `Text` supports ASCII art rendering using `figlet_rs` fonts,
//! allowing text to be transformed into large decorative ASCII banners.

use crate::interfaces::{
    geometry::DiscreteCoord,
    pixels::Pixel,
    rendering::Renderable,
};
use std::ops::Deref;

/// A textual content positioned in 2D discrete terminal space.
///
/// Holds the string content and its starting position.
/// Implements [`Renderable`] to convert into pixels for rendering.
///
/// # Examples
///
/// ```rust
/// use overture::primitives::text::Text;
/// use overture::interfaces::{geometry::DiscreteCoord, rendering::Renderable};
///
/// let text = Text::new("Hello", DiscreteCoord::new(5, 10));
/// let pixels = text.rasterize();
/// assert_eq!(pixels.len(), 5);  // One pixel per character
/// ```
pub struct Text {
    /// The string content of this text primitive.
    pub content: String,

    /// The position of the text's starting point (top-left corner).
    pub pos: DiscreteCoord,
}

impl Text {
    /// Creates a new `Text` instance from a string-like value and a position.
    ///
    /// # Parameters
    ///
    /// - `content`: Text content (any type convertible into `String`).
    /// - `pos`: Starting position of the text (top-left coordinate).
    ///
    /// # Returns
    ///
    /// A `Text` instance ready to be rendered.
    pub fn new<S: Into<String>>(content: S, pos: DiscreteCoord) -> Self {
        Text {
            content: content.into(),
            pos,
        }
    }

    /// Converts this `Text` into an ASCII art banner using an optional FIGfont.
    ///
    /// If no font is provided, the standard FIGfont is used by default.
    ///
    /// Returns a vector of `Text` lines representing the ASCII art.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use overture::primitives::text::Text;
    /// use overture::interfaces::geometry::DiscreteCoord;
    /// 
    /// let text = Text::new("Hi", DiscreteCoord::new(0, 0));
    /// let art_lines = text.ascii_art(None); // Uses default standard font
    /// ```
    pub fn ascii_art(&self, font: Option<&figlet_rs::FIGfont>) -> Vec<Text> {
        let font = match font {
            Some(f) => f,
            None => &figlet_rs::FIGfont::standard().unwrap(),
        };
        let figure = font.convert(&self.content).unwrap();

        figure
            .to_string()
            .lines()
            .enumerate()
            .map(|(i, line)| {
                Text::new(line.to_string(), DiscreteCoord::new(
                    self.pos.x,
                    self.pos.y + i as u32,
                ))
            })
            .collect()
    }

    /// Converts this `Text` into ASCII art using a font loaded by name.
    ///
    /// Looks for the font file `fonts/{font_name}.flf` relative to the project.
    ///
    /// # Panics
    ///
    /// Panics if the specified font file cannot be loaded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use overture::primitives::text::Text;
    /// use overture::interfaces::geometry::DiscreteCoord;
    /// 
    /// let text = Text::new("Hello", DiscreteCoord::new(0, 0));
    /// let art = text.ascii_art_by_name("larry3d");
    /// ```
    pub fn ascii_art_by_name(&self, font_name: &str) -> Vec<Text> {
        let font = get_font_by_name(font_name);
        self.ascii_art(Some(&font))
    }

    /// Converts this `Text` into ASCII art using the default standard FIGfont.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use overture::primitives::text::Text;
    /// use overture::interfaces::geometry::DiscreteCoord;
    /// 
    /// let text = Text::new("Hello", DiscreteCoord::new(0, 0));
    /// let art = text.ascii_art_default();
    /// ```
    pub fn ascii_art_default(&self) -> Vec<Text> {
        self.ascii_art(None)
    }
}

impl Renderable for Text {
    /// Converts the text string into a vector of pixels positioned sequentially horizontally.
    ///
    /// Each character corresponds to one pixel at `pos + (index, 0)`.
    fn pixels(&self) -> Vec<Pixel> {
        let mut pixels = vec![];
        let mut pointer = self.pos;
        let unit = DiscreteCoord::new(1, 0);
        for ch in self.content.chars() {
            pixels.push(Pixel::new_with_char(ch, pointer, true));
            pointer += unit;
        }
        pixels
    }

    /// Returns the dimensions of the text as (width, y-position).
    ///
    /// Width is counted as the number of characters in the content.
    fn dim(&self) -> DiscreteCoord {
        DiscreteCoord::new(self.content.chars().count() as u32, self.pos.y)
    }
}

impl Deref for Text {
    type Target = String;

    /// Dereferences `Text` to its inner string content.
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

/// Loads a FIGfont by name from the local `fonts/` directory or returns the standard font.
///
/// # Panics
///
/// Panics if a font file with the given name cannot be found or loaded.
fn get_font_by_name(name: &str) -> figlet_rs::FIGfont {
    if name.eq_ignore_ascii_case("standard") {
        figlet_rs::FIGfont::standard().unwrap()
    } else {
        let filename = format!("fonts/{}.flf", name.to_lowercase());
        figlet_rs::FIGfont::from_file(&filename)
            .unwrap_or_else(|_| panic!("Failed to load font file: {}", filename))
    }
}
