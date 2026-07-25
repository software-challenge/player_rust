use std::{fs::OpenOptions, io::{Read, Write}, net::TcpStream};

use xml::EventReader;

use crate::connection::parser::{parse_joined::parse_joined, parse_message::parse_message, message::Message};

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
        
        let mut buffer: [u8; 200] = [0; 200];

        match reservation_code {
            Some(rc) => self.connection.write(format!("<protocol><joinPrepared reservationCode=\"{}\"/>", rc).as_bytes())?,
            None => self.connection.write(b"<protocol><join/>")?
        };
        
        // Receive the welcome message from the server and read it into the buffer
        let last_index: usize = self.read_message_to_buffer(&mut buffer)? -1;

        if buffer.is_empty() {
            return Err("No bytes received by the server".into()); //Err(ConnectionHandlerError::ZeroBytesReadToBuffer);
        }

        if !buffer.starts_with(b"<protocol>"){return Err("Invalid XML format".into());}

        // Parse the welcome message to extract the roomId
        let raw_xml = Self::xml_payload_from_buffer(&buffer, last_index);
        let parser: EventReader<&[u8]> = EventReader::new(raw_xml);
        self.room_id = Some(parse_joined(parser)?);
    
        self.connected = true; 
        return Ok(());
    }

    /// Reads a new message from the server, parses it, and returns the parsed message
    pub fn get_new_message(&mut self) -> Result<Box<Message>, Box<dyn std::error::Error>> {
        let mut buffer = [0; 4096];
        let last_index: usize = self.read_message_to_buffer(&mut buffer)? -1;

        if buffer.is_empty() {
            return Err("No bytes received by the server".into()); //Err(ConnectionHandlerError::ZeroBytesReadToBuffer);
        }

        let raw_xml = Self::xml_payload_from_buffer(&buffer, last_index);
        let parser: EventReader<&[u8]> = EventReader::new(raw_xml);
        let message: Box<Message> = parse_message(parser)?;

        return Ok(message);
    }

    fn xml_payload_from_buffer(buffer: &[u8], last_index: usize) -> &[u8] {
        let start = buffer.iter().position(|&b| b == b'<').unwrap_or(0);
        &buffer[start..=last_index]
    }

    fn read_message_to_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, Box<dyn std::error::Error>> {

        let mut buffer_index: usize = self.read_to_buffer(buffer)?;

        while !Self::buffer_ends_with_room_tag(buffer, buffer_index) {
            let number_of_new_bytes: usize = self.read_to_buffer(&mut buffer[buffer_index..])?;
            buffer_index += number_of_new_bytes;
        }

        Ok(buffer_index)
    }

    fn buffer_ends_with_room_tag(buffer: &[u8], bytes_in_buffer: usize) -> bool {
        buffer[..bytes_in_buffer].ends_with(b"</room>")
    }

    fn read_to_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, Box<dyn std::error::Error>> {
        match self.connection.read(buffer){
            Ok(0) => {
                return Err("Zero Bytes Read To Buffer".into()); //Err(ConnectionHandlerError::ZeroBytesReadToBuffer);
            },
            Ok(b) => {
                // Debugging
                self.log_file.write_all(&buffer[..b])?;
                self.log_file.write_all(b"\n")?;
                self.log_file.flush()?;

                return Ok(b)
            },
            Err(e) => return Err(format!("Error reading to buffer: {}", e).into()), //Err(ConnectionHandlerError::Io(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ConnectionHandler;

    #[test]
    fn extracts_xml_payload_from_room_message() {
        let buffer = b"<room roomId=\"abc\"><data/></room>";
        let payload = ConnectionHandler::xml_payload_from_buffer(buffer, buffer.len() - 1);

        assert_eq!(payload, b"<room roomId=\"abc\"><data/></room>");
    }

    #[test]
    fn detects_completed_room_message_without_zero_padding() {
        let mut buffer = [0u8; 64];
        let payload = b"<room roomId=\"abc\"><data/></room>";
        buffer[..payload.len()].copy_from_slice(payload);

        assert!(ConnectionHandler::buffer_ends_with_room_tag(&buffer, payload.len()));
    }

    #[test]
    fn does_not_treat_zero_padding_as_completed_message() {
        let mut buffer = [0u8; 64];
        let payload = b"<room roomId=\"abc\"><data/>";
        buffer[..payload.len()].copy_from_slice(payload);

        assert!(!ConnectionHandler::buffer_ends_with_room_tag(&buffer, payload.len()));
    }
}