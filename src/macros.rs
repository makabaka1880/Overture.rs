// Created by Sean L. on Jun. 23.
// Last Updated by Sean L. on Jun. 23.
// 
// overture.rs
// src/macros.rs
// 
// Makabaka1880, 2025. All rights reserved.

//! A collection of macros used throughout the Overture rendering framework.
//!
//! These macros assist in building renderable lists, styling chains,
//! and conveniently wrapping values in `Some`.

/// This module provides a collection of procedural macros to simplify the construction
/// of common data structures and patterns used throughout the overture crate.
/// 
/// # Provided Macros
/// 
/// - [`renderable_list!`]: Constructs a `RenderableList` from a sequence of renderable expressions,
///   expanding to a linked-list-like structure.
/// - [`style!`]: Builds a recursive style chain using `RenderStyle::Styled`, with a fallback to `RenderStyle::Plain`.
/// - [`some!`]: A convenience macro for wrapping a value in `Option::Some`.
/// 
/// These macros are designed to improve ergonomics and reduce boilerplate when working with
/// renderable lists, style chains, and optional values in the overture ecosystem.
pub mod macros {
    /// Constructs a `RenderableList` using a list of renderable expressions.
    ///
    /// This macro expands to a linked list-like structure using
    /// `RenderableList::Link` and `RenderableList::Nil`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use overture::prelude::*;
    /// 
    /// let list = renderable_list![
    ///     DummyRenderable,
    ///     DummyRenderable
    /// ];
    ///
    /// match list {
    ///     RenderableList::Link(_, _) => { /* success */ },
    ///     _ => panic!("Expected a linked list"),
    /// }
    /// ```
    #[macro_export]
    macro_rules! renderable_list {
        () => {
            $crate::interfaces::containers::RenderableList::Nil
        };
        ($head:expr $(, $tail:expr)* $(,)?) => {
            $crate::interfaces::containers::RenderableList::Link(
                Box::new($head),
                Box::new(renderable_list![$($tail),*])
            )
        };
    }

    /// Constructs a `RenderStyle` by chaining style elements.
    ///
    /// This macro builds up a recursive style chain using
    /// `RenderStyle::Styled`, with the final fallback being `RenderStyle::Plain`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use overture::prelude::*;
    ///
    /// let style_chain = style![ANSISequence::Bold, ANSISequence::Underline];
    ///
    /// match style_chain {
    ///     RenderStyle::Styled(_, _) => { /* success */ },
    ///     _ => panic!("Expected styled chain"),
    /// }
    /// ```
    #[macro_export]
    macro_rules! style {
        () => {
            $crate::interfaces::styling::RenderStyle::Plain
        };
        ($head:expr $(, $tail:expr)* $(,)?) => {
            $crate::interfaces::styling::RenderStyle::Styled(
                $head,
                Box::new(style![$($tail),*])
            )
        };
    }

    /// A convenience macro for `Some(expr)`.
    ///
    /// Useful for quickly wrapping values in an `Option::Some`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use overture::prelude::*;
    ///
    /// let maybe_value = some!(42);
    /// assert_eq!(maybe_value, Some(42));
    /// ```
    #[macro_export]
    macro_rules! some {
        ($x:expr) => {
            Option::Some($x)
        };
    }
}
