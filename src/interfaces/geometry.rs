// Created by Sean L. on Jun. 23.
// Last Updated by Sean L. on Jun. 24.
//
// overture.rs
// src/interfaces/vector.rs
//
// Makabaka1880, 2025. All rights reserved.

//! Module `vector` provides core abstractions and implementations for 2D unsigned vectors.
//!
//! This module defines the [`UnsignedR2DVector`] trait for working with vectors in the
//! positive quadrant (using `u32` coordinates), as well as the [`DiscreteCoord`] struct,
//! a fundamental implementation representing grid coordinates.
//!
//! The components support coordinate accessors, vector arithmetic (addition, subtraction),
//! and are designed primarily for use in grid-based spatial computations, rendering,
//! and layout tasks.
//!
//! # Key Types
//!
//! - [`UnsignedR2DVector`]: Trait for 2D vectors with non-negative coordinates.
//! - [`DiscreteCoord`]: Concrete struct implementing `UnsignedR2DVector` for discrete coordinates.
//!
//! # Examples
//!
//! ```rust
//! use overture::interfaces::geometry::{DiscreteCoord, UnsignedR2DVector};
//!
//! let a = DiscreteCoord::new(1, 2);
//! let b = DiscreteCoord::new(3, 4);
//! let c = a.add(b);
//! assert_eq!(c, DiscreteCoord::new(4, 6));
//! ```

use std::ops::{Add, Sub, AddAssign, SubAssign, Neg};
use std::cmp::{max};

/// A trait representing a two-dimensional unsigned vector with basic coordinate accessors and addition.
///
/// This trait is intended for vector types that operate in the first quadrant with non-negative coordinates (`u32`).
///
/// # Methods
///
/// - `x()` and `y()` return the horizontal and vertical components.
/// - `add(self, other)` returns the vector sum of two vectors.
///
/// # Examples
///
/// ```
/// use overture::interfaces::geometry::UnsignedR2DVector;
/// 
/// #[derive(Debug, PartialEq)]
/// struct DiscreteCoord {
///     x: u32,
///     y: u32,
/// }
///
/// impl UnsignedR2DVector for DiscreteCoord {
///     fn x(&self) -> u32 { self.x }
///     fn y(&self) -> u32 { self.y }
///
///     fn add(self, other: Self) -> Self {
///         DiscreteCoord {
///             x: self.x + other.x,
///             y: self.y + other.y,
///         }
///     }
/// }
///
/// let a = DiscreteCoord { x: 2, y: 3 };
/// let b = DiscreteCoord { x: 4, y: 5 };
/// let c = a.add(b);
/// assert_eq!(c, DiscreteCoord { x: 6, y: 8 });
/// ```

pub trait UnsignedR2DVector {
    /// Returns the horizontal component of the vector.
    fn x(&self) -> u32;

    /// Returns the vertical component of the vector.
    fn y(&self) -> u32;

    /// Returns the vector sum of this vector and another.
    ///
    /// # Parameters
    ///
    /// - `other`: The vector to add.
    ///
    /// # Returns
    ///
    /// A new vector representing the coordinate-wise sum.
    fn add(self, other: Self) -> Self;
}

/// A structure representing a discrete, unsigned 2D coordinate in the first quadrant.
///
/// In overture, `DiscreteCoord` is used to model spatial positions and sizes in grid-based systems.
/// It implements the [`UnsignedR2DVector`] trait and provides convenience methods for
/// coordinate arithmetic and transformation.
///
/// This type is central to systems that deal with rendering, layout, and tile-based
/// computations, such as terminal UI engines and 2D games.
///
/// # Examples
///
/// ```
/// use overture::interfaces::geometry::{DiscreteCoord, UnsignedR2DVector};
///
/// let a = DiscreteCoord::new(2, 3);
/// let b = DiscreteCoord::new(1, 1);
/// let sum = a + b;
/// assert_eq!(sum, DiscreteCoord::new(3, 4));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiscreteCoord {
    /// The x-coordinate of the point.
    pub x: u32,

    /// The y-coordinate of the point.
    pub y: u32,
}

