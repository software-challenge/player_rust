use std::{fmt::Error, str::FromStr};

use xml::{EventReader, reader::XmlEvent};

use crate::connection::parser::message::Message;

#[derive(Debug)]
pub struct GameResult {
    winner_team: Option<Team>,
    regular: bool,
    reason: Box<str>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Team {
    One,
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

pub fn parse_result(mut parser: EventReader<&[u8]>) -> Box<Message> {
    let mut result = GameResult {
        winner_team: None,
        regular: false,
        reason: Box::from("Result not parsed!"),
    };

    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name.local_name.as_str() {
                    "winner" => {
                        for atr in attributes {
                            match atr.name.local_name.as_str() {
                                "team" => {if let Ok(team) = Team::from_str(&atr.value) {result.winner_team = Some(team)}},
                                "regular" => {if let Ok(reg) = bool::from_str(&atr.value) {result.regular = reg}},
                                "reason" => {result.reason = String::into_boxed_str(atr.value)},
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            },
            Ok(XmlEvent::EndDocument) => {break;},
            Ok(_) => {},
            Err(e) => {
                eprintln!("{}", e);
                break;
            },
        }
    }

    println!("{:?}", result);
    Box::new(Message::Result(Some(result)))
}

