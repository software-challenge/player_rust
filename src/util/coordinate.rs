use crate::game::r#move::Rotation;

pub struct Coordinates {

}

impl Coordinates {
    /// Normalizes the coordinates by translating them so that the minimum x and y values become (0, 0).
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

    /// Rotates the coordinates by the specified rotation (clockwise) relative to the coordinate origin.
    /// Does not normalize the coordinates after rotation, so the minimum x and y values may not be (0, 0).
    pub fn rotate_coordinates(coordinates: Vec<Coordinate>, rotation: &Rotation) -> Vec<Coordinate> {
        let mut rotated_coordinates: Vec<Coordinate> = Vec::new();

        for mut coord in coordinates {
            coord = coord.rotate(rotation);
            rotated_coordinates.push(coord);
        }

        rotated_coordinates
    }

    pub fn flip_coordinates(coordinates: Vec<Coordinate>) -> Vec<Coordinate> {
        let mut flipped_coordinates: Vec<Coordinate> = Vec::new();

        for mut coord in coordinates {
            coord = coord.flip_on_vertical();
            flipped_coordinates.push(coord);
        }

        flipped_coordinates
    }
}

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
    pub fn rotate(&mut self, rotation: &Rotation) -> Coordinate{
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
    pub fn flip_on_vertical(&mut self) -> Coordinate {
        Coordinate { x: -self.x, y: self.y }
    }
}
