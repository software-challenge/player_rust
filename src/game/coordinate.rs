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

/// Normalizes a set of coordinates so that the top-left-most cell becomes the origin.
///
/// This is useful after rotating or mirroring a piece so its local coordinate
/// system starts at `(0, 0)` with non-negative values only.
pub fn normalize_coordinates(coordinates: &Vec<Coordinate>) -> Vec<Coordinate> {
    let mut min_x = isize::MAX;
    let mut min_y = isize::MAX;

    for coord in coordinates {
        if coord.x < min_x {
            min_x = coord.x;
        }
        
        if coord.y < min_y {
            min_y = coord.y;
        }
    }

    let mut normalized_coordinates: Vec<Coordinate> = Vec::new();
    for coord in coordinates {
        normalized_coordinates.push(Coordinate { x: coord.x - min_x, y: coord.y - min_y });
    }

    normalized_coordinates
}

/// Rotates every coordinate by the specified clockwise rotation.
///
/// The returned coordinates are not normalized, so they may still have negative
/// values after the transformation.
pub fn rotate_coordinates(coordinates: Vec<Coordinate>, rotation: &Rotation) -> Vec<Coordinate> {
    let mut rotated_coordinates: Vec<Coordinate> = Vec::new();

    for coord in coordinates {
        rotated_coordinates.push(coord.rotate(rotation));
    }

    rotated_coordinates
}

/// Mirrors coordinates across the vertical axis.
///
/// This is the helper used when a piece is flipped before it is placed on the
/// board.
pub fn flip_coordinates(coordinates: Vec<Coordinate>) -> Vec<Coordinate> {
    let mut flipped_coordinates: Vec<Coordinate> = Vec::new();

    for coord in coordinates {
        flipped_coordinates.push(coord.flip_on_vertical());
    }

    flipped_coordinates
}