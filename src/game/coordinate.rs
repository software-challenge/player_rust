#[cfg(test)]
#[path = "tests/coordinate.rs"]
mod tests;

use crate::game::rotation::Rotation;

/// A 2D coordinate used for tile placement and piece transformations.
///
/// Coordinates are relative to the board origin and are commonly normalized to
/// start at `(0, 0)` before a piece is placed.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

impl Coordinate {
    /// Creates a new coordinate.
    pub const fn new(x: isize, y: isize) -> Self {
        Coordinate { x, y }
    }

    /// Adds another coordinate to this one in place.
    pub fn add(&mut self, other: &Coordinate) {
        self.x += other.x;
        self.y += other.y;
    }

    /// Subtracts another coordinate from this one in place.
    pub fn subtract(&mut self, other: &Coordinate) {
        self.x -= other.x;
        self.y -= other.y;
    }

    /// Multiplies the coordinate by a scalar in place.
    pub fn multiply(&mut self, scalar: isize){
        self.x *= scalar;
        self.y *= scalar;
    }

    /// Divides the coordinate by a scalar in place.
    pub fn divide(&mut self, scalar: isize) {
        self.x /= scalar;
        self.y /= scalar;
    }

    /// Rotates this coordinate around the origin by the given [`Rotation`].
    ///
    /// The rotation is applied in a clockwise direction to match the engine's
    /// board coordinate system.
    pub fn rotate(&self, rotation: &Rotation) -> Coordinate {
        match rotation {
            Rotation::Right => {
                // Board coordinates increase downward, so clockwise is (x, y) -> (-y, x).
                Coordinate { x: -self.y, y: self.x }
            },
            Rotation::Mirror => {
                // 180 degrees clokweise rotation (x, y) -> (-x, -y)
                Coordinate { x: -self.x, y: -self.y }
            },
            Rotation::Left => {
                // 270 degrees clockwise rotation (x, y) -> (y, -x)
                Coordinate { x: self.y, y: -self.x }
            },
            Rotation::None => {
                // No rotation, return the original coordinates
                Coordinate { x: self.x, y: self.y }
            }
        }
    }

    /// Flips the coordinates on the vetical axis (y-axis) relative to the coordinate origin.
    pub fn flip_on_vertical(&self) -> Coordinate {
        Coordinate { x: -self.x, y: self.y }
    }
}

/// Normalizes the coordinates in place so that their minimum x and y values become (0, 0).
///
/// Overwrites the original coordinates with the normalized values.
pub fn normalize_coordinates(coordinates: &mut [Coordinate]) {
    let Some(first) = coordinates.first() else {
        return;
    };

    let (mut min_x, mut min_y) = (first.x, first.y);
    for coord in coordinates.iter().skip(1) {
        min_x = min_x.min(coord.x);
        min_y = min_y.min(coord.y);
    }

    for coord in coordinates {
        coord.x -= min_x;
        coord.y -= min_y;
    }
}

/// Rotates every coordinate by the specified clockwise rotation.
///
/// The rotated coordinates are not normalized, so they may still have negative
/// values after the transformation.
/// Overwrites the original coordinates with the normalized values.
pub fn rotate_coordinates(coordinates: &mut [Coordinate], rotation: &Rotation) {
    for coord in coordinates {
        *coord = coord.rotate(rotation);
    }
}

/// Mirrors coordinates across the vertical axis.
///
/// This is the helper used when a piece is flipped before it is placed on the
/// board.
/// Overwrites the original coordinates with the normalized values.
pub fn flip_coordinates(coordinates: &mut [Coordinate]) {
    for coord in coordinates {
        *coord = coord.flip_on_vertical();
    }
}