impl DiscreteCoord {
    /// Creates a new `DiscreteCoord` from the given `x` and `y` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use overture::interfaces::geometry::DiscreteCoord;
    /// 
    /// let coord = DiscreteCoord::new(5, 10);
    /// assert_eq!(coord.x, 5);
    /// assert_eq!(coord.y, 10);
    /// ```
    pub fn new(x: u32, y: u32) -> Self {
        DiscreteCoord { x, y }
    }

    /// Creates a new `DiscreteCoord` from given, probably non-positive `x` and `y` values. Clamps at 0
    /// Used in scenarios where underflow occurs but requires no immediate panic to terminate the program.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use overture::interfaces::geometry::{Translation, DiscreteCoord};
    /// 
    /// let trans_coord = Translation::new(-5, 10);
    /// let coord = DiscreteCoord::new_from_signed(trans_coord);
    /// assert_eq!(coord.x, 0);
    /// assert_eq!(coord.y, 10);
    /// ```
    pub fn new_from_signed(coord: Translation) -> Self {
        DiscreteCoord { x: max(0, coord.x) as u32, y: max(0, coord.y) as u32 }
    }

    /// Creates a `Translation` instance based on the current `DiscreteCoord`.
    /// Helper method in rendering calculations.
    pub fn to_translation(&self) -> Translation {
        Translation::new(self.x as i32, self.y as i32)
    }

    /// The constant representing the origin point `(0, 0)`.
    ///
    /// Useful as a default or base point for transformations.
    pub const ORIGIN: DiscreteCoord = DiscreteCoord { x: 0, y: 0 };
}

impl UnsignedR2DVector for DiscreteCoord {
    /// Returns the x-coordinate.
    fn x(&self) -> u32 { self.x }

    /// Returns the y-coordinate.
    fn y(&self) -> u32 { self.y }

