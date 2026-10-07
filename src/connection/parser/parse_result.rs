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
    #[serde(rename = "part", default)]
    pub parts: Vec<u64>,
}

/// The winning team and the reason.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Winner {
    #[serde(rename = "@team", default)]
    pub team: Option<Team>,

    #[serde(rename = "@regular")]
    #[serde(default)]
    pub regular: bool,

    #[serde(rename = "@reason")]
    #[serde(default)]
    pub reason: Option<String>,
}

pub fn parse_result(xml: &str) -> Box<Message> {
    Box::from(Message::Result(from_str(xml).expect("Failed to parse game result")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_result_valid_xml() {
        let xml = r#"
            <data class="result">
                <definition>
                    <fragment name="Siegpunkte">
                    <aggregation>SUM</aggregation>
                    <relevantForRanking>true</relevantForRanking>
                    </fragment>
                    <fragment name="Punkte">
                    <aggregation>AVERAGE</aggregation>
                    <relevantForRanking>true</relevantForRanking>
                    </fragment>
                </definition>
                <scores>
                    <entry>
                    <player name="Spieler 1" team="ONE"/>
                    <score>
                        <part>2</part>
                        <part>164</part>
                    </score>
                    </entry>
                    <entry>
                    <player name="Spieler 2" team="TWO"/>
                    <score>
                        <part>0</part>
                        <part>82</part>
                    </score>
                    </entry>
                </scores>
                <winner team="ONE" regular="true" reason="Spieler 1 hat am meisten Punkte erzielt."/>
            </data>
            "#;

        let result = parse_result(xml);

        assert_eq!(
            result.as_ref(),
            &Message::Result(Some(GameResult {
                definition: Definition {
                    fragments: vec![
                        Fragment {
                            name: "Siegpunkte".to_owned(),
                            aggregation: "SUM".to_owned(),
                            relevant_for_ranking: true,
                        },
                        Fragment {
                            name: "Punkte".to_owned(),
                            aggregation: "AVERAGE".to_owned(),
                            relevant_for_ranking: true,
                        },
                    ],
                },
                scores: Scores {
                    entries: vec![
                        Entry {
                            player: Player {
                                name: "Spieler 1".to_owned(),
                                team: Team::One,
                            },
                            score: Score {
                                parts: vec![2, 164]
                            },
                        },
                        Entry {
                            player: Player {
                                name: "Spieler 2".to_owned(),
                                team: Team::Two,
                            },
                            score: Score {
                                parts: vec![0, 82]
                            },
                        },
                    ],
                },
                winner: Winner {
                    team: Some(Team::One),
                    regular: true,
                    reason: Some("Spieler 1 hat am meisten Punkte erzielt.".to_owned()),
                },
            }))
        );
    }
}