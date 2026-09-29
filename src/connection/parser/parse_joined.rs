use quick_xml::Reader;

pub fn parse_joined(xml: &[u8]) -> Result<Box<str>, Box<dyn std::error::Error>> {
    let xml_str = std::str::from_utf8(xml)?;
    let mut reader = Reader::from_str(xml_str);

    loop {
        match reader.read_event()? {
            quick_xml::events::Event::Empty(j) if j.name().as_ref() == "joined" =>  {
                for atrr in j.attributes() {
                    let attr = atrr?;
                    if attr.key.as_ref() == "roomId" {
                        return Ok(Box::from(attr.value))
                    }
                }
            },
            quick_xml::events::Event::Eof => {break;}, 
            _ => {}
        }
    }
    Err("No <joined> element found in XML".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_joined_valid_xml() {
        let xml = b"<joined roomId=\"room123\"/>";
        let result = parse_joined(xml);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_ref(), "room123");
    }
}