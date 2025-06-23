# 🎼 Overture.rs

**Overture** is an experimental, opinionated, and ergonomic middleware for building high-performance
**text-based user interfaces (TUIs)** in Rust. It offers a clean rendering pipeline built
around composable primitives, predictable layout behavior, and ANSI-compatible styling.

## ✨ Highlights

- **Composable rendering engine** via the `Renderable` trait.
- **Structured ANSI styling** with type-safe escape sequences.
- **Protected pixels** to prevent important UI elements from being pruned.
- **Flexible layout placement**, including alignment presets and coordinate offsets.
- **Modern design patterns** inspired by SwiftUI, TUI toolkits, and declarative UI systems.

## 📦 Modules

- [`engine`] — The core rendering engine (`OvertureRenderEngine`).
- [`interfaces`] — Traits, data structures, and abstraction interfaces.
- [`ioopts`] — ANSI terminal options: styling, cursor control, etc.
- [`primitives`] — UI shapes, boxes, and composition-ready widgets.
- [`prelude`] — Common types and traits for quick use.

## 🔍 Example

```rust
use overture::prelude::*;

// Set up the screen dimensions
let cols = 100;
let rows = 20;

// Create a new rendering engine with the desired size
let mut engine = OvertureRenderEngine::new(cols, rows - 2);
let screen_dim = DiscreteCoord::new(cols, rows);

// Create a centered, styled text banner
let banner = primitives::text::Text::new("Welcome To", DiscreteCoord::ORIGIN)
    .rasterize()                                                    // Convert text to a renderable raster
    .prune()                                                        // Remove unnecessary whitespace
    .align(RenderPlacementConfig::CenterStage, screen_dim)          // Center on screen
    .translate(Translation::new(0, -5))                             // Move up by 5 rows
    .style(style![ANSISequence::FgMagenta, ANSISequence::Bold]);    // Apply magenta color and bold style

// Create a centered ASCII art brand below the banner
let brand = primitives::text::Text::new("Overture", DiscreteCoord::ORIGIN)
    .ascii_art_by_name("larry3d")                                   // Render as ASCII art using font 'larry3d'
    .rasterize()
    .prune()
    .align(RenderPlacementConfig::CenterStage, screen_dim)
    .translate(Translation::new(0, 2))                              // Move down by 2 rows
    .style(style![ANSISequence::FgMagenta, ANSISequence::Bold]);

// Create a soft box frame around the entire screen
let box_frame = primitives::shape::SoftBox::new(
    DiscreteCoord::new(0, 0),
    DiscreteCoord::new(cols - 1, rows - 1)
);

// Load all UI elements into the engine
engine.load_renderable(banner, None);
engine.load_renderable(brand, None);
engine.load_renderable(box_frame, None);

// Render the final output to the terminal
engine.render(rows as u16);
```

This example creates a centered styled text banner and surrounds it with a soft box frame.
Each UI element is built using method chaining, enabling fluent configuration.

```
╭──────────────────────────────────────────────────────────────────────────────────────────────────╮
│                                                                                                  │
│                                                                                                  │
│                                                                                                  │
│                                            Welcome To                                            │
│                                                                                                  │
│                                                                                                  │
│               _____                               __                                             │
│              /\  __`\                            /\ \__                                          │
│              \ \ \/\ \    __  __     __    _ __  \ \ ,_\   __  __   _ __     __                  │
│               \ \ \ \ \  /\ \/\ \  /'__`\ /\`'__\ \ \ \/  /\ \/\ \ /\`'__\ /'__`\                │
│                \ \ \_\ \ \ \ \_/ |/\  __/ \ \ \/   \ \ \_ \ \ \_\ \\ \ \/ /\  __/                │
│                 \ \_____\ \ \___/ \ \____\ \ \_\    \ \__\ \ \____/ \ \_\ \ \____\               │
│                  \/_____/  \/__/   \/____/  \/_/     \/__/  \/___/   \/_/  \/____/               │
│                                                                                                  │
│                                                                                                  │
│                                                                                                  │
│                                                                                                  │
│                                                                                                  │
╰──────────────────────────────────────────────────────────────────────────────────────────────────╯
```

> For a full demo, see `demo::test()`.

## ⚠️ Status

Overture is **currently under active development**. APIs may change rapidly.
Not production-ready, but already expressive for hobby or experimental TUI rendering.

---

Makabaka1880, 2025. All rights reserved.