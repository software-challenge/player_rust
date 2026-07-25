use xml::{EventReader, reader::XmlEvent};

use crate::connection::parser::message::Message;

pub fn parse_message(parser: EventReader<&[u8]>) -> Result<Box<Message>, Box<dyn std::error::Error>> {
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                if name.local_name == "data" {
                    for attr in attributes {
                        if attr.name.local_name == "class" {
                            match attr.value.as_str() {
                                "memento" => {
                                    return Ok(Box::new(Message {
                                        message_type: crate::connection::parser::message::MessageType::Memento,
                                        game_state: None, // TODO parse game state from XML
                                        result: None,
                                    }));
                                },
                                "moveRequest" => {
                                    return Ok(Box::new(Message {
                                        message_type: crate::connection::parser::message::MessageType::MoveRequest,
                                        game_state: None,
                                        result: None,
                                    }));
                                },
                                "result" => {
                                    return Ok(Box::new(Message {
                                        message_type: crate::connection::parser::message::MessageType::Result,
                                        game_state: None,
                                        result: None, // TODO parse result from XML
                                    }));
                                },
                                _ => {
                                    return Err(format!("Unknown class attribute value: {}", attr.value).into());
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
                continue;
            }
            _ => {}
        }
    }
    Err("Failed to parse message".into())
}