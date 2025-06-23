// Created by Sean L. on Jun. 23.
// Last Updated by Sean L. on Jun. 23.
// 
// overture.rs
// src/ioopts/box_drawing.rs
// 
// Makabaka1880, 2025. All rights reserved.

//! Provides Unicode constants for double line box drawing characters.
//!
//! This module defines constants for various double line box drawing elements,
//! such as corners, lines, T-junctions, and crossings. These can be used to
//! construct tables, frames, or other box-based UI elements in terminal applications.

/// Provides Unicode box-drawing characters for constructing text-based boxes and tables.
pub mod box_drawing {
    // Corners

    /// Left upper corner (┌)
    pub const LU_CORNER: char = '┌';

    /// Right upper corner (┐)
    pub const RU_CORNER: char = '┐';

    /// Left lower corner (└)
    pub const LD_CORNER: char = '└';

    /// Right lower corner (┘)
    pub const RD_CORNER: char = '┘';

    // Curved Corners

    /// Left upper curved corner (╭)
    pub const LU_CORNER_SOFT: char = '╭';

    /// Right upper curved corner (╮)
    pub const RU_CORNER_SOFT: char = '╮';

    /// Left lower curved corner (╰)
    pub const LD_CORNER_SOFT: char = '╰';

    /// Right lower curved corner (╯)
    pub const RD_CORNER_SOFT: char = '╯';

    // Edges

    /// Horizontal line (─)
    pub const H_LINE: char = '─';

    /// Vertical line (│)
    pub const V_LINE: char = '│';

    // Junctions

    /// T-junction pointing down (┬)
    pub const T_DOWN: char = '┬';

    /// T-junction pointing up (┴)
    pub const T_UP: char = '┴';

    /// T-junction pointing left (┤)
    pub const T_LEFT: char = '┤';

    /// T-junction pointing right (├)
    pub const T_RIGHT: char = '├';

    /// Cross junction (┼)
    pub const CROSS: char = '┼';
}

/// Provides Unicode box-drawing characters for double-lined boxes,
/// useful for rendering tables or UI elements with a fancier appearance.
pub mod box_drawing_double {
    /// Unicode character for the upper-left double line box drawing corner.
    pub const LU_CORNER_D: char = '╔';

    /// Unicode character for the upper-right double line box drawing corner.
    pub const RU_CORNER_D: char = '╗';

    /// Unicode character for the lower-left double line box drawing corner.
    pub const LD_CORNER_D: char = '╚';

    /// Unicode character for the lower-right double line box drawing corner.
    pub const RD_CORNER_D: char = '╝';

    /// Unicode character for the horizontal double line box drawing.
    pub const H_LINE_D: char = '═';

    /// Unicode character for the vertical double line box drawing.
    pub const V_LINE_D: char = '║';

    /// Unicode character for the double line T-junction pointing down.
    pub const T_DOWN_D: char = '╦';

    /// Unicode character for the double line T-junction pointing up.
    pub const T_UP_D: char = '╩';

    /// Unicode character for the double line T-junction pointing left.
    pub const T_LEFT_D: char = '╣';

    /// Unicode character for the double line T-junction pointing right.
    pub const T_RIGHT_D: char = '╠';

    /// Unicode character for the double line crossing.
    pub const CROSS_D: char = '╬';
}