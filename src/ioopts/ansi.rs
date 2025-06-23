// Created by Sean L. on Jun. 23.
// Last Updated by Sean L. on Jun. 23.
// 
// overture.rs
// src/ioopts/ansi.rs
// 
// Makabaka1880, 2025. All rights reserved.

//! ANSI Escape Sequences for Terminal Styling and Control
//!
//! This module provides definitions and utilities for working with ANSI escape sequences,
//! which are widely used to control text formatting, colors, and cursor behavior in terminal
//! emulators that support ANSI standards.
//!
//! # Features
//!
//! - **Text Styling:** Constants and enums for common text styles like bold, italic, underline,
//!   blink, strikethrough, and their corresponding "off" sequences to disable those styles.
//! - **Color Support:** Foreground and background color codes for standard and bright colors,
//!   as well as 24-bit RGB colors for precise customization.
//! - **Cursor and Screen Control:** Functions and constants for moving the cursor, saving/restoring
//!   its position, hiding/showing it, and clearing lines or the screen.
//!
//! # Usage
//!
//! The module is internally organized into three submodules.
//! The `ANSISequence` enum encapsulates various ANSI escape sequences in a type-safe manner,
//! with a convenient method [`to_esc_code`](#method.to_esc_code) to retrieve the actual escape code string.
//!
//! # Example
//!
//! ```rust
//! use overture::ioopts::ansi::{ANSISequence};
//!
//! // Print bold red text
//! print!("{}Hello, world!{}", 
//!     ANSISequence::Bold.to_esc_code(), 
//!     ANSISequence::Reset.to_esc_code()
//! );
//! ```
//!
//! # Notes
//!
//! This module assumes the terminal emulator supports ANSI escape sequences, which is standard
//! on most modern terminals including xterm, Windows Terminal, and many others.
//!
//! For advanced cursor movement or extended terminal control sequences, consider expanding
//! the `cursor` submodule or adding new variants to the `ANSISequence` enum.


pub(crate) mod styling {
    pub(crate) const RESET: &str = "\x1b[0m";
    pub(crate) const BOLD: &str = "\x1b[1m";
    pub(crate) const DIM: &str = "\x1b[2m";
    pub(crate) const ITALIC: &str = "\x1b[3m";
    pub(crate) const UNDERLINE: &str = "\x1b[4m";
    pub(crate) const BLINK: &str = "\x1b[5m";
    pub(crate) const UNKNOWN: &str = "\x1b[6m";
    pub(crate) const INVERT: &str = "\x1b[7m";
    pub(crate) const HIDDEN: &str = "\x1b[8m";
    pub(crate) const STRIKETHROUGH: &str = "\x1b[9m";

    pub(crate) const NO_BOLD: &str = "\x1b[21m";
    pub(crate) const NO_DIM: &str = "\x1b[22m";
    pub(crate) const NO_ITALIC: &str = "\x1b[23m";
    pub(crate) const NO_UNDERLINE: &str = "\x1b[24m";
    pub(crate) const NO_BLINK: &str = "\x1b[25m";
    pub(crate) const NO_UNKNOWN: &str = "\x1b[26m";
    pub(crate) const NO_INVERT: &str = "\x1b[27m";
    pub(crate) const NO_HIDDEN: &str = "\x1b[28m";
    pub(crate) const NO_STRIKETHROUGH: &str = "\x1b[29m";
}

pub(crate) mod color {
    pub(crate) const FG_BLACK: &str = "\x1b[30m";
    pub(crate) const FG_RED: &str = "\x1b[31m";
    pub(crate) const FG_GREEN: &str = "\x1b[32m";
    pub(crate) const FG_YELLOW: &str = "\x1b[33m";
    pub(crate) const FG_BLUE: &str = "\x1b[34m";
    pub(crate) const FG_MAGENTA: &str = "\x1b[35m";
    pub(crate) const FG_CYAN: &str = "\x1b[36m";
    pub(crate) const FG_WHITE: &str = "\x1b[37m";

    pub(crate) const FG_BRIGHT_BLACK: &str = "\x1b[90m";
    pub(crate) const FG_BRIGHT_RED: &str = "\x1b[91m";
    pub(crate) const FG_BRIGHT_GREEN: &str = "\x1b[92m";
    pub(crate) const FG_BRIGHT_YELLOW: &str = "\x1b[93m";
    pub(crate) const FG_BRIGHT_BLUE: &str = "\x1b[94m";
    pub(crate) const FG_BRIGHT_MAGENTA: &str = "\x1b[95m";
    pub(crate) const FG_BRIGHT_CYAN: &str = "\x1b[96m";
    pub(crate) const FG_BRIGHT_WHITE: &str = "\x1b[97m";

