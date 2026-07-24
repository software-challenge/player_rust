use xml::{EventReader, reader::XmlEvent};

pub fn parse_joined(parser: EventReader<&[u8]>) -> Result<Box<str>, Box<dyn std::error::Error>> {
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                if name.local_name == "joined" {
                    for attr in attributes {
                        if attr.name.local_name == "roomId" {
                            return Ok(Box::from(attr.value));
                        }
                    }
                }
            },
            Err(e) => return Err("Error parsing XML".into()), //Err(ConnectionHandlerError::Xml(e)),
            _ => {}
        }
    }
    return Err("No <joined> element found in XML".into()); //Err(ConnectionHandlerError::NoJoinedElement);
}