// Created by Sean L. on Jun. 23.
// Last Updated by Sean L. on Jun. 23.
// 
// overture.rs
// src/utils/debug.rs
// 
// Makabaka1880, 2025. All rights reserved.

// use crate::{
//     interfaces::{
//         layers::{ MaskPixel, Mask, Layer },
//         pixels::{ Pixel },
//         rendering::{ RenderChar },
//         styling::{ RenderStyle }
//     }
// };

use std::fmt::{Display, Formatter, Result};

impl Display for crate::interfaces::containers::RenderableList {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[OBJO] RenderableList at {}", &self)
    }
}

impl Display for crate::interfaces::geometry::DiscreteCoord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[OBJO] DiscreteCoord ({}, {})", self.x, self.y)
    }
}

impl Display for crate::interfaces::geometry::Translation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[OBJO] Translation ({}, {})", self.x, self.y)
    }
}

impl Display for crate::interfaces::layers::MaskPixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[OBJO] MaskPixel On? {} at {}", self.masked, self.pos)
    }
}