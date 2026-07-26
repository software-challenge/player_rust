use crate::game::r#move::Rotation;

pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

impl Coordinate {
    pub fn new(x: isize, y: isize) -> Self {
        Coordinate { x, y }
    }

    pub fn add(&self, other: &Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn subtract(&self, other: &Coordinate) -> Coordinate {
        Coordinate {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn multiply(&self, scalar: isize) -> Coordinate {
        Coordinate {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn divide(&self, scalar: isize) -> Coordinate {
        Coordinate {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }

    /// Transforms the coordinates relative to the coordinate origin by applying the specified rotation.
    /// The rotation is applied in a clockwise direction.
    /// The old coordinates are overwritten!
    pub fn rotate(&mut self, rotation: &Rotation) {
        match rotation {
            Rotation::Right => {
                // 90 degrees clockwise rotation (x, y) -> (y, -x)
                let old_x = self.x;
                self.x = self.y;
                self.y = -old_x;
            },
            Rotation::Mirror => {
                // 180 degrees clokweise rotation (x, y) -> (-x, -y)
                self.x = -self.x;
                self.y = -self.y;
            },
            Rotation::Left => {
                // 270 degrees clockwise rotation (x, y) -> (-y, x)
                let old_x = self.x;
                self.x = -self.y;
                self.y = old_x;
            },
            Rotation::None => {}
        }
    }

    /// Flips the coordinates on the vetical axis (y-axis) relative to the coordinate origin.
    /// The old coordinates are overwritten!
    pub fn flip_on_vertical(&mut self) {
        self.x = -self.x;
    }
}