use std::{fs::OpenOptions, io::{Read, Write}, net::TcpStream};

use xml::EventReader;

use crate::connection::parser::{parse_joined::parse_joined, parse_message::parse_message, message::Message};
use crate::game::r#move::Move;

pub struct ConnectionHandler {
    pub connected: bool,
    pub connection: TcpStream,
    log_file: std::fs::File,
    pub room_id: Option<Box<str>>,
}

impl ConnectionHandler {

    /// Creates a new `ConnectionHandler` instance by retrieving competition system parameters from command line arguments.
    /// Automatically connects to the competition system using the provided host, port and reservation code.
    pub fn new_from_commandline_args() -> Result<Self, Box<dyn std::error::Error>> {
        
        let cmd_args = crate::util::cmdl_args::get_competition_system_parameters();

        // Construct address using provided host and port, or default values if not provided
        let host = if let Some(host) = cmd_args.0 {
            host
        } else {
            Box::from("127.0.0.1")
        };

        let port = if let Some(port) = cmd_args.1 {
            port
        } else {
            Box::from("13050")
        };

        let address = format!("{}:{}", host, port);

        let mut connection_handler = ConnectionHandler{
            connected: false,
            connection: TcpStream::connect(address)?,
            room_id: None,
            // Debugging
            log_file: OpenOptions::new()
                .create(true)
                .append(true)
                .open("connection_reads.log")?,
        };

        connection_handler.join(cmd_args.2.as_deref())?;

        return Ok(connection_handler);
    }

    /// Joins a game, optionally using a reservation code if provided.
    pub fn join(&mut self, reservation_code: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
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
        let raw_xml = Self::xml_payload_from_buffer(&buffer);
        let parser: EventReader<&[u8]> = EventReader::new(raw_xml);
        self.room_id = Some(parse_joined(parser)?);
    
        self.connected = true; 
        return Ok(());
    }

    /// Sends a move to the server in XML format.
    pub fn send_move(&mut self, m: &Move) -> Result<(), Box<dyn std::error::Error>> {
        let move_xml = format!(
            "<room roomId=\"{}\"><data class=\"sc.plugin2027.SetMove\"><piece color=\"{}\" kind=\"{}\" rotation=\"NONE\" isFlipped=\"false\"><position x=\"{}\" y=\"{}\"/></piece></data></room>",
            self.room_id.as_ref().unwrap_or(&Box::from("")),
            m.team.to_string(),
            m.piece.to_string(),
            m.x,
            m.y
        );

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

        let raw_xml: &[u8] = Self::xml_payload_from_buffer(&buffer);
        let parser: EventReader<&[u8]> = EventReader::new(raw_xml);
        let message: Box<Message> = parse_message(parser)?;

        return Ok(message);
    }

    fn xml_payload_from_buffer(buffer: &[u8]) -> &[u8] {
        let start = buffer.iter().position(|&b| b == b'<').unwrap_or(0);
        &buffer[start..]
    }

    fn read_message_to_buffer(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut buffer = Vec::new();

        loop {
            let number_of_new_bytes = self.read_to_buffer(&mut buffer)?;

            if number_of_new_bytes == 0 {
                return Err("Zero Bytes Read To Buffer".into()); //Err(ConnectionHandlerError::ZeroBytesReadToBuffer);
            }

            if Self::buffer_ends_with_room_tag(&buffer) {
                return Ok(buffer);
            }
        }
    }

    fn buffer_ends_with_room_tag(buffer: &[u8]) -> bool {
        buffer.ends_with(b"</room>")
    }

    fn read_to_buffer(&mut self, buffer: &mut Vec<u8>) -> Result<usize, Box<dyn std::error::Error>> {
        let start_len = buffer.len();
        buffer.resize(start_len + 4096, 0);

        match self.connection.read(&mut buffer[start_len..]){
            Ok(0) => {
                buffer.truncate(start_len);
                return Err("Zero Bytes Read To Buffer".into()); //Err(ConnectionHandlerError::ZeroBytesReadToBuffer);
            },
            Ok(b) => {
                buffer.truncate(start_len + b);

                // Debugging
                self.log_file.write_all(&buffer[start_len..start_len + b])?;
                self.log_file.write_all(b"\n")?;
                self.log_file.flush()?;

                return Ok(b)
            },
            Err(e) => {
                buffer.truncate(start_len);
                return Err(format!("Error reading to buffer: {}", e).into()); //Err(ConnectionHandlerError::Io(e))
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