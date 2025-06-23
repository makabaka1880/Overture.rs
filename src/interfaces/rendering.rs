// Created by Sean L. on Jun. 23.
// Last Updated by Sean L. on Jun. 24.
// 
// overture.rs
// src/interfaces/render.rs
// 
// Makabaka1880, 2025. All rights reserved.

//! The `render` module defines the core abstractions for characters rendered to the terminal.
//!
//! It introduces the `RenderChar` struct, representing a styled character, and the `Renderable` trait,
//! which enables objects to be converted into pixel representations and placed within a 2D grid.
//!
//! This module plays a foundational role in the Overture rendering engine by:
//! - Representing visual elements as styled characters.
//! - Allowing composable rendering through traits and spatial manipulation (e.g. alignment, translation).
//! - Supporting extensibility for user-defined shapes or widgets.
//!
//! Renderable objects generate `Pixel` values containing `RenderChar`s and can be manipulated
//! before being drawn by `OvertureRenderEngine`.

use crate::{
    engine::OvertureRenderEngine,
    interfaces::{
        geometry::{DiscreteCoord, RenderPlacementConfig, Translation}, pixels::Pixel, styling::RenderStyle
    }
};
use std::{cmp::max, io::StderrLock};

/// A single character intended for rendering in the terminal, with associated style.
///
/// This struct encapsulates a `char` and a `RenderStyle`, allowing styled visual output.
///
/// # Examples
/// ```rust
/// use overture::interfaces::{rendering::RenderChar, styling::RenderStyle};
/// use overture::ioopts::ansi::ANSISequence;
/// use overture::style;            // Imports the style linked list constructor macro for easier styling
/// 
/// let styled = RenderChar::new('X', style![ANSISequence::Bold, ANSISequence::BgBlue]);
/// let plain = RenderChar::new_plain('X');     // Renders a bold `X`on a blue background
/// ```
#[derive(Clone, Debug)]
pub struct RenderChar {
    /// The character to be printed.
    pub ch: char,
    /// The style applied to this character (e.g. bold, color).
    pub style: RenderStyle,
}


impl RenderChar {
    /// Creates a new instance of [`RenderChar`].
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use overture::interfaces::{rendering::RenderChar, styling::RenderStyle};
    /// use overture::ioopts::ansi::ANSISequence;
    /// use overture::style;            // Imports the style linked list constructor macro for easier styling
    /// 
    /// let char = RenderChar::new('a', style![ANSISequence::FgRed]);     // Renders a red `a`
    /// ```
    pub fn new(ch: char, style: RenderStyle) -> Self {
        RenderChar { ch: ch, style: style }
    }

    /// Creates a `RenderChar` with no style (`RenderStyle::Plain`).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use overture::interfaces::rendering::RenderChar;
    /// 
    /// let char = RenderChar::new_plain('a');      // Renders an `a` with no style
    /// ```
    pub fn new_plain(ch: char) -> Self {
        RenderChar { ch: ch, style: RenderStyle::Plain }
    }

    /// A constant blank character with no style. Often used as a default or placeholder.
    pub const BLANK_RENDER_CHAR: RenderChar = RenderChar {
        ch: ' ',
        style: RenderStyle::Plain,
    };
}

impl PartialEq for RenderChar {
    /// Compares two `RenderChar`s by both character and style.
    fn eq(&self, other: &Self) -> bool {
        self.ch == other.ch && self.style == other.style
    }
}

/// Trait for any object that can be rendered as a collection of styled pixels.
///
/// Types implementing `Renderable` must define how to convert themselves into
/// a list of `Pixel`s, and optionally provide logic for spatial manipulation,
/// such as translation, alignment, and pruning.
///
/// This is the core trait used by the rendering engine.
pub trait Renderable {
    /// Returns the set of pixels that represent this renderable.
    fn pixels(&self) -> Vec<Pixel>;
    /// Returns the spatial dimension of this renderable (width × height).
    fn dim(&self) -> DiscreteCoord;

    /// Renders this object at a specific position (`x`, `y`) in the given engine.
    fn render_at(&self, x: u32, y: u32, engine: &mut OvertureRenderEngine) {
        let pixels = self.pixels();
        if pixels.is_empty() { return; }

        for pixel in pixels {
            let px = x + pixel.position.x;
            let py = y + pixel.position.y;
            engine.set_pixel(px, py, pixel.content);
        }
    }

    /// Translates this object by the given offset.
    fn translate(&self, by: Translation) -> Vec<Pixel> {
        self.pixels()
            .iter()
            .map(|x| Pixel::new(x.content.clone(), x.position + by, x.protected))
            .collect()
    }

