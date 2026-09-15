use std::{
    fmt::Write as _, io::{self, Read, Write}, net::TcpStream
};

#[cfg(feature = "debug-recv-comm-log")]
use std::fs::OpenOptions;

use quick_xml::Reader;
use xml::{EventReader, reader::XmlEvent};

use crate::connection::{parser::{message::Message, parse_joined::parse_joined, parse_result::parse_result}, parser_strategy::ParserStrategy};
use crate::game::r#move::Move;


///Indicates that the ConnectionHandler is connected.
#[derive(Debug)]
pub struct Connected;
///Indicates that the ConnectionHandler has joined a game, also holds the room id.
#[derive(Debug)]
pub struct Joined {
    room_id: Box<str>
}

#[derive(Debug)]
pub struct ConnectionHandler<State, S: ParserStrategy> {
    connection: TcpStream,
    #[cfg(feature = "debug-recv-comm-log")]
    log_file: std::fs::File,
    strategy: S,
    state: State,
}   

impl<S: ParserStrategy> ConnectionHandler<(), S> {
    /// Attempts to create a new ConnectionHandler instance by connecting to the specified host and port.
    /// If no host or port is provided, defaults to "127.0.0.1" and "13050".
    pub fn try_new(host: Option<&str>, port: Option<&str>, strategy: S) -> Result<ConnectionHandler<Connected, S>, Box<dyn std::error::Error>> {
        // Construct address using provided host and port, or default values if not provided
        let host = host.unwrap_or("127.0.0.1");
        let port = port.unwrap_or("13050");

        let address = format!("{}:{}", host, port);

        Ok(ConnectionHandler{
            connection: TcpStream::connect(address)?,
            #[cfg(feature = "debug-recv-comm-log")]
            log_file: OpenOptions::new()
                .create(true)
                .append(true)
                .open("connection_log.xml")?,
                state: Connected,
                strategy
        })
    }

    /// Creates a new `ConnectionHandler` instance by retrieving competition system parameters from command line arguments.
    /// Automatically connects to the competition system using the provided host, port and reservation code.
    pub fn new_from_commandline_args(strategy: S) -> Result<ConnectionHandler<Joined, S>, Box<dyn std::error::Error>> {
        let cmd_args = crate::util::cmdl_args::get_competition_system_parameters();
        
        let connection_handler = Self::try_new(cmd_args.get_host().as_deref(), cmd_args.get_port().as_deref(), strategy)?;

        connection_handler.join(cmd_args.get_reservation().as_deref())
    }
}

impl<S: ParserStrategy> ConnectionHandler<Connected, S> {
    /// Joins a game, optionally using a reservation code if provided.
    pub fn join(mut self, reservation_code: Option<&str>) -> Result<ConnectionHandler<Joined, S>, Box<dyn std::error::Error>> {
        match reservation_code {
            Some(rc) => self.connection.write(format!("<protocol><joinPrepared reservationCode=\"{}\"/>", rc).as_bytes())?,
            None => self.connection.write(b"<protocol><join/>")?
        };
        
        // Receive the welcome message from the server and read it into the buffer
        let buffer = read_message_to_buffer(&mut self.connection)?;

        if buffer.is_empty() {
            return Err("No bytes received by the server".into()); //Err(ConnectionHandlerError::ZeroBytesReadToBuffer);
        }

        if !buffer.starts_with(b"<protocol>"){return Err("Invalid XML format".into());}

        // Parse the welcome message to extract the roomId
        let raw_xml: &[u8] = xml_payload_from_buffer(&buffer);
        let parser: EventReader<&[u8]> = EventReader::new(raw_xml);
    
        return Ok(ConnectionHandler { 
            connection: self.connection,
            #[cfg(feature = "debug-recv-comm-log")]
            log_file: self.log_file,
            state: Joined { room_id: parse_joined(parser)? },
            strategy: self.strategy
        });
    }


}

impl<S: ParserStrategy> ConnectionHandler<Joined, S> {
    pub fn get_room_id(&self) -> &Box<str> {
        &self.state.room_id
    }

    /// Sends a move to the server in XML format.
    pub fn send_move(&mut self, m: &Move) -> Result<(), Box<dyn std::error::Error>> {

        let mut move_xml = String::new();
        write!(move_xml, "<room roomId=\"{}\">", self.get_room_id().as_ref())?;

        if m.skip {
            write!(move_xml, "<data class=\"sc.plugin2027.SkipMove\"><color>{}</color></data></room>", m.color.to_string())?;
        } else {
            write!(move_xml, "<data class=\"sc.plugin2027.SetMove\"><piece color=\"{}\" kind=\"{}\" rotation=\"{}\" isFlipped=\"{}\"><position x=\"{}\" y=\"{}\"/></piece></data></room>",
            m.color.to_string(),
            m.piece,
            m.rotation.to_string(),
            m.is_flipped,
            m.x,
            m.y)?;
        }

        self.connection.write_all(move_xml.as_bytes())?;
        self.connection.flush()?;

        Ok(())
    }

