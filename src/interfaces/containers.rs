// Created by Sean L. on Jun. 23.
// Last Updated by Sean L. on Jun. 24.
// 
// overture.rs
// src/interfaces/containers.rs
// 
// Makabaka1880, 2025. All rights reserved.
//! A composite container for chaining multiple [`Renderable`] objects together.
//!
//! `RenderableList` implements the composite design pattern for terminal rendering,
//! enabling a list of heterogeneous renderable objects to be grouped and rendered
//! as a single unit.
//!
//! Internally, it uses a linked structure (`Link` + `Nil`) for simplicity and recursion-based rendering.
//!
//! # Examples
//! ```rust
//! use overture::interfaces::containers::RenderableList;
//! use overture::primitives::shape::Rectangle;
//! use overture::interfaces::geometry::DiscreteCoord;
//! use overture::interfaces::rendering::Renderable;
//!
//! let rect = Rectangle::new(DiscreteCoord::new(0, 0), DiscreteCoord::new(10, 5));
//!
//! let group = RenderableList::from_items(vec![rect]);
//! ```

use crate::interfaces::{
    geometry::DiscreteCoord,
    pixels::Pixel,
    rendering::Renderable,
};

/// A recursive list of `Renderable` objects, forming a composite renderable group.
///
/// This structure allows you to batch multiple renderable objects together
/// and treat them as one, enabling recursive rendering and pixel collection.
pub enum RenderableList {
    /// A renderable element and the rest of the list.
    Link(Box<dyn Renderable>, Box<RenderableList>),

    /// The end of the list.
    Nil,
}

impl RenderableList {
    /// Creates an empty `RenderableList`.
    pub fn new() -> Self {
        RenderableList::Nil
    }

    /// Builds a `RenderableList` from a vector of boxed renderables.
    ///
    /// # Parameters
    /// - `a`: A vector of `Box<dyn Renderable>` items.
    ///
    /// # Returns
    /// A `RenderableList` containing all elements.
    pub fn build_from_vector(a: Vec<Box<dyn Renderable>>) -> Self {
        a.into_iter()
            .rfold(RenderableList::Nil, |acc, item| {
                RenderableList::Link(item, Box::new(acc))
            })
    }

    /// Builds a `RenderableList` from a vector of owned renderable items.
    ///
    /// Automatically boxes the items and constructs the list.
    ///
    /// # Type Parameters
    /// - `T`: A concrete type implementing `Renderable`.
    pub fn from_items<T: Renderable + 'static>(items: Vec<T>) -> Self {
        let boxed: Vec<Box<dyn Renderable>> = items
            .into_iter()
            .map(|item| Box::new(item) as Box<dyn Renderable>)
            .collect();
        RenderableList::build_from_vector(boxed)
    }

    /// Returns the number of renderable elements in the list.
    pub fn len(&self) -> usize {
        match self {
            RenderableList::Link(_, tail) => 1 + tail.len(),
            RenderableList::Nil => 0,
        }
    }

    /// Returns `true` if the list is empty.
    pub fn is_empty(&self) -> bool {
        matches!(self, RenderableList::Nil)
    }

    /// Iterates over the items in the list as `&dyn Renderable`.
    pub fn iter<'a>(&'a self) -> RenderableListIter<'a> {
        RenderableListIter { current: Some(self) }
    }
}

/// An iterator over references to the `Renderable` objects in a `RenderableList`.
pub struct RenderableListIter<'a> {
    current: Option<&'a RenderableList>,
}

impl<'a> Iterator for RenderableListIter<'a> {
    type Item = &'a dyn Renderable;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(RenderableList::Link(head, tail)) => {
                self.current = Some(tail);
                Some(head.as_ref())
            }
            _ => {
                self.current = None;
                None
            }
        }
    }
}

impl Renderable for RenderableList {
    /// Recursively collects pixels from all elements in the list.
    fn pixels(&self) -> Vec<Pixel> {
        match self {
            RenderableList::Link(head, tail) => {
                let mut pixels = head.pixels();
                pixels.extend(tail.pixels());
                pixels
            }
            RenderableList::Nil => Vec::new(),
        }
    }

    /// Returns the dimension of the first renderable element (not cumulative).
    ///
    /// Useful when you're treating the list as a group, but not for layout calculations.
    fn dim(&self) -> DiscreteCoord {
        match self {
            RenderableList::Link(head, _) => head.dim(),
            RenderableList::Nil => DiscreteCoord { x: 0, y: 0 },
        }
    }
}
