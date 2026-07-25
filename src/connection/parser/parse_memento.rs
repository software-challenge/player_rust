use xml::EventReader;
use xml::reader::XmlEvent;

use crate::{connection::parser::message::Message, game::{board::Board, gamestate::GameState, pieces::Pieces}};

pub fn parse_memento(mut parser: EventReader<&[u8]>) -> Box<Message> {
    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                // Search for the state element
                if name.local_name == "state" {
                    // Extract the turn and then decide what to do based on the turn value
                    for attr in attributes {
                        if attr.name.local_name == "turn" {
                            let turn_value = attr.value.parse::<u32>().unwrap_or(0);
                            
                            if turn_value == 0 {
                                // Extract all information for the initial state
                                
                                print!("Extracting initial game state from memento message...");

                                let mut game_state = GameState {
                                    board: Board::new(),
                                    turn: 0,
                                    blue_pieces: Vec::new(),
                                    yellow_pieces: Vec::new(),
                                    red_pieces: Vec::new(),
                                    green_pieces: Vec::new(),
                                };

                                let mut current_pieces: Option<&mut Vec<Pieces>> = None;

                                loop {
                                    match parser.next() {
                                        Ok(XmlEvent::StartElement { name, .. }) => {
                                            if name.local_name == "blueShapes" {
                                                current_pieces = Some(&mut game_state.blue_pieces);
                                            } else if name.local_name == "yellowShapes" {
                                                current_pieces = Some(&mut game_state.yellow_pieces);
                                            } else if name.local_name == "redShapes" {
                                                current_pieces = Some(&mut game_state.red_pieces);
                                            } else if name.local_name == "greenShapes" {
                                                current_pieces = Some(&mut game_state.green_pieces);
                                            } else if name.local_name == "validColors" {
                                                current_pieces = None; // No pieces to add for validColors
                                            }
                                        }
                                        Ok(XmlEvent::Characters(text)) if current_pieces.is_some() => {
                                            current_pieces.as_mut().unwrap().push(Pieces::from_string(&text));
                                        }
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name.local_name == "state" {
                                                return Box::new(Message {
                                                    message_type: crate::connection::parser::message::MessageType::MementoInitial,
                                                    game_state: Some(game_state),
                                                    last_move: None,
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
