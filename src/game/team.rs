use std::{fmt::Error, str::FromStr};

use serde::Deserialize;

use crate::game::color::Color::{self, Blue, Yellow, Red, Green};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Team {
    #[serde(rename = "ONE")]
    One,
    #[serde(rename = "TWO")]
    Two,
}

impl Team {
    pub fn get_team_colors(&self) -> [Color; 2] {
        match self {
            Team::One => [Blue, Red],
            Team::Two => [Yellow, Green],
        }
    }
}

impl FromStr for Team {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "one" => Ok(Team::One),
            "two" => Ok(Team::Two),
            _ => Err(Error)
        }
    }
}