    /// Reads a new message from the server, parses it, and returns the parsed message
    pub fn get_new_message(&mut self) -> Result<Box<Message>, Box<dyn std::error::Error>> {
        let buffer: Vec<u8> = read_message_to_buffer(&mut self.connection)?;

        if buffer.is_empty() {
            return Err("No bytes received by the server".into()); //Err(ConnectionHandlerError::ZeroBytesReadToBuffer);
        }

        let raw_xml: &[u8] = xml_payload_from_buffer(&buffer);
        let parser: EventReader<&[u8]> = EventReader::new(raw_xml);
        let message: Box<Message> = Self::parse_message(parser)?;

        Ok(message)
    }

    pub fn new_parse_message(xml: &[u8])-> Result<Box<Message>, Box<dyn std::error::Error>> {
        let xml_str = std::str::from_utf8(xml)?;

        let data_start = xml_str.find("<data ").ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "could not find <data start tag",
        ))?;
        let data_end = xml_str.rfind("</data>").ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "could not find </data> end tag",
        ))? + 6;

        let mut reader = Reader::from_str(&xml_str[data_start..]);
        loop {
            match reader.read_event()? {
                quick_xml::events::Event::Start(e) if e.name().as_ref() == "data"=> {
                    match &*e.attributes().find(|attribute| {
                        if let Ok(attr) = attribute {
                            attr.key.as_ref() == "class"
                        } else {
                            false
                        }
                    }).ok_or(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "could not find class atrr on data tag"))??.value {
                        "memento" => return Ok(Box::new(Message::MoveRequest)),
                        "moveRequest" => return Ok(Box::new(Message::MoveRequest)),
                        "result" => return Ok(parse_result(&xml_str[data_start..=data_end])),
                        attr_val => {return Err(Box::from(io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    format!("Unknown class attribute value: {}", attr_val))))}

                    }
                },
                quick_xml::events::Event::Eof => {break},
                _ => ()
            }
        }

        Err(Box::from(io::Error::new(
            io::ErrorKind::InvalidData,
            "could not find data class")))
    }

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
                                        return Ok(S::parse_memento(parser))
                                    },
                                    "moveRequest" => {
                                        return Ok(Box::new(Message::MoveRequest));
                                    },
                                    "result" => {
                                        //return Ok(parse_result(parser))
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
}

/// Checks if the buffer ends with the "</room>" closing tag.
fn buffer_ends_with_room_tag(buffer: &[u8]) -> bool {
    buffer.ends_with(b"</room>")
}

fn xml_payload_from_buffer(buffer: &[u8]) -> &[u8] {
    let start = buffer.iter().position(|&b| b == b'<').unwrap_or(0);
    &buffer[start..]
}

fn read_message_to_buffer(tcp_stream: &mut TcpStream) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();

    loop {
        let number_of_new_bytes = read_to_buffer(tcp_stream, &mut buffer)?;

        if number_of_new_bytes == 0 {
            return Err("Zero Bytes Read To Buffer".into()); //Err(ConnectionHandlerError::ZeroBytesReadToBuffer);
        }

        if buffer_ends_with_room_tag(&buffer) {
            return Ok(buffer);
        }
    }
}

fn read_to_buffer(tcp_stream: &mut TcpStream, buffer: &mut Vec<u8>) -> Result<usize, Box<dyn std::error::Error>> {
    let start_len = buffer.len();
    buffer.resize(start_len + 4096, 0);

    match tcp_stream.read(&mut buffer[start_len..]){
        Ok(0) => {
            buffer.truncate(start_len);
            Err("Zero Bytes Read To Buffer".into()) //Err(ConnectionHandlerError::ZeroBytesReadToBuffer);
        },
        Ok(b) => {
            buffer.truncate(start_len + b);

            #[cfg(feature = "debug-recv-comm-log")]
            {
            self.log_file.write_all(&buffer[start_len..start_len + b])?;
            self.log_file.write_all(b"\n")?;
            self.log_file.flush()?;
            }

            Ok(b)
        },
        Err(e) => {
            buffer.truncate(start_len);
            Err(format!("Error reading to buffer: {}", e).into()) //Err(ConnectionHandlerError::Io(e))
        },
    }
}

#[cfg(test)]
mod tests {
use crate::connection::handler::{buffer_ends_with_room_tag, xml_payload_from_buffer};

    #[test]
    fn extracts_xml_payload_from_room_message() {
        let buffer = b"<room roomId=\"abc\"><data/></room>";
        let payload = xml_payload_from_buffer(buffer);

        assert_eq!(payload, b"<room roomId=\"abc\"><data/></room>");
    }

    #[test]
    fn detects_completed_room_message_without_zero_padding() {
        let mut buffer = vec![0u8; 64];
        let payload = b"<room roomId=\"abc\"><data/></room>";
        buffer[..payload.len()].copy_from_slice(payload);
        buffer.truncate(payload.len());

        assert!(buffer_ends_with_room_tag(&buffer));
    }

    #[test]
    fn does_not_treat_incomplete_message_as_completed_message() {
        let mut buffer = vec![0u8; 64];
        let payload = b"<room roomId=\"abc\"><data/>";
        buffer[..payload.len()].copy_from_slice(payload);
        buffer.truncate(payload.len());

        assert!(!buffer_ends_with_room_tag(&buffer));
    }
}