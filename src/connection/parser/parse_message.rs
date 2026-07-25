use xml::{EventReader, reader::XmlEvent};

pub fn parse_message(parser: EventReader<&[u8]>) -> Result<Box<str>, Box<dyn std::error::Error>> {
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                if name.local_name == "data" {
                    for attr in attributes {
                        if attr.name.local_name == "class" {
                            match attr.value.as_str() {
                                "memento" => {
                                    return Ok("memento".into());
                                },
                                "moveRequest" => {
                                    return Ok("moveRequest".into());
                                },
                                "result" => {
                                    return Ok("result".into());
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