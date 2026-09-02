use xml::{EventReader, reader::XmlEvent};

use crate::connection::parser::{message::Message, parse_memento::parse_memento, parse_result::parse_result};

pub fn parse_message(mut parser: EventReader<&[u8]>) -> Result<Box<Message>, Box<dyn std::error::Error>> {
    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                // Search for the data element
                if name.local_name == "data" {
                    for attr in attributes {
                        if attr.name.local_name == "class" {
                            match attr.value.as_str() {
                                "memento" => {
                                    println!("Parsing memento message...");
                                    return Ok(parse_memento(parser))
                                },
                                "moveRequest" => {
                                    return Ok(Box::new(Message {
                                        message_type: crate::connection::parser::message::MessageType::MoveRequest,
                                        game_state: None,
                                        last_move: None,
                                        turn: None,
                                        result: None,
                                    }));
                                },
                                "result" => {
                                    println!("Parsing memento result...");
                                    return Ok(parse_result(parser))
                                },
                                _ => {
                                    return Err(format!("Unknown class attribute value: {}", attr.value).into());
                                }
                            }
                        }
                    }
                }
            }
            Ok(XmlEvent::EndDocument) => {
                //If reached then the document ended without finding a data element, which is unexpected
                return Err("Error while parsing message: Unexpected end of document".into());
            }
            Err(e) => {
                return Err(format!("Error while parsing message: {e}").into());
            }
            Ok(_) => {}
        }
    }
}