    /// Aligns this object within a bounding box of size `dim`, according to the placement config.
    ///
    /// The object is first normalized to `(0,0)`, then offset to the correct aligned position.
    fn align(&self, to: RenderPlacementConfig, dim: DiscreteCoord) -> Vec<Pixel> {
        let pixels = self.pixels();

        if pixels.is_empty() {
            return vec![];
        }

        let min_x = pixels.iter().map(|p| p.position.x).min().unwrap_or(0);
        let min_y = pixels.iter().map(|p| p.position.y).min().unwrap_or(0);
        let max_x = pixels.iter().map(|p| p.position.x).max().unwrap_or(0);
        let max_y = pixels.iter().map(|p| p.position.y).max().unwrap_or(0);

        let obj_width = max_x - min_x + 1;
        let obj_height = max_y - min_y + 1;

        let available_width = dim.x.saturating_sub(obj_width);
        let available_height = dim.y.saturating_sub(obj_height);

        let (offset_x, offset_y) = match to {
            RenderPlacementConfig::TopLeft         => (0, 0),
            RenderPlacementConfig::TopRight        => (available_width, 0),
            RenderPlacementConfig::BottomLeft      => (0, available_height),
            RenderPlacementConfig::BottomRight     => (available_width, available_height),
            RenderPlacementConfig::CenterTop       => (available_width / 2, 0),
            RenderPlacementConfig::CenterBottom    => (available_width / 2, available_height),
            RenderPlacementConfig::CenterLeft      => (0, available_height / 2),
            RenderPlacementConfig::CenterRight     => (available_width, available_height / 2),
            RenderPlacementConfig::CenterStage     => (available_width / 2, available_height / 2),
            RenderPlacementConfig::Offset(offset)  => (offset.x as u32, offset.y as u32),
        };


        let total_translation = Translation::new(
            offset_x as i32 - min_x as i32,
            offset_y as i32 - min_y as i32,
        );

        self.translate(total_translation)
    }

    /// Produces the final pixel representation of the object.
    ///
    /// This method may eventually include font rendering, style application, etc.
    fn rasterize(&self) -> Vec<Pixel> {
        self.pixels()
    }

    /// Filters out non-essential pixels by removing those that contain the blank character (`' '`),
    /// unless they are explicitly marked as protected.
    ///
    /// This method helps optimize rendering performance by skipping over visual "padding"
    /// or transparent areas that don't need to be redrawn — common in text UI layouts.
    ///
    /// Pixels can be marked as `protected` to avoid accidental pruning of important whitespace,
    /// such as spacing or alignment characters in decorated UI elements.
    ///
    /// # Recommended Use
    ///
    /// Use `prune` after transforming or composing UI elements to eliminate redundant pixels,
    /// especially in generated or dynamically translated content.
    ///
    /// To prevent important blanks from being pruned, use [`protect`] or [`set_protect`] beforehand.
    fn prune(&self) -> Vec<Pixel> {
        self.rasterize()
            .into_iter()
            .filter(|pixel| (pixel.content.ch != RenderChar::BLANK_RENDER_CHAR.ch) || pixel.protected  )
            .collect()
    }

    /// Returns a copy of the renderable's pixels, with all pixels marked as protected.
    ///
    /// This ensures that these pixels will **not** be removed by [`prune`], even if their character
    /// content is the blank character (`' '`).
    ///
    /// # Use Case
    ///
    /// This is useful when building UI components with intentional spacing — for example, soft boxes,
    /// banners, or alignment gutters — where trimming blanks would break layout integrity.
    fn protect(&self) -> Vec<Pixel> {
        let list: Vec<Pixel> = self.pixels().iter()
        .map(|p| Pixel::new(p.content.to_owned(), p.position, true))
        .collect();
        list
    }


    /// Returns a copy of the renderable's pixels, setting the `protected` flag
    /// to the given `bool` for each pixel.
    ///
    /// Unlike [`protect`], which sets all flags to `true`, this method provides
    /// precise control over protection status in dynamic cases.
    ///
    /// # Use Case
    ///
    /// You may want to conditionally toggle protection for specific rendering passes
    /// or propagate flags from user input or layout rules.
    fn set_protect(&self, protect: bool) -> Vec<Pixel> {
        self.pixels().iter()
        .map(|p| Pixel::new(p.content.to_owned(), p.position, protect))
        .collect()
    }


}

impl<T: Renderable> Renderable for Vec<T> {
    /// Combines the pixels of each renderable in the vector into a single list.
    fn pixels(&self) -> Vec<Pixel> {
        self.iter().flat_map(|x| x.pixels()).collect()
    }
    /// Returns the bounding box dimension covering all renderables in the vector.
    fn dim(&self) -> DiscreteCoord {
        self.iter()
            .flat_map(|r| r.pixels())
            .fold(DiscreteCoord::ORIGIN, |acc, p| {
                DiscreteCoord::new(acc.x.max(p.position.x), acc.y.max(p.position.y))
            })
    }
}