    pub(crate) const BG_BLACK: &str = "\x1b[40m";
    pub(crate) const BG_RED: &str = "\x1b[41m";
    pub(crate) const BG_GREEN: &str = "\x1b[42m";
    pub(crate) const BG_YELLOW: &str = "\x1b[43m";
    pub(crate) const BG_BLUE: &str = "\x1b[44m";
    pub(crate) const BG_MAGENTA: &str = "\x1b[45m";
    pub(crate) const BG_CYAN: &str = "\x1b[46m";
    pub(crate) const BG_WHITE: &str = "\x1b[47m";

    pub(crate) const BG_BRIGHT_BLACK: &str = "\x1b[100m";
    pub(crate) const BG_BRIGHT_RED: &str = "\x1b[101m";
    pub(crate) const BG_BRIGHT_GREEN: &str = "\x1b[102m";
    pub(crate) const BG_BRIGHT_YELLOW: &str = "\x1b[103m";
    pub(crate) const BG_BRIGHT_BLUE: &str = "\x1b[104m";
    pub(crate) const BG_BRIGHT_MAGENTA: &str = "\x1b[105m";
    pub(crate) const BG_BRIGHT_CYAN: &str = "\x1b[106m";
    pub(crate) const BG_BRIGHT_WHITE: &str = "\x1b[107m";

