#[cfg(test)]
#[path = "tests/coordinate.rs"]
mod tests;

use crate::game::rotation::Rotation;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

impl Coordinate {
    pub const fn new(x: isize, y: isize) -> Self {
        Coordinate { x, y }
    }

    pub fn add(&mut self, other: &Coordinate) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn subtract(&mut self, other: &Coordinate) {
        self.x -= other.x;
        self.y -= other.y;
    }

    pub fn multiply(&mut self, scalar: isize){
        self.x *= scalar;
        self.y *= scalar;
    }

    pub fn divide(&mut self, scalar: isize) {
        self.x /= scalar;
        self.y /= scalar;
    }

    /// Transforms the coordinates relative to the coordinate origin by applying the specified rotation.
    /// The rotation is applied in a clockwise direction.
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
pub fn normalize_coordinates(coordinates: &mut [Coordinate]) {
    let Some(first) = coordinates.first() else {
        return;
    };

    let (mut min_x, mut min_y) = (first.x, first.y);
    #[allow(clippy::indexing_slicing)]
    for coord in &coordinates[1..] {
        min_x = min_x.min(coord.x);
        min_y = min_y.min(coord.y);
    }

    for coord in coordinates {
        coord.x -= min_x;
        coord.y -= min_y;
    }
}

/// Rotates the coordinates in place clockwise relative to the coordinate origin.
/// Does not normalize the coordinates after rotation, so the minimum x and y values may not be (0, 0).
pub fn rotate_coordinates(coordinates: &mut [Coordinate], rotation: &Rotation) {
    for coord in coordinates {
        *coord = coord.rotate(rotation);
    }
}

/// Flips the coordinates in place on the vertical axis (y-axis) relative to the coordinate origin.
pub fn flip_coordinates(coordinates: &mut [Coordinate]) {
    for coord in coordinates {
        *coord = coord.flip_on_vertical();
    }
}