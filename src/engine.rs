// Created by Sean L. on Jun. 23.
// Last Updated by Sean L. on Jun. 23.
// 
// overture.rs
// src/engine.rs
// 
// Makabaka1880, 2025. All rights reserved.


//! Overture Render Engine
//!
//! This module provides the [`OvertureRenderEngine`] struct, a terminal-based UI engine for rendering
//! styled content onto a fixed-width character grid. It supports styled text rendering, element
//! placement strategies, and renderable object composition via [`RenderableList`].
//!
//! # Usage
//!
//! 1. Construct renderables.
//! 2. Load them using [`OvertureRenderEngine::load_renderable`] with an optional placement config.
//! 3. Call [`OvertureRenderEngine::render`] with the intended height.
//! 4. Call [`OvertureRenderEngine::flush`] to finalize display.
//!
//! # See Also
//!
//! - [`Renderable`] trait for compatible objects.
//! - [`RenderPlacementConfig`] for positioning.
//! - [`RenderChar`] for styled characters.

use std::io::{Write};
use std::cmp::{max};
use crate::interfaces::{
    rendering::{RenderChar, Renderable},
    pixels::Pixel,
    styling::RenderStyle,
    geometry::{DiscreteCoord, RenderPlacementConfig},
    containers::{RenderableList}
};

/// A terminal-based UI engine for rendering styled content onto a fixed-width character grid.
///
/// The `OvertureRenderEngine` maintains a 2D buffer of [`RenderChar`]s and renders
/// them to the terminal using ANSI escape codes. It supports styled text rendering,
/// element placement strategies, and renderable object composition via [`RenderableList`].
///
/// # Overview
///
/// - `width`: Fixed width of the render area (in characters).
/// - `buffer`: 2D screen buffer storing what will be printed to terminal.
/// - `objects`: List of [`Renderable`] elements managed by the engine.
///
/// # Example
///
/// ```rust
/// use overture::prelude::*;
///
/// let mut engine = OvertureRenderEngine::new(40, 20);
/// let label = primitives::text::Text::new("Hello, world!", DiscreteCoord::ORIGIN);  // implements `Renderable`
/// engine.load_renderable(label, Some(RenderPlacementConfig::CenterTop));
/// engine.render(20);
/// ```
///
/// # Rendering Flow
///
/// 1. Construct renderables.
/// 2. Load them using [`load_renderable`] with an optional placement config.
/// 3. Call [`render`] with the intended height.
/// 4. Call [`flush`] to finalize display.
///
/// # See Also
///
/// - [`Renderable`] trait for compatible objects.
/// - [`RenderPlacementConfig`] for positioning.
/// - [`RenderChar`] for styled characters.
pub struct OvertureRenderEngine {
    pub width: u32,
    pub objects: RenderableList,
    pub buffer: Vec<Vec<RenderChar>>,
}


impl OvertureRenderEngine {
    /// Creates a new instance of the Overture render engine with the given width and height.
    ///
    /// Initializes the internal screen buffer with blank `RenderChar`s and sets up an empty
    /// list of objects to render.
    ///
    /// # Parameters
    ///
    /// - `width`: The width (in columns) of the terminal buffer.
    /// - `height`: The height (in rows) of the terminal buffer.
    ///
    /// # Returns
    ///
    /// An `OvertureRenderEngine` with pre-allocated blank space and an empty scene.
    ///
    /// # Example
    /// ```
    /// use overture::prelude::*;
    /// 
    /// let mut engine = OvertureRenderEngine::new(80, 24);
    /// ```
    pub fn new(width: u32, height: u32) -> Self {
        OvertureRenderEngine {
            width,
            objects: RenderableList::new(),
            buffer: vec![vec![RenderChar::BLANK_RENDER_CHAR; width as usize]; height as usize],
        }
    }

