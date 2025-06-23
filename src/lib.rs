// Created by Sean L. on Jun. 23.
// Last Updated by Sean L. on Jun. 23.
// 
// overture.rs
// src/lib.rs
// 
// Entry File. Project started at 2025, Jun. 23.
// Makabaka1880, 2025. All rights reserved.

//! # 🎼 Overture
//!
//! **Overture** is an experimental, opinionated, and ergonomic middleware for building high-performance
//! **text-based user interfaces (TUIs)** in Rust. It offers a clean rendering pipeline built
//! around composable primitives, predictable layout behavior, and ANSI-compatible styling.
//!
//! ## ✨ Highlights
//!
//! - **Composable rendering engine** via the `Renderable` trait.
//! - **Structured ANSI styling** with type-safe escape sequences.
//! - **Protected pixels** to prevent important UI elements from being pruned.
//! - **Flexible layout placement**, including alignment presets and coordinate offsets.
//! - **Modern design patterns** inspired by SwiftUI, TUI toolkits, and declarative UI systems.
//!
//! ## 📦 Modules
//!
//! - [`engine`] — The core rendering engine (`OvertureRenderEngine`).
//! - [`interfaces`] — Traits, data structures, and abstraction interfaces.
//! - [`ioopts`] — ANSI terminal options: styling, cursor control, etc.
//! - [`primitives`] — UI shapes, boxes, and composition-ready widgets.
//! - [`prelude`] — Common types and traits for quick use.
//!
//! ## 🔍 Example
//!
//! ```no_run
//! use overture::prelude::*;
//! 
//! let cols = 100; let rows = 20;
//! let mut engine = OvertureRenderEngine::new(cols, rows - 2);
//! let screen_dim = DiscreteCoord::new(cols, rows);
//! 
//! let banner = primitives::text::Text::new("Welcome To", DiscreteCoord::ORIGIN)
//!     .rasterize()
//!     .prune()
//!     .align(RenderPlacementConfig::CenterStage, screen_dim)
//!     .translate(Translation::new(0, -5))
//!     .style(style![ANSISequence::FgMagenta, ANSISequence::Bold]);
//!
//! let brand = primitives::text::Text::new("Overture", DiscreteCoord::ORIGIN)
//!     .ascii_art_by_name("larry3d")
//!     .rasterize()
//!     .prune()
//!     .align(RenderPlacementConfig::CenterStage, screen_dim)
//!     .translate(Translation::new(0, 2))
//!     .style(style![ANSISequence::FgMagenta, ANSISequence::Bold]);
//! 
//! let box_frame = primitives::shape::SoftBox::new(
//!     DiscreteCoord::new(0, 0),
//!     DiscreteCoord::new(cols - 1, rows - 1)
//! );
//! 
//! engine.load_renderable(banner, None);
//! engine.load_renderable(brand, None);
//! engine.load_renderable(box_frame, None);
//! engine.render(rows as u16);
//! ```
//!
//! This example creates a centered styled text banner and surrounds it with a soft box frame.
//! Each UI element is built using method chaining, enabling fluent configuration.
//! 
//! ```
//! ╭──────────────────────────────────────────────────────────────────────────────────────────────────╮
//! │                                                                                                  │
//! │                                                                                                  │
//! │                                                                                                  │
//! │                                            Welcome To                                            │
//! │                                                                                                  │
//! │                                                                                                  │
//! │               _____                               __                                             │
//! │              /\  __`\                            /\ \__                                          │
//! │              \ \ \/\ \    __  __     __    _ __  \ \ ,_\   __  __   _ __     __                  │
//! │               \ \ \ \ \  /\ \/\ \  /'__`\ /\`'__\ \ \ \/  /\ \/\ \ /\`'__\ /'__`\                │
//! │                \ \ \_\ \ \ \ \_/ |/\  __/ \ \ \/   \ \ \_ \ \ \_\ \\ \ \/ /\  __/                │
//! │                 \ \_____\ \ \___/ \ \____\ \ \_\    \ \__\ \ \____/ \ \_\ \ \____\               │
//! │                  \/_____/  \/__/   \/____/  \/_/     \/__/  \/___/   \/_/  \/____/               │
//! │                                                                                                  │
//! │                                                                                                  │
//! │                                                                                                  │
//! │                                                                                                  │
//! │                                                                                                  │
//! ╰──────────────────────────────────────────────────────────────────────────────────────────────────╯
//! ```
//! 
//! > For a full demo, see `demo::test()`.
//! 
//! ## ⚠️ Status
//!
//! Overture is **currently under active development**. APIs may change rapidly.
//! Not production-ready, but already expressive for hobby or experimental TUI rendering.
//!
//! ## © License
//! Makabaka1880, 2025. All rights reserved.

#[macro_use] mod macros;
pub mod interfaces;
pub mod ioopts;
pub mod engine;
pub mod primitives;
pub mod prelude;

pub mod demo;
pub(crate) mod utils;