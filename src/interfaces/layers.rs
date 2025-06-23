// Created by Sean L. on Jun. 23.
// Last Updated by Sean L. on Jun. 24.
// 
// overture.rs
// src/interfaces/layers.rs
// 
// Makabaka1880, 2025. All rights reserved.

use interfaces::rendering::Renderable;

use super::geometry::DiscreteCoord;

#[derive(Clone)]
pub struct MaskPixel {
    pub masked: bool,
    pub pos: DiscreteCoord
}

impl MaskPixel {
    fn new(pos: DiscreteCoord) -> Self {
        MaskPixel { masked: true, pos }
    }
}

pub type Mask = Vec<MaskPixel>;

trait Maskable {
    fn mask(&self) -> Vec<MaskPixel>;
}

impl Maskable for MaskPixel {
    fn mask(&self) -> Vec<MaskPixel> {
        vec![self.clone()]
    }
}

impl Maskable for Mask {
    fn mask(&self) -> Vec<MaskPixel> {
        self.to_owned()
    }
}

pub struct Layer {
    // content: dyn Renderable,
    pub mask: Option<Mask>
}