    // Sets a specific pixel in the terminal buffer.
    ///
    /// If the target position is outside the current buffer height, the buffer is automatically
    /// extended with blank rows. Pixels outside the width are silently ignored.
    ///
    /// # Parameters
    ///
    /// - `x`: The horizontal coordinate (column).
    /// - `y`: The vertical coordinate (row).
    /// - `ch`: The styled character to place at the given position.
    ///
    /// # Behavior
    ///
    /// - The internal buffer grows **vertically** as needed (never shrinks).
    /// - Horizontal bounds are clamped to avoid index panics.
    ///
    /// # Example
    /// ```
    /// use overture::prelude::*;
    /// 
    /// let mut engine = OvertureRenderEngine::new(80, 24);
    /// engine.set_pixel(10, 5, RenderChar::new_plain('X'));
    /// ```
    pub fn set_pixel(&mut self, x: u32, y: u32, ch: RenderChar) {
        let x = x as usize;
        let y = y as usize;

        while self.buffer.len() <= y {
            self.buffer.push(vec![RenderChar::BLANK_RENDER_CHAR; self.width as usize]);
        }

        if x < self.width as usize {
            self.buffer[y][x] = ch;
        }
    }

    /// Clears the terminal screen by issuing the appropriate ANSI sequence
    /// and flushing `stdout`.
    ///
    /// Typically called at the start of each frame or before rendering a new scene.
    ///
    /// # Example
    /// ```
    /// use overture::prelude::*;
    /// 
    /// let mut engine = OvertureRenderEngine::new(80, 24);
    /// engine.flush(); // Clears screen before a fresh render
    /// ```
    pub fn flush(&self) {
        println!("{}", crate::ioopts::ansi::cursor::CLEAR_SCREEN);
        std::io::stdout().flush().unwrap();
    }

    /// Renders the current screen buffer to the terminal.
    ///
    /// This method walks through each character in the internal buffer and prints them
    /// row by row, applying ANSI styling sequences as needed.
    ///
    /// # Parameters
    ///
    /// - `height`: The minimum number of lines to ensure in the buffer before rendering.
    ///   If the buffer is shorter, it is padded with blank rows.
    ///
    /// # Styling Behavior
    ///
    /// Styled characters (`RenderStyle::Styled`) are recursively printed with nested styles applied.
    /// After each character, ANSI reset (`\x1b[0m`) is emitted to avoid style leakage.
    ///
    /// # Performance Notes
    ///
    /// This method performs a full buffer flush to stdout. For fine-grained updates,
    /// a smarter diff-based renderer may be layered on top later.
    ///
    /// # Example
    /// ```
    /// use overture::prelude::*;
    /// 
    /// let mut engine = OvertureRenderEngine::new(80, 24);
    /// engine.render(24); // Renders a 24-line frame to terminal
    /// ```
    pub fn render(&mut self, height: u16) {
        // Pad buffer to required height
        while self.buffer.len() < height as usize {
            self.buffer.push(vec![RenderChar::BLANK_RENDER_CHAR; self.width as usize]);
        }

        for line in &self.buffer {
            for ch in line {
                match &ch.style {
                    RenderStyle::Plain => print!("{}", ch.ch),
                    RenderStyle::Styled(seq, boxed_style) => {
                        print!("{}", seq.to_esc_code());
                        match **boxed_style {
                            RenderStyle::Plain => print!("{}", ch.ch),
                            RenderStyle::Styled(_, _) => {
                                let inner_ch = RenderChar {
                                    ch: ch.ch,
                                    style: *boxed_style.clone(),
                                };
                                match inner_ch.style {
                                    RenderStyle::Nil => print!("{}", RenderChar::BLANK_RENDER_CHAR.ch),
                                    RenderStyle::Plain => print!("{}", inner_ch.ch),
                                    RenderStyle::Styled(_, _) => {
                                        print!("{}", inner_ch.ch);
                                    }
                                }
                            },
                            RenderStyle::Nil => print!("{}", RenderChar::BLANK_RENDER_CHAR.ch)
                        }
                        print!("{}", crate::ioopts::ansi::styling::RESET);
                    }
                    RenderStyle::Nil => todo!(),
                }
            }
            println!();
        }
        std::io::stdout().flush().unwrap();
    }


