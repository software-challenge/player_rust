use std::{fs::OpenOptions, io::{Read, Write}, net::TcpStream};
use std::fmt::Write as _;

use xml::EventReader;

use crate::connection::parser::{parse_joined::parse_joined, parse_message::parse_message, message::Message};
use crate::game::r#move::Move;

/// A trait indicating that a connection is active.
pub trait IsConnected {}

///Indicates that the ConnectionHandler is connected.
pub struct Connected;
impl IsConnected for Connected {}
///Indicates that the ConnectionHandler has joined a game, also holds the room id.
pub struct Joined {
    room_id: Box<str>
}
impl IsConnected for Joined {}

#[derive(Debug)]
pub struct ConnectionHandler<State> {
    pub connection: TcpStream,
    log_file: std::fs::File,
    state: State,
}


impl ConnectionHandler<()> {
    /// Attempts to create a new ConnectionHandler instance by connecting to the specified host and port.
    /// If no host or port is provided, defaults to "127.0.0.1" and "13050".
    pub fn try_new(host: Option<&str>, port: Option<&str>) -> Result<ConnectionHandler<Connected>, Box<dyn std::error::Error>> {
        // Construct address using provided host and port, or default values if not provided
        let host = host.unwrap_or("127.0.0.1");
        let port = port.unwrap_or("13050");

        let address = format!("{}:{}", host, port);

        Ok(ConnectionHandler{
            connection: TcpStream::connect(address)?,
            // Debugging
            log_file: OpenOptions::new()
                .create(true)
                .append(true)
                .open("connection_reads.log")?,
                state: Connected,
        })
    }

    /// Creates a new `ConnectionHandler` instance by retrieving competition system parameters from command line arguments.
    /// Automatically connects to the competition system using the provided host, port and reservation code.
    pub fn new_from_commandline_args() -> Result<ConnectionHandler<Joined>, Box<dyn std::error::Error>> {
        let cmd_args = crate::util::cmdl_args::get_competition_system_parameters();
        
        let connection_handler = Self::try_new(cmd_args.get_host().as_deref(), cmd_args.get_port().as_deref())?;

        connection_handler.join(cmd_args.get_reservation().as_deref())
    }

    /// Checks if the buffer ends with the "</room>" closing tag.
    fn buffer_ends_with_room_tag(buffer: &[u8]) -> bool {
        buffer.ends_with(b"</room>")
    }

    fn xml_payload_from_buffer(buffer: &[u8]) -> &[u8] {
        let start = buffer.iter().position(|&b| b == b'<').unwrap_or(0);
        &buffer[start..]
    }
}

impl ConnectionHandler<Connected> {
    /// Joins a game, optionally using a reservation code if provided.
    pub fn join(mut self, reservation_code: Option<&str>) -> Result<ConnectionHandler<Joined>, Box<dyn std::error::Error>> {
        match reservation_code {
            Some(rc) => self.connection.write(format!("<protocol><joinPrepared reservationCode=\"{}\"/>", rc).as_bytes())?,
            None => self.connection.write(b"<protocol><join/>")?
        };
        
        // Receive the welcome message from the server and read it into the buffer
        let buffer = self.read_message_to_buffer()?;

        if buffer.is_empty() {
            return Err("No bytes received by the server".into()); //Err(ConnectionHandlerError::ZeroBytesReadToBuffer);
        }

        if !buffer.starts_with(b"<protocol>"){return Err("Invalid XML format".into());}

        // Parse the welcome message to extract the roomId
        let raw_xml = ConnectionHandler::xml_payload_from_buffer(&buffer);
        let parser: EventReader<&[u8]> = EventReader::new(raw_xml);
    
        return Ok(ConnectionHandler { 
            connection: self.connection,
            log_file: self.log_file,
            state: Joined { room_id: parse_joined(parser)? }
        });
    }


}

impl ConnectionHandler<Joined> {
    pub fn get_room_id(&self) -> &Box<str> {
        &self.state.room_id
    }

