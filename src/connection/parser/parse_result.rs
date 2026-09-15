use crate::{
    connection::parser::message::Message,
    game::team::Team
};

use serde::Deserialize;
use quick_xml::de::from_str;

/// The game result contained in a `<data class="result">` element.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct GameResult {
    pub definition: Definition,

    pub scores: Scores,

    pub winner: Winner,
}

/// The scoring fragments (e.g. "Siegpunkte", "Punkte").
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
pub struct Definition {
    #[serde(rename = "fragment", default)]
    pub fragments: Vec<Fragment>,
}

/// A single scoring fragment.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
pub struct Fragment {
    #[serde(rename = "@name", default)]
    pub name: String,

    #[serde(rename = "aggregation", default)]
    pub aggregation: String,

    #[serde(rename = "relevantForRanking", default)]
    pub relevant_for_ranking: bool,
}

/// All player score entries.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
pub struct Scores {
    #[serde(rename = "entry", default)]
    pub entries: Vec<Entry>,
}

/// A single player entry inside `<scores>`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Entry {
    #[serde(rename = "player")]
    pub player: Player,

    #[serde(rename = "score")]
    pub score: Score,
}

/// A player identified by name and team (both are attributes).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Player {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@team")]
    pub team: Team,
}

/// A list of scored parts (e.g. `<part>2</part><part>102</part>`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
pub struct Score {
    #[serde(rename = "$text", default)]
    pub parts: Vec<u64>,
}

/// The winning team and the reason.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Winner {
    #[serde(rename = "@team")]
    pub team: Team,

    #[serde(rename = "@regular")]
    #[serde(default)]
    pub regular: bool,

    #[serde(rename = "@reason")]
    #[serde(default)]
    pub reason: Option<String>,
}

pub fn parse_result(xml: &str) -> Box<Message> {
    Box::from(Message::Result(from_str(xml).unwrap()))
}

