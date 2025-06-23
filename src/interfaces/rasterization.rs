// Created by Sean L. on Jun. 23.
// Last Updated by Sean L. on Jun. 24.
// 
// overture.rs
// src/interfaces/clusters.rs
// 
// Makabaka1880, 2025. All rights reserved.

//! Internal implementations of [`Renderable`] and [`Stylable`] for pixel clusters.
//!
//! This module provides trait implementations to treat a vector of [`Pixel`]s
//! as a composite renderable entity and enables applying styles to all contained pixels.
//!
//! This enables collections of pixels to be manipulated and rendered as a unit within the engine.

use crate::interfaces::{
    rendering::{Renderable, RenderChar},
    styling::{RenderStyle, Stylable},
    pixels::{Pixel},
    geometry::{DiscreteCoord},
};

impl Renderable for Vec<Pixel> {
    fn pixels(&self) -> Vec<Pixel> {
        self.iter().map(|p| Pixel {
            content: p.content.clone(),
            position: p.position,
            protected: p.protected
        }).collect()
    }

    fn dim(&self) -> DiscreteCoord {
        if self.is_empty() {
            DiscreteCoord::ORIGIN
        } else {
            let (max_x, max_y) = self.iter().fold((0, 0), |(mx, my), p| {
                (
                    mx.max(p.position.x),
                    my.max(p.position.y),
                )
            });
            DiscreteCoord::new(max_x + 1, max_y + 1)
        }
    }
}

impl Stylable for Vec<Pixel> {
    fn style(&self, style_seq: RenderStyle) -> Self {
        self.iter()
            .map(|x| {
                let styled_content = match &style_seq {
                    RenderStyle::Styled(seq, other) => {
                        RenderChar::new(x.content.ch, RenderStyle::Styled(seq.to_owned(), other.to_owned()))
                    }
                    _ => x.content.clone(),
                };
                
                Pixel::new(styled_content, x.position, x.protected)
            })
            .collect()
    }
}