    pub fn fg_rgb(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[38;2;{};{};{}m", r, g, b)
    }

    pub fn bg_rgb(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[48;2;{};{};{}m", r, g, b)
    }
}
pub(crate) mod cursor {
    pub(crate) fn move_up(n: usize) -> String {
        format!("\x1b[{}A", n)
    }

    pub(crate) fn move_down(n: usize) -> String {
        format!("\x1b[{}B", n)
    }

    pub(crate) fn move_right(n: usize) -> String {
        format!("\x1b[{}C", n)
    }

    pub(crate) fn move_left(n: usize) -> String {
        format!("\x1b[{}D", n)
    }

    pub(crate) fn move_to(row: usize, col: usize) -> String {
        format!("\x1b[{};{}H", row, col)
    }

    pub(crate) fn move_to_column(col: usize) -> String {
        format!("\x1b[{}G", col)
    }

    pub(crate) fn move_to_row(row: usize) -> String {
        format!("\x1b[{}d", row)
    }

    pub(crate) const SAVE_CURSOR: &str = "\x1b[s";
    pub(crate) const RESTORE_CURSOR: &str = "\x1b[u";
    pub(crate) const HIDE_CURSOR: &str = "\x1b[?25l";
    pub(crate) const SHOW_CURSOR: &str = "\x1b[?25h";
    pub(crate) const CLEAR_SCREEN: &str = "\x1b[2J";
    pub(crate) const CLEAR_LINE: &str = "\x1b[2K";
    pub(crate) const ERASE_TO_END_OF_LINE: &str = "\x1b[K";
    pub(crate) const ENABLE_ALTERNATE_SCREEN: &str = "\x1b[?1049h";
    pub(crate) const DISABLE_ALTERNATE_SCREEN: &str = "\x1b[?1049l";
}


/// An enumeration of ANSI escape sequences used for styling terminal output.
///
/// This enum includes:
/// - **Text Styling**: such as bold, italic, underline, etc.
/// - **Color Modifiers**: standard, bright, and 24-bit RGB colors for both foreground and background.
/// - **Screen Control**: (if extended later) like cursor movement or screen clearing.
///
/// Use the [`to_esc_code`](ANSISequence::to_esc_code) method to convert each variant into its corresponding ANSI string.
///
/// # Examples
/// ```rust
/// use overture::ioopts::ansi::ANSISequence;
///
/// let bold_code = ANSISequence::Bold.to_esc_code();
/// let bright_red_text = format!("{}Hello{}", ANSISequence::FgBrightRed.to_esc_code(), ANSISequence::Reset.to_esc_code());
/// ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ANSISequence {
    // MARK: Styling

    /// Reset all styles and colors.
    Reset,
    /// Bold text style.
    Bold,
    /// Dim text style.
    Dim,
    /// Italic text style.
    Italic,
    /// Underlined text style.
    Underline,
    /// Blinking text style.
    Blink,
    /// Inverted foreground and background colors.
    Invert,
    /// Hidden text (invisible).
    Hidden,
    /// Strikethrough text style.
    Strikethrough,

    /// Disable bold text style.
    NoBold,
    /// Disable dim text style.
    NoDim,
    /// Disable italic text style.
    NoItalic,
    /// Disable underline text style.
    NoUnderline,
    /// Disable blinking text style.
    NoBlink,
    /// Disable inverted colors.
    NoInvert,
    /// Disable hidden text style.
    NoHidden,
    /// Disable strikethrough text style.
    NoStrikethrough,

    // MARK: Foreground Colors

    /// Set foreground color to black.
    FgBlack,
    /// Set foreground color to red.
    FgRed,
    /// Set foreground color to green.
    FgGreen,
    /// Set foreground color to yellow.
    FgYellow,
    /// Set foreground color to blue.
    FgBlue,
    /// Set foreground color to magenta.
    FgMagenta,
    /// Set foreground color to cyan.
    FgCyan,
    /// Set foreground color to white.
    FgWhite,

    /// Set bright foreground color to black (gray).
    FgBrightBlack,
    /// Set bright foreground color to red.
    FgBrightRed,
    /// Set bright foreground color to green.
    FgBrightGreen,
    /// Set bright foreground color to yellow.
    FgBrightYellow,
    /// Set bright foreground color to blue.
    FgBrightBlue,
    /// Set bright foreground color to magenta.
    FgBrightMagenta,
    /// Set bright foreground color to cyan.
    FgBrightCyan,
    /// Set bright foreground color to white.
    FgBrightWhite,

    // MARK: Background Colors

    /// Set background color to black.
    BgBlack,
    /// Set background color to red.
    BgRed,
    /// Set background color to green.
    BgGreen,
    /// Set background color to yellow.
    BgYellow,
    /// Set background color to blue.
    BgBlue,
    /// Set background color to magenta.
    BgMagenta,
    /// Set background color to cyan.
    BgCyan,
    /// Set background color to white.
    BgWhite,

    /// Set bright background color to black (gray).
    BgBrightBlack,
    /// Set bright background color to red.
    BgBrightRed,
    /// Set bright background color to green.
    BgBrightGreen,
    /// Set bright background color to yellow.
    BgBrightYellow,
    /// Set bright background color to blue.
    BgBrightBlue,
    /// Set bright background color to magenta.
    BgBrightMagenta,
    /// Set bright background color to cyan.
    BgBrightCyan,
    /// Set bright background color to white.
    BgBrightWhite,

    /// Set foreground color to a custom RGB value using 24-bit color.
    ///
    /// Each parameter represents the intensity of red, green, and blue channels respectively,
    /// with values ranging from 0 to 255. This allows for precise color customization
    /// beyond the standard ANSI color palette.
    ///
    /// # Example
    /// ```rust
    /// use overture::ioopts::ansi::ANSISequence;
    /// 
    /// let custom_fg = ANSISequence::FgRGB(128, 64, 255);
    /// ```
    FgRGB(u8, u8, u8),

    /// Set background color to a custom RGB value using 24-bit color.
    ///
    /// Each parameter represents the intensity of red, green, and blue channels respectively,
    /// with values ranging from 0 to 255. This enables rich background color customization
    /// for terminal output.
    ///
    /// # Example
    /// ```rust
    /// use overture::ioopts::ansi::ANSISequence;
    /// 
    /// let custom_bg = ANSISequence::BgRGB(10, 200, 150);
    /// ```
    BgRGB(u8, u8, u8),

}

impl ANSISequence {
    /// Returns the ANSI escape code string corresponding to the `ANSISequence` variant.
    ///
    /// This method converts an `ANSISequence` enum variant into its actual ANSI escape
    /// code string, which can then be used to style terminal output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use overture::ioopts::ansi::ANSISequence;
    ///
    /// let bold_code = ANSISequence::Bold.to_esc_code();
    /// let reset_code = ANSISequence::Reset.to_esc_code();
    /// let styled_text = format!("{}Hello, world!{}", bold_code, reset_code);
    /// println!("{}", styled_text);
    /// ```
    ///
    /// # Returns
    ///
    /// A `String` containing the ANSI escape sequence corresponding to the variant.
    pub fn to_esc_code(&self) -> String {
        use crate::ioopts::ansi::{styling, color};

        match self {
            // Styling
            ANSISequence::Reset => styling::RESET.to_string(),
            ANSISequence::Bold => styling::BOLD.to_string(),
            ANSISequence::Dim => styling::DIM.to_string(),
            ANSISequence::Italic => styling::ITALIC.to_string(),
            ANSISequence::Underline => styling::UNDERLINE.to_string(),
            ANSISequence::Blink => styling::BLINK.to_string(),
            ANSISequence::Invert => styling::INVERT.to_string(),
            ANSISequence::Hidden => styling::HIDDEN.to_string(),
            ANSISequence::Strikethrough => styling::STRIKETHROUGH.to_string(),

            ANSISequence::NoBold => styling::NO_BOLD.to_string(),
            ANSISequence::NoDim => styling::NO_DIM.to_string(),
            ANSISequence::NoItalic => styling::NO_ITALIC.to_string(),
            ANSISequence::NoUnderline => styling::NO_UNDERLINE.to_string(),
            ANSISequence::NoBlink => styling::NO_BLINK.to_string(),
            ANSISequence::NoInvert => styling::NO_INVERT.to_string(),
            ANSISequence::NoHidden => styling::NO_HIDDEN.to_string(),
            ANSISequence::NoStrikethrough => styling::NO_STRIKETHROUGH.to_string(),

            // Foreground Colors
            ANSISequence::FgBlack => color::FG_BLACK.to_string(),
            ANSISequence::FgRed => color::FG_RED.to_string(),
            ANSISequence::FgGreen => color::FG_GREEN.to_string(),
            ANSISequence::FgYellow => color::FG_YELLOW.to_string(),
            ANSISequence::FgBlue => color::FG_BLUE.to_string(),
            ANSISequence::FgMagenta => color::FG_MAGENTA.to_string(),
            ANSISequence::FgCyan => color::FG_CYAN.to_string(),
            ANSISequence::FgWhite => color::FG_WHITE.to_string(),

            ANSISequence::FgBrightBlack => color::FG_BRIGHT_BLACK.to_string(),
            ANSISequence::FgBrightRed => color::FG_BRIGHT_RED.to_string(),
            ANSISequence::FgBrightGreen => color::FG_BRIGHT_GREEN.to_string(),
            ANSISequence::FgBrightYellow => color::FG_BRIGHT_YELLOW.to_string(),
            ANSISequence::FgBrightBlue => color::FG_BRIGHT_BLUE.to_string(),
            ANSISequence::FgBrightMagenta => color::FG_BRIGHT_MAGENTA.to_string(),
            ANSISequence::FgBrightCyan => color::FG_BRIGHT_CYAN.to_string(),
            ANSISequence::FgBrightWhite => color::FG_BRIGHT_WHITE.to_string(),

            // Background Colors
            ANSISequence::BgBlack => color::BG_BLACK.to_string(),
            ANSISequence::BgRed => color::BG_RED.to_string(),
            ANSISequence::BgGreen => color::BG_GREEN.to_string(),
            ANSISequence::BgYellow => color::BG_YELLOW.to_string(),
            ANSISequence::BgBlue => color::BG_BLUE.to_string(),
            ANSISequence::BgMagenta => color::BG_MAGENTA.to_string(),
            ANSISequence::BgCyan => color::BG_CYAN.to_string(),
            ANSISequence::BgWhite => color::BG_WHITE.to_string(),

            ANSISequence::BgBrightBlack => color::BG_BRIGHT_BLACK.to_string(),
            ANSISequence::BgBrightRed => color::BG_BRIGHT_RED.to_string(),
            ANSISequence::BgBrightGreen => color::BG_BRIGHT_GREEN.to_string(),
            ANSISequence::BgBrightYellow => color::BG_BRIGHT_YELLOW.to_string(),
            ANSISequence::BgBrightBlue => color::BG_BRIGHT_BLUE.to_string(),
            ANSISequence::BgBrightMagenta => color::BG_BRIGHT_MAGENTA.to_string(),
            ANSISequence::BgBrightCyan => color::BG_BRIGHT_CYAN.to_string(),
            ANSISequence::BgBrightWhite => color::BG_BRIGHT_WHITE.to_string(),

            // RGB Colors (dereference tuple fields)
            ANSISequence::FgRGB(r, g, b) => color::fg_rgb(*r, *g, *b),
            ANSISequence::BgRGB(r, g, b) => color::bg_rgb(*r, *g, *b),
        }
    }
}
