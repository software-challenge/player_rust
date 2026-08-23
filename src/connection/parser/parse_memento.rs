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
                                
                                println!("Extracting initial game state from memento message...");

                                let mut starting_piece = PieceType::Mono; // Default value, will be overwritten if found 

                                for attr in attributes {
                                    if attr.name.local_name == "startPiece" {
                                        starting_piece = PieceType::from_str(&attr.value).unwrap();
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
                                                let game_state = GameState::new(starting_piece, Board::new(), 0, 1, Team::Blue, blue_pieces, yellow_pieces, red_pieces, green_pieces);

                                                return Box::new(Message {
                                                    message_type: crate::connection::parser::message::MessageType::MementoInitial,
                                                    game_state: Some(game_state),
                                                    last_move: None,
                                                    turn: None,
                                                    result: None,
                                                });
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

                                println!("Extracting last move from memento message...");

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
                                                    if attr.name.local_name == "class" {
                                                        if attr.value == "sc.plugin2027.SkipMove" {
                                                            skip = true;
                                                        }
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
                                                    return Box::new(Message {
                                                        message_type: crate::connection::parser::message::MessageType::MementoLastMove,
                                                        game_state: None,
                                                        last_move: Some(Box::new(Move { team: team.unwrap(), piece: PieceType::Mono, x: 0, y: 0, is_flipped: false, rotation: Rotation::None, skip: true })),
                                                        turn: Some(turn_value as u8),
                                                        result: None,
                                                    });
                                                }

                                                return Box::new(Message {
                                                    message_type: crate::connection::parser::message::MessageType::MementoLastMove,
                                                    game_state: None,
                                                    last_move: Some(Box::new(Move { team: team.unwrap(), piece: piece.unwrap(), x: x.unwrap(), y: y.unwrap(), is_flipped: is_flipped.unwrap(), rotation: rotation.unwrap(), skip: false })),
                                                    turn: Some(turn_value as u8),
                                                    result: None,
                                                });
                                            }
                                        }
                                        Ok(XmlEvent::EndDocument) => {
                                            return Box::new(Message {
                                                message_type: crate::connection::parser::message::MessageType::MementoLastMove,
                                                game_state: None,
                                                last_move: None,
                                                turn: None,
                                                result: None,
                                            });
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
                return Box::new(Message {
                    message_type: crate::connection::parser::message::MessageType::MementoInitial,
                    game_state: None,
                    last_move: None,
                    turn: None,
                    result: None,
                });
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error while parsing memento: {e}");
                continue;
            }
        }
    }

}
