// Created by Sean L. on Jun. 24.
// Last Updated by Sean L. on Jun. 24.
// 
// overture.rs
// src/demo.rs
// 
// Makabaka1880, 2025. All rights reserved.

use crate::prelude::*;

use std::io;

pub fn test() {
    let (cols, rows) = (130, 30);
    let mut engine_instace = OvertureRenderEngine::new(cols as u32, (rows - 3) as u32);
    let term_dim = DiscreteCoord::new(cols, rows);
    let border = primitives::shape::SoftBox::
        new(
            DiscreteCoord::ORIGIN, 
            DiscreteCoord::new(cols - 1, rows)
        );
    let rec = primitives::shape::SoftBox::
        new(
            DiscreteCoord::ORIGIN, 
            DiscreteCoord::new(cols * 3 / 4, rows * 2 / 3)
        )
        .rasterize()
        .prune()
        .align(RenderPlacementConfig::CenterStage, term_dim)
        .style(style![ANSISequence::FgCyan]);

    let text = primitives::text::Text::
        new(
            " Intro ",
            DiscreteCoord::ORIGIN
        )
        .rasterize()
        .protect()
        .align(RenderPlacementConfig::CenterStage, term_dim)
        .translate(Translation::new(-((cols * 3 / 8) as i32) + 5, -((rows / 3) as i32)))
        .style(style![ANSISequence::FgBlue])
        .prune();

    let logo= primitives::text::Text::
        new(
            "$",
            DiscreteCoord::ORIGIN
        )
        .ascii_art_by_name("3-d")
        .rasterize()
        .prune()
        .align(RenderPlacementConfig::CenterStage, term_dim)
        .style(style![ANSISequence::FgGreen]).translate(Translation::new(0, -3));

    let group = renderable_list![rec, text.translate(Translation::new(5, 0)), logo];
    engine_instace.load_renderable(
        group, Option::None
    );

    let slogan = primitives::text::Text::
        new(
            "Low-level Rendering Made Simple!",
            DiscreteCoord::ORIGIN
        )
        .rasterize()
        .prune()
        .align(RenderPlacementConfig::CenterStage, term_dim)
        .translate(Translation::new(0, 5))
        .style(style!(ANSISequence::Bold, ANSISequence::FgBrightCyan));

    engine_instace.load_renderable(slogan, Option::None);
    engine_instace.load_renderable(border, Option::None);
    engine_instace.render((rows) as u16);
    let mut _s = String::new();
    let _ = io::stdin().read_line(&mut _s);
}

