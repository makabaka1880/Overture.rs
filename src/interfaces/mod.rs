// Created by Sean L. on Jun. 23.
// Last Updated by Sean L. on Jun. 24.
// 
// Overture.rs
// src/interfaces/mod.rs
// 
// Makabaka1880, 2025. All rights reserved.

//! The `interfaces` module defines core building blocks for rendering.
//!
//! It provides:
//! - `geometry`: 2D coordinates and layout configurations.
//! - `rendering`: Traits and logic for rendering units to terminal.
//! - `pixel`: Representation of a renderable terminal cell.
//! - `styling`: Style application using ANSI sequences.
//! - `rasterization`: Trait impls that convert structures into pixels.
//! - `containers`: Renderable containers without type erasure.
//! - `layers`: Provides API for opacity and layering

pub mod geometry;
pub mod rendering;
pub mod pixels;
pub mod rasterization;
pub mod styling;
pub mod containers;
pub mod layers;