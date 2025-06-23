// Created by Sean L. on Jun. 24.
// Last Updated by Sean L. on Jun. 24.
// 
// overture.rs
// src/prelude.rs
// 
// Makabaka1880, 2025. All rights reserved.

//! Commonly used traits and types for Overture.
//! Users can `use overture::prelude::*` to bring these into scope.

pub use crate::interfaces::geometry::DiscreteCoord;
pub use crate::interfaces::geometry::RenderPlacementConfig;
pub use crate::interfaces::geometry::Translation;
pub use crate::interfaces::rendering::Renderable;
pub use crate::interfaces::rendering::RenderChar;
pub use crate::interfaces::containers::RenderableList;
pub use crate::interfaces::pixels::Pixel;
pub use crate::interfaces::styling::RenderStyle;
pub use crate::interfaces::styling::Stylable;

pub use crate::ioopts::ansi::ANSISequence;

pub use crate::primitives;

pub use crate::engine::OvertureRenderEngine;

pub use crate::{style, renderable_list, some};