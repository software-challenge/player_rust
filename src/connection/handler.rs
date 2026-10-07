#![allow(clippy::indexing_slicing)]

use std::{
    fmt::Write as _, io::{self, Read, Write}, net::TcpStream
};

#[cfg(feature = "debug-recv-comm-log")]
use std::fs::OpenOptions;

use quick_xml::Reader;

use crate::{
    connection::parser::{
        message::Message, 
        parse_joined::parse_joined, 
        parse_result::parse_result,
        parser_strategy::ParserStrategy
    },
    game::r#move::Move
};

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
    receive_buffer: Vec<u8>,
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
            receive_buffer: Vec::new(),
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
        #[cfg(feature = "debug-recv-comm-log")]
        let buffer = read_message_to_buffer(
            &mut self.connection,
            &mut self.receive_buffer,
            &mut self.log_file,
        )?;
        #[cfg(not(feature = "debug-recv-comm-log"))]
        let buffer = read_message_to_buffer(&mut self.connection, &mut self.receive_buffer)?;

        if buffer.is_empty() {
            return Err("No bytes received by the server".into()); //Err(ConnectionHandlerError::ZeroBytesReadToBuffer);
        }

        if !buffer.starts_with(b"<protocol>"){return Err("Invalid XML format".into());}

        // Parse the welcome message to extract the roomId
        let raw_xml: &[u8] = xml_payload_from_buffer(&buffer);
    
        return Ok(ConnectionHandler { 
            connection: self.connection,
            receive_buffer: self.receive_buffer,
            #[cfg(feature = "debug-recv-comm-log")]
            log_file: self.log_file,
            state: Joined { room_id: parse_joined(raw_xml)? },
            strategy: self.strategy
        });
    }


}

impl<S: ParserStrategy> ConnectionHandler<Joined, S> {
    pub fn get_room_id(&self) -> &str {
        &self.state.room_id
    }

    /// Sends a move to the server in XML format.
    pub fn send_move(&mut self, m: &Move) -> Result<(), Box<dyn std::error::Error>> {

        let mut move_xml = String::new();
        write!(move_xml, "<room roomId=\"{}\">", self.get_room_id())?;

        if m.skip {
            write!(move_xml, "<data class=\"sc.plugin2027.SkipMove\"><color>{}</color></data></room>", m.color)?;
        } else {
            write!(move_xml, "<data class=\"sc.plugin2027.SetMove\"><piece color=\"{}\" kind=\"{}\" rotation=\"{}\" isFlipped=\"{}\"><position x=\"{}\" y=\"{}\"/></piece></data></room>",
            m.color,
            m.piece,
            m.rotation,
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
        #[cfg(feature = "debug-recv-comm-log")]
        let buffer: Vec<u8> = read_message_to_buffer(
            &mut self.connection,
            &mut self.receive_buffer,
            &mut self.log_file,
        )?;
        #[cfg(not(feature = "debug-recv-comm-log"))]
        let buffer: Vec<u8> = read_message_to_buffer(&mut self.connection, &mut self.receive_buffer)?;

        if buffer.is_empty() {
            return Err("No bytes received by the server".into()); //Err(ConnectionHandlerError::ZeroBytesReadToBuffer);
        }

        let raw_xml: &[u8] = xml_payload_from_buffer(&buffer);
        let message: Box<Message> = Self::parse_message(raw_xml)?;

        Ok(message)
    }

    pub fn parse_message(xml: &[u8])-> Result<Box<Message>, Box<dyn std::error::Error>> {
        let xml_str = std::str::from_utf8(xml)?;

        let data_start = xml_str.find("<data ").ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "could not find <data start tag",
        ))?;
        let data_end = xml_str.rfind("</data>");

        let mut reader = Reader::from_str(xml_str);
        loop {
            match reader.read_event()? {
                quick_xml::events::Event::Start(e) if e.name().as_ref() == "data"=> {
                        let data_end = data_end.ok_or(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "could not find </data> end tag",
                        ))? + 6;
                    match &*e.attributes().find(|attribute| {
                        if let Ok(attr) = attribute {
                            attr.key.as_ref() == "class"
                        } else {
                            false
                        }
                    }).ok_or(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "could not find class atrr on data tag"))??.value {
                        "memento" => {
                            return S::parse_memento(&xml_str[data_start..=data_end])
                        },
                        "result" => {
                            return Ok(parse_result(&xml_str[data_start..=data_end]))
                        },
                        "error" => {eprint!("Error: {}", xml_str)}
                        attr_val => {return Err(Box::from(io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    format!("Unknown class attribute value: {}", attr_val))))}

                    }
                },
                quick_xml::events::Event::Empty(e) if e.name().as_ref() == "data"=> {
                    match &*e.attributes().find(|attribute| {
                        if let Ok(attr) = attribute {
                            attr.key.as_ref() == "class"
                        } else {
                            false
                        }
                    }).ok_or(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "could not find class atrr on data tag"))??.value {
                        "moveRequest" => {
                            return Ok(Box::new(Message::MoveRequest))
                        },
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
}

/// Removes the first complete room message and retains any following bytes.
fn take_room_message(buffer: &mut Vec<u8>) -> Option<Vec<u8>> {
    const ROOM_END_TAG: &[u8] = b"</room>";
    let message_end = buffer
        .windows(ROOM_END_TAG.len())
        .position(|window| window == ROOM_END_TAG)?
        + ROOM_END_TAG.len();
    let remaining = buffer.split_off(message_end);
    Some(std::mem::replace(buffer, remaining))
}

fn xml_payload_from_buffer(buffer: &[u8]) -> &[u8] {
    let start = buffer.iter().position(|&b| b == b'<').unwrap_or(0);
    &buffer[start..]
}

fn read_message_to_buffer(
    tcp_stream: &mut TcpStream,
    buffer: &mut Vec<u8>,
    #[cfg(feature = "debug-recv-comm-log")] log_file: &mut std::fs::File,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    loop {
        if let Some(message) = take_room_message(buffer) {
            return Ok(message);
        }

        #[cfg(feature = "debug-recv-comm-log")]
        let number_of_new_bytes = read_to_buffer(tcp_stream, buffer, log_file)?;
        #[cfg(not(feature = "debug-recv-comm-log"))]
        let number_of_new_bytes = read_to_buffer(tcp_stream, buffer)?;

        if number_of_new_bytes == 0 {
            return Err("Zero Bytes Read To Buffer".into()); //Err(ConnectionHandlerError::ZeroBytesReadToBuffer);
        }
    }
}

