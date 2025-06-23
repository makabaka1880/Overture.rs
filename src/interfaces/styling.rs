// Created by Sean L. on Jun. 23.
// Last Updated by Sean L. on Jun. 24.
// 
// overture.rs
// src/interfaces/styling.rs
// 
// Makabaka1880, 2025. All rights reserved.

//! Terminal styling system for [`Renderable`] objects in Overture.
//!
//! This module defines the [`RenderStyle`] enum for applying ANSI-based styling
//! to characters rendered in the terminal, along with the [`Stylable`] trait
//! to generically enable styling on any renderable element.
//!
//! `RenderStyle` supports recursive composition of ANSI sequences to model
//! chained styling effects (e.g., bold + color).

use crate::{
    interfaces::{pixels::Pixel, rendering::Renderable}, ioopts::ansi::ANSISequence
};


/// Recursive enumeration for defining styling rules of a [`Pixel`] in a terminal UI.
///
/// `RenderStyle` enables layering of ANSI sequences to create compound styles,
/// such as bold + underline + color. It is implemented as a recursive enum for
/// composability, mimicking a styling stack.
///
/// See also [`Stylable`] for applying styles to renderable items.
#[derive(Clone, Debug, PartialEq)]
pub enum RenderStyle {
    /// Indicates an invisible style. Useful for explicitly blanking out characters.
    Nil,

    /// The default terminal style, with no modifications applied.
    Plain,

    /// A styled terminal sequence applied atop another [`RenderStyle`] layer.
    ///
    /// This variant allows recursive composition of ANSI styles by combining an [`ANSISequence`]
    /// with another [`RenderStyle`]. Styles are interpreted in order of nesting.
    ///
    /// # Example
    /// ```rust
    /// use overture::ioopts::ansi::ANSISequence;
    /// use overture::interfaces::styling::RenderStyle;
    ///
    /// let style = RenderStyle::Styled(ANSISequence::Bold,
    ///     Box::new(RenderStyle::Styled(ANSISequence::FgRed,
    ///         Box::new(RenderStyle::Plain)
    ///     ))
    /// );
    /// ```
    Styled(ANSISequence, Box<RenderStyle>),
}

/// Trait for types that support terminal styling using [`RenderStyle`].
///
/// This trait is implemented by renderable UI elements that can have ANSI-based
/// styling applied in a composable way. It is most commonly used to decorate
/// rendered characters or shapes with color, boldness, underlining, etc.
///
/// # Example
/// ```rust
/// use overture::interfaces::{
///     geometry::DiscreteCoord,    // For declaring the primitive's location
///     rendering::Renderable,      // For using `.rasterize()`
///     styling::{Stylable, RenderStyle}
/// };
/// use overture::ioopts::ansi::ANSISequence;
/// use overture::primitives::Text;
/// use overture::style;        // For using the style macro for faster styling
/// 
/// let label = Text::new("hello", DiscreteCoord::ORIGIN);
/// let styled = label.rasterize().style(style![ANSISequence::FgGreen]);
/// ```
pub trait Stylable: Renderable + Sized {
    /// Applies a [`RenderStyle`] to the current item, returning a styled clone.
    ///
    /// This method enables method chaining on renderable elements for styling purposes.
    fn style(&self, style_seq: RenderStyle) -> Vec<Pixel>;
}