    /// Sends a move to the server in XML format.
    pub fn send_move(&mut self, m: &Move) -> Result<(), Box<dyn std::error::Error>> {

        let mut move_xml = String::new();
        write!(move_xml, "<room roomId=\"{}\">", self.get_room_id().as_ref())?;

        if m.skip {
            write!(move_xml, "<data class=\"sc.plugin2027.SkipMove\"><color>{}</color></data></room>", m.team.to_string())?;
        } else {
            write!(move_xml, "<data class=\"sc.plugin2027.SetMove\"><piece color=\"{}\" kind=\"{}\" rotation=\"{}\" isFlipped=\"{}\"><position x=\"{}\" y=\"{}\"/></piece></data></room>",
            m.team.to_string(),
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
        let buffer: Vec<u8> = self.read_message_to_buffer()?;

        if buffer.is_empty() {
            return Err("No bytes received by the server".into()); //Err(ConnectionHandlerError::ZeroBytesReadToBuffer);
        }

        let raw_xml: &[u8] = ConnectionHandler::xml_payload_from_buffer(&buffer);
        let parser: EventReader<&[u8]> = EventReader::new(raw_xml);
        let message: Box<Message> = parse_message(parser)?;

        Ok(message)
    }

    
}

impl<State: IsConnected> ConnectionHandler<State> {
    fn read_message_to_buffer(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut buffer = Vec::new();

        loop {
            let number_of_new_bytes = self.read_to_buffer(&mut buffer)?;

            if number_of_new_bytes == 0 {
                return Err("Zero Bytes Read To Buffer".into()); //Err(ConnectionHandlerError::ZeroBytesReadToBuffer);
            }

            if ConnectionHandler::buffer_ends_with_room_tag(&buffer) {
                return Ok(buffer);
            }
        }
    }

    fn read_to_buffer(&mut self, buffer: &mut Vec<u8>) -> Result<usize, Box<dyn std::error::Error>> {
        let start_len = buffer.len();
        buffer.resize(start_len + 4096, 0);

        match self.connection.read(&mut buffer[start_len..]){
            Ok(0) => {
                buffer.truncate(start_len);
                Err("Zero Bytes Read To Buffer".into()) //Err(ConnectionHandlerError::ZeroBytesReadToBuffer);
            },
            Ok(b) => {
                buffer.truncate(start_len + b);

                // Debugging
                self.log_file.write_all(&buffer[start_len..start_len + b])?;
                self.log_file.write_all(b"\n")?;
                self.log_file.flush()?;

                Ok(b)
            },
            Err(e) => {
                buffer.truncate(start_len);
                Err(format!("Error reading to buffer: {}", e).into()) //Err(ConnectionHandlerError::Io(e))
            },
        }
    }
}


#[cfg(test)]
mod tests {
use super::ConnectionHandler;

    #[test]
    fn extracts_xml_payload_from_room_message() {
        let buffer = b"<room roomId=\"abc\"><data/></room>";
        let payload = ConnectionHandler::xml_payload_from_buffer(buffer);

        assert_eq!(payload, b"<room roomId=\"abc\"><data/></room>");
    }

    #[test]
    fn detects_completed_room_message_without_zero_padding() {
        let mut buffer = vec![0u8; 64];
        let payload = b"<room roomId=\"abc\"><data/></room>";
        buffer[..payload.len()].copy_from_slice(payload);
        buffer.truncate(payload.len());

        assert!(ConnectionHandler::buffer_ends_with_room_tag(&buffer));
    }

    #[test]
    fn does_not_treat_incomplete_message_as_completed_message() {
        let mut buffer = vec![0u8; 64];
        let payload = b"<room roomId=\"abc\"><data/>";
        buffer[..payload.len()].copy_from_slice(payload);
        buffer.truncate(payload.len());

        assert!(!ConnectionHandler::buffer_ends_with_room_tag(&buffer));
    }
}