fn read_to_buffer(
    tcp_stream: &mut TcpStream,
    buffer: &mut Vec<u8>,
    #[cfg(feature = "debug-recv-comm-log")] log_file: &mut std::fs::File,
) -> Result<usize, Box<dyn std::error::Error>> {
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
                log_file.write_all(&buffer[start_len..start_len + b])?;
                log_file.write_all(b"\n")?;
                log_file.flush()?;
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
    use crate::connection::handler::{take_room_message, xml_payload_from_buffer};
    use crate::{connection::parser::message::Message, game::parser::Blokus2026};

    #[test]
    fn extracts_xml_payload_from_room_message() {
        let buffer = b"<room roomId=\"abc\"><data/></room>";
        let payload = xml_payload_from_buffer(buffer);

        assert_eq!(payload, b"<room roomId=\"abc\"><data/></room>");
    }

    #[test]
    fn extracts_first_room_message_and_retains_trailing_bytes() {
        let message = b"<room roomId=\"abc\"><data/></room>";
        let trailing = b"<removedFromGame roomId=\"abc\"/>";
        let mut buffer = [message.as_slice(), trailing.as_slice()].concat();

        assert_eq!(take_room_message(&mut buffer), Some(message.to_vec()));
        assert_eq!(buffer, trailing);
    }

    #[test]
    fn extracts_coalesced_room_messages_in_order() {
        let first = b"<room roomId=\"first\"><data/></room>";
        let second = b"<room roomId=\"second\"><data/></room>";
        let mut buffer = [first.as_slice(), second.as_slice()].concat();

        assert_eq!(take_room_message(&mut buffer), Some(first.to_vec()));
        assert_eq!(take_room_message(&mut buffer), Some(second.to_vec()));
        assert!(buffer.is_empty());
    }

    #[test]
    fn leaves_incomplete_room_message_buffered() {
        let mut buffer = b"<room roomId=\"abc\"><data/>".to_vec();

        assert_eq!(take_room_message(&mut buffer), None);
        assert_eq!(buffer, b"<room roomId=\"abc\"><data/>");
    }

    #[test]
    fn parses_move_request_message() {
        let xml = br#"<room roomId="room123"><data class="moveRequest"/></room>"#;

        let message = super::ConnectionHandler::<super::Joined, Blokus2026>::parse_message(xml)
            .expect("valid move request should parse");

        assert_eq!(message.as_ref(), &Message::MoveRequest);
    }

    #[test]
    fn parses_result_message() {
        let xml = br#"<room roomId="room123"><data class="result"><definition/><scores/><winner regular="false"/></data></room>"#;

        let message = super::ConnectionHandler::<super::Joined, Blokus2026>::parse_message(xml)
            .expect("valid result message should parse");

        assert!(matches!(
            message.as_ref(),
            Message::Result(Some(_))
        ));
    }

    #[test]
    fn rejects_unknown_data_class() {
        let xml = br#"<room roomId="room123"><data class="unknown"/></room>"#;

        let error = super::ConnectionHandler::<super::Joined, Blokus2026>::parse_message(xml)
            .expect_err("unknown data classes should be rejected");

        assert!(error.to_string().contains("Unknown class attribute value: unknown"));
    }

    #[test]
    fn rejects_message_without_data_element() {
        let xml = br#"<room roomId="room123"/>"#;

        let error = super::ConnectionHandler::<super::Joined, Blokus2026>::parse_message(xml)
            .expect_err("messages without a data element should be rejected");

        assert!(error.to_string().contains("could not find <data start tag"));
    }
}
