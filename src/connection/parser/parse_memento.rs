use std::str::FromStr;

use xml::EventReader;
use xml::reader::XmlEvent;

use crate::{connection::parser::message::Message, game::{board::{Board, Team}, gamestate::GameState, r#move::{Move, Rotation}, piece::PieceType}};

pub fn parse_memento(mut parser: EventReader<&[u8]>) -> Box<Message> {
    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                // Search for the state element
                if name.local_name == "state" {
                    // Extract the turn and then decide what to do based on the turn value
                    for attr in attributes.clone() {
                        if attr.name.local_name == "turn" {
                            let turn_value = attr.value.parse::<u32>().unwrap_or(0);
                            
                            if turn_value == 0 {
                                // Extract all information for the initial state
                                let mut starting_piece = PieceType::Mono; // Default value, will be overwritten if found 
                                let mut starting_team = Team::Blue; // Default value, will be overwritten if found 
                                for attr in attributes {
                                    if attr.name.local_name == "startPiece" {
                                        starting_piece = PieceType::from_str(&attr.value).unwrap();
                                    }

                                    if attr.name.local_name == "startTeam" {
                                        starting_team = match attr.value.as_str() {
                                            "ONE" => Team::Blue,
                                            "TWO" => Team::Yellow,
                                            _ => {
                                                //Fallback in case no valid startTeam exists.
                                                eprintln!("No valid team found in startTeam; falling back to Team Blue!");
                                                Team::Blue
                                            }
                                        }
                                    }
                                }

                                let mut blue_pieces: Vec<PieceType> = Vec::new();
                                let mut yellow_pieces: Vec<PieceType> = Vec::new();
                                let mut red_pieces: Vec<PieceType> = Vec::new();
                                let mut green_pieces: Vec<PieceType> = Vec::new();

                                let mut current_teams_pieces: Option<&mut Vec<PieceType>> = None;

                                loop {
                                    match parser.next() {
                                        Ok(XmlEvent::StartElement { name, .. }) => {
                                            if name.local_name == "blueShapes" {
                                                current_teams_pieces = Some(&mut blue_pieces);
                                            } else if name.local_name == "yellowShapes" {
                                                current_teams_pieces = Some(&mut yellow_pieces);
                                            } else if name.local_name == "redShapes" {
                                                current_teams_pieces = Some(&mut red_pieces    );
                                            } else if name.local_name == "greenShapes" {
                                                current_teams_pieces = Some(&mut green_pieces);
                                            } else if name.local_name == "validColors" {
                                                current_teams_pieces = None; // No PieceType to add for validColors
                                            }
                                        }
                                        Ok(XmlEvent::Characters(text)) if current_teams_pieces.is_some() => {
                                            current_teams_pieces.as_mut().unwrap().push(PieceType::from_str(&text).unwrap());
                                        }
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name.local_name == "state" {
                                                // TODO: Replace current turn team with real starting team

                                                let game_state = GameState::new(starting_piece, starting_team == Team::Blue, Board::new(), 0, 1, starting_team, blue_pieces, yellow_pieces, red_pieces, green_pieces);

                                                return Box::new(Message::MementoInitial(Some(game_state)));
                                            }
                                        }
                                        Ok(_) => {}
                                        Err(e) => {
                                            eprintln!("Error while parsing memento: {e}");
                                            continue;
                                        }
                                    }
                                }

                            } else {
                                // Extract last move
                                let mut team: Option<Team> = None;
                                let mut piece: Option<PieceType> = None;
                                let mut x: Option<usize> = None;
                                let mut y: Option<usize> = None;
                                let mut is_flipped: Option<bool> = None;
                                let mut rotation: Option<Rotation> = None;
                                let mut skip: bool = false;

                                let mut next_text_color = false;

                                loop {
                                    match parser.next() {
                                        Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                                            if name.local_name == "piece" {
                                                for attr in attributes {
                                                    match attr.name.local_name.as_str() {
                                                        "color" => team = Some(Team::from_string(&attr.value)),
                                                        "kind" => piece = Some(PieceType::from_str(&attr.value).unwrap()),
                                                        "isFlipped" => is_flipped = Some(attr.value.parse::<bool>().unwrap()),
                                                        "rotation" => rotation = Some(Rotation::from_string(&attr.value).unwrap()),
                                                        _ => {}
                                                    }
                                                }
                                            } else if name.local_name == "position" {
                                                for attr in attributes {
                                                    match attr.name.local_name.as_str() {
                                                        "x" => x = Some(attr.value.parse::<usize>().unwrap()),
                                                        "y" => y = Some(attr.value.parse::<usize>().unwrap()),
                                                        _ => {}
                                                    }
                                                }
                                            } else if name.local_name == "lastMove" {
                                                for attr in attributes {
                                                    if attr.name.local_name == "class" && attr.value == "sc.plugin2027.SkipMove" {
                                                        skip = true;
                                                    }
                                                }
                                            } else if name.local_name == "color" {
                                                next_text_color = true;
                                            }
                                        }
                                        Ok(XmlEvent::Characters(text)) => {
                                            if next_text_color {
                                                team = Some(Team::from_string(&text));
                                                next_text_color = false;
                                            }
                                        }
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name.local_name == "lastMove" {

                                                if skip {
                                                    return Box::new(Message::MementoLastMove(Some(turn_value as u8), Some(Move::new(team.unwrap(), PieceType::Mono, 0, 0, false, Rotation::None, true))));
                                                }

                                                return Box::new(Message::MementoLastMove(Some(turn_value as u8), Some(Move::new(team.unwrap(), piece.unwrap(), x.unwrap(), y.unwrap(), is_flipped.unwrap(), rotation.unwrap(), false))));
                                            }
                                        }
                                        Ok(XmlEvent::EndDocument) => {
                                            return Box::new(Message::MementoLastMove(None, None));
                                        }
                                        Ok(_) => {}
                                        Err(e) => {
                                            eprintln!("Error while parsing memento: {e}");
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Ok(XmlEvent::EndDocument) => {
                return Box::new(Message::MementoInitial(None));
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error while parsing memento: {e}");
                continue;
            }
        }
    }

}