        /// Loads a `Renderable` object into the rendering engine with optional placement logic.
    ///
    /// This method normalizes and positions a `Renderable` object into the screen buffer
    /// based on the given placement strategy. It computes the object's bounding box,
    /// aligns it as specified, and then delegates the final rendering to the object itself.
    ///
    /// # Parameters
    ///
    /// - `obj`: The object to render. Must implement the `Renderable` trait.
    /// - `placement`: An optional `RenderPlacementConfig` enum that controls where the object
    ///   should be anchored within the engine’s buffer. If omitted, defaults to `TopLeft`.
    ///
    /// # Behavior
    ///
    /// - The object's pixels are **normalized** to start from `(0, 0)` relative to its own bounds.
    /// - The engine then determines a **target anchor point** in the screen buffer based on the
    ///   placement configuration.
    /// - Finally, the object is rendered at that position using its `render_at` method.
    ///
    /// # Placement Options
    ///
    /// You can align renderables to:
    ///
    /// - `TopLeft`, `TopRight`
    /// - `BottomLeft`, `BottomRight`
    /// - `CenterTop`, `CenterBottom`, `CenterLeft`, `CenterRight`
    /// - `CenterStage` (true center of the screen)
    /// - `Offset(x, y)` — manually specify the top-left corner of the renderable.
    ///
    /// # Example
    ///
    /// ```
    /// use overture::prelude::*;
    /// 
    /// let box = primitive::shape::SoftBox::new(DiscreteCoord::ORIGIN, DiscreteCoord::new(10, 20));
    /// 
    /// let mut engine = OvertureRenderEngine::new(80, 24);
    /// engine.load_renderable(box, Some(RenderPlacementConfig::CenterStage));
    /// ```
    ///
    /// # Notes
    ///
    /// - If the renderable has no pixels (`pixels().is_empty()`), this method is a no-op.
    /// - Out-of-bounds rendering is silently ignored at the pixel level (clipped).
    /// - `protected` pixels are preserved across prunes and reflows.
    pub fn load_renderable<T: Renderable>(&mut self, obj: T, placement: Option<RenderPlacementConfig>) {
        let pixels = obj.pixels();
        if pixels.is_empty() {
            return;
        }

        let min_x = pixels.iter().map(|p| p.position.x).min().unwrap_or(0);
        let min_y = pixels.iter().map(|p| p.position.y).min().unwrap_or(0);
        let normalized_pixels: Vec<Pixel> = pixels
            .iter()
            .map(|p| Pixel::new(p.content.clone(), DiscreteCoord::new(
                p.position.x - min_x,
                p.position.y - min_y,
            ), p.protected))
            .collect();

        let obj_width = normalized_pixels.iter().map(|p| p.position.x).max().unwrap_or(0) + 1;
        let obj_height = normalized_pixels.iter().map(|p| p.position.y).max().unwrap_or(0) + 1;

        let available_width = self.width.saturating_sub(obj_width);
        let available_height = (self.buffer.len() as u32).saturating_sub(obj_height);


        let (x, y) = match placement.unwrap_or(RenderPlacementConfig::TopLeft) {
            RenderPlacementConfig::TopLeft         => (0, 0),
            RenderPlacementConfig::TopRight        => (available_width, 0),
            RenderPlacementConfig::BottomLeft      => (0, available_height),
            RenderPlacementConfig::BottomRight     => (available_width, available_height),
            RenderPlacementConfig::CenterTop       => (available_width / 2, 0),
            RenderPlacementConfig::CenterBottom    => (available_width / 2, available_height),
            RenderPlacementConfig::CenterLeft      => (0, available_height / 2),
            RenderPlacementConfig::CenterRight     => (available_width, available_height / 2),
            RenderPlacementConfig::CenterStage     => (available_width / 2, available_height / 2),
            RenderPlacementConfig::Offset(offset)  => (max(offset.x, 0) as u32, max(offset.y, 0) as u32),
        };

        let pos: DiscreteCoord = DiscreteCoord::new(x, y);
        obj.render_at(pos.x, pos.y, self);
    }
}