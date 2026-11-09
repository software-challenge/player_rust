use std::{fmt::Error, str::FromStr};

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Team {
    #[serde(rename = "ONE")]
    One,
    #[serde(rename = "TWO")]
    Two,
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