    /// Adds two 2D vectors component-wise.
    ///
    /// # Examples
    ///
    /// ```
    /// use overture::interfaces::geometry::{DiscreteCoord, UnsignedR2DVector};
    /// 
    /// let a = DiscreteCoord::new(1, 2);
    /// let b = DiscreteCoord::new(3, 4);
    /// let result = a.add(b);
    /// assert_eq!(result, DiscreteCoord::new(4, 6));
    /// ```
    fn add(self, other: Self) -> Self {
        DiscreteCoord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for DiscreteCoord {
    /// Adds another coordinate to `self` in-place.
    ///
    /// # Examples
    ///
    /// ```
    /// use overture::interfaces::geometry::DiscreteCoord;
    /// 
    /// let mut a = DiscreteCoord::new(2, 3);
    /// a += DiscreteCoord::new(4, 1);
    /// assert_eq!(a, DiscreteCoord::new(6, 4));
    /// ```
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Add for DiscreteCoord {
    type Output = DiscreteCoord;

    /// Adds two coordinates component-wise.
    ///
    /// # Examples
    ///
    /// ```
    /// use overture::interfaces::geometry::DiscreteCoord;
    /// 
    /// let a = DiscreteCoord::new(3, 5);
    /// let b = DiscreteCoord::new(2, 2);
    /// let result = a + b;
    /// assert_eq!(result, DiscreteCoord::new(5, 7));
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        DiscreteCoord::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for DiscreteCoord {
    type Output = DiscreteCoord;

    /// Subtracts two coordinates component-wise, clamping at zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use overture::interfaces::geometry::DiscreteCoord;
    /// 
    /// let a = DiscreteCoord::new(5, 4);
    /// let b = DiscreteCoord::new(7, 6);
    /// let result = a - b;
    /// assert_eq!(result, DiscreteCoord::new(0, 0));
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        DiscreteCoord::new(
            self.x.saturating_sub(rhs.x),
            self.y.saturating_sub(rhs.y),
        )
    }
}

impl SubAssign for DiscreteCoord {
    /// Subtracts another coordinate from `self` in-place, clamping at zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use overture::interfaces::geometry::DiscreteCoord;
    /// 
    /// let mut a = DiscreteCoord::new(5, 5);
    /// a -= DiscreteCoord::new(7, 2);
    /// assert_eq!(a, DiscreteCoord::new(0, 3));
    /// ```
    fn sub_assign(&mut self, other: Self) {
        self.x = self.x.saturating_sub(other.x);
        self.y = self.y.saturating_sub(other.y);
    }
}

/// Represents a signed 2D translation vector.
///
/// This struct is commonly used to offset or shift grid-based coordinates,
/// such as `DiscreteCoord`, which represents unsigned (non-negative) 2D points. 
/// Ensures robustness by clamping at 0 rather than panicking at underflows.
/// 
/// It supports addition, subtraction, and negation, enabling easy movement and positioning
/// logic in spatial computations.
///
/// # Examples
///
/// ```rust
/// use overture::interfaces::geometry::{Translation, DiscreteCoord};
///
/// let t = Translation::new(-3, 5);
/// let pos = DiscreteCoord::new(10, 10);
/// let shifted = t.apply_to(pos);
/// assert_eq!(shifted, DiscreteCoord::new(7, 15));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Translation {
    pub x: i32,
    pub y: i32,
}

impl Translation {
    /// Constructs a new translation vector with the given components.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Applies this translation to an unsigned discrete coordinate.
    /// Coordinates that would become negative are clamped to 0.
    ///
    /// # Examples
    /// ```
    /// use overture::interfaces::geometry::{Translation, DiscreteCoord};
    /// 
    /// let t = Translation::new(-4, 2);
    /// let coord = DiscreteCoord::new(3, 5);
    /// let result = t.apply_to(coord);
    /// assert_eq!(result, DiscreteCoord::new(0, 7)); // x can't go below 0
    /// ```
    pub fn apply_to(self, coord: DiscreteCoord) -> DiscreteCoord {
        DiscreteCoord::new(
            max(0, coord.x as i32 + self.x) as u32,
            max(0, coord.y as i32 + self.y) as u32,
        )
    }

    /// Returns the zero translation (no movement).
    pub const fn zero() -> Self {
        Translation { x: 0, y: 0 }
    }
}

impl Add for Translation {
    type Output = Self;

    /// Adds two translations component-wise.
    ///
    /// # Examples
    ///
    /// ```
    /// use overture::interfaces::geometry::Translation;
    /// 
    /// let t1 = Translation::new(3, 4);
    /// let t2 = Translation::new(1, 2);
    /// let result = t1 + t2;
    /// assert_eq!(result, Translation::new(4, 6));
    /// ```
    fn add(self, rhs: Self) -> Self {
        Translation::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Translation {
    type Output = Self;

    /// Subtracts two translations component-wise.
    ///
    /// # Examples
    ///
    /// ```
    /// use overture::interfaces::geometry::Translation;
    /// 
    /// let t1 = Translation::new(5, 7);
    /// let t2 = Translation::new(2, 3);
    /// let result = t1 - t2;
    /// assert_eq!(result, Translation::new(3, 4));
    /// ```
    fn sub(self, rhs: Self) -> Self {
        Translation::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl AddAssign for Translation {
    /// Adds another translation to `self` in-place.
    ///
    /// # Examples
    ///
    /// ```
    /// use overture::interfaces::geometry::Translation;
    /// 
    /// let mut t = Translation::new(1, 1);
    /// t += Translation::new(2, 3);
    /// assert_eq!(t, Translation::new(3, 4));
    /// ```
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Translation {
    /// Subtracts another translation from `self` in-place.
    ///
    /// # Examples
    ///
    /// ```
    /// use overture::interfaces::geometry::Translation;
    /// 
    /// let mut t = Translation::new(5, 5);
    /// t -= Translation::new(2, 3);
    /// assert_eq!(t, Translation::new(3, 2));
    /// ```
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Neg for Translation {
    type Output = Self;

    /// Negates the translation, reversing direction of both components.
    ///
    /// # Examples
    ///
    /// ```
    /// use overture::interfaces::geometry::Translation;
    /// 
    /// let t = Translation::new(3, -4);
    /// let neg = -t;
    /// assert_eq!(neg, Translation::new(-3, 4));
    /// ```
    fn neg(self) -> Self {
        Translation::new(-self.x, -self.y)
    }
}

impl Add<DiscreteCoord> for Translation {
    type Output = DiscreteCoord;

    /// Applies this signed translation to an unsigned coordinate,
    /// clamping negative results to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use overture::interfaces::geometry::{Translation, DiscreteCoord};
    /// 
    /// let t = Translation::new(3, -1);
    /// let c = DiscreteCoord::new(5, 2);
    /// let result = t + c;
    /// assert_eq!(result, DiscreteCoord::new(8, 1));
    /// ```
    fn add(self, rhs: DiscreteCoord) -> DiscreteCoord {
        DiscreteCoord::new(
            (rhs.x as i32 + self.x).max(0) as u32,
            (rhs.y as i32 + self.y).max(0) as u32,
        )
    }
}

impl Add<Translation> for DiscreteCoord {
    type Output = DiscreteCoord;

    /// Applies a signed translation to this unsigned coordinate,
    /// clamping negative results to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use overture::interfaces::geometry::{Translation, DiscreteCoord};
    /// 
    /// let c = DiscreteCoord::new(5, 2);
    /// let t = Translation::new(-3, 4);
    /// let result = c + t;
    /// assert_eq!(result, DiscreteCoord::new(2, 6));
    /// ```
    fn add(self, rhs: Translation) -> Self::Output {
        DiscreteCoord::new(
            (self.x as i32 + rhs.x).max(0) as u32,
            (self.y as i32 + rhs.y).max(0) as u32,
        )
    }
}

impl Sub<Translation> for DiscreteCoord {
    type Output = DiscreteCoord;

    /// Applies a negative translation (reverse movement),
    /// clamping negative results to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use overture::interfaces::geometry::{Translation, DiscreteCoord};
    /// 
    /// let c = DiscreteCoord::new(5, 6);
    /// let t = Translation::new(2, 3);
    /// let result = c - t;
    /// assert_eq!(result, DiscreteCoord::new(3, 3));
    /// ```
    fn sub(self, rhs: Translation) -> Self::Output {
        self + (-rhs)
    }
}


/// Specifies how a renderable element should be positioned within the rendering space.
///
/// `RenderPlacementConfig` provides a set of high-level layout options (like centering or corner placement),
/// as well as a low-level `Offset` variant for precise manual control using a `DiscreteCoord`.
///
/// It is primarily used in the rendering engine to determine the anchor point when placing
/// a renderable group or element.
///
/// # Examples
///
/// ```rust
/// use overture::interfaces::geometry::{RenderPlacementConfig, Translation};
///
/// let config = RenderPlacementConfig::CenterStage;
///
/// let offset = RenderPlacementConfig::Offset(Translation::new(4, 2));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderPlacementConfig {
    /// Aligns the element to the **top-left** corner of the available space.
    TopLeft,

    /// Aligns the element to the **top-right** corner.
    TopRight,

    /// Aligns the element to the **bottom-left** corner.
    BottomLeft,

    /// Aligns the element to the **bottom-right** corner.
    BottomRight,

    /// Aligns the element vertically centered, anchored to the **left edge**.
    CenterLeft,

    /// Aligns the element horizontally centered, anchored to the **top edge**.
    CenterTop,

    /// Aligns the element vertically centered, anchored to the **right edge**.
    CenterRight,

    /// Aligns the element horizontally centered, anchored to the **bottom edge**.
    CenterBottom,

    /// Fully centers the element in both horizontal and vertical directions.
    CenterStage,

    /// Offsets the element by an explicit displacement in grid units.
    ///
    /// This variant uses a `Translation` to move the element from its default position
    /// by the specified amount. Useful for fine-grained positioning when other placement
    /// modes aren’t flexible enough.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use overture::interfaces::geometry::{RenderPlacementConfig, Translation};
    ///
    /// let placement = RenderPlacementConfig::Offset(Translation::new(3, 1));
    /// ```
    Offset(Translation),
}
