use std::{io::{Read, Write}, net::TcpStream};

use xml::EventReader;

use crate::{connection::parser::parse_joined::parse_joined, game::{board::Board, gamestate::GameState}};

pub struct ConnectionHandler {
    pub connected: bool,
    pub connection: TcpStream,
    pub room_id: Option<Box<str>>,
    pub board:Option<Board>,
    pub game_state: Option<GameState>,
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
            board: None,
            game_state: None,
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

        let last_index: usize = self.read_message_to_buffer(&mut buffer)? -1;

        if buffer.is_empty() {
            return Err("No bytes received by the server".into()); //Err(ConnectionHandlerError::ZeroBytesReadToBuffer);
        }

        if !buffer.starts_with(b"<protocol>"){return Err("Invalid XML format".into());}

        // Parse the welcome message to extract the roomId
        let parser: EventReader<&[u8]> = EventReader::new(&buffer[10..=last_index]);
        self.room_id = Some(parse_joined(parser)?);
    
        self.connected = true; 
        return Ok(());
    }

    pub fn read_message_to_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, Box<dyn std::error::Error>> {

        let mut buffer_index: usize = self.read_to_buffer(buffer)?;

        if buffer[..=buffer_index].ends_with(b"</room>") { return Ok(buffer_index) };

        loop{
            let number_of_new_bytes: usize = self.read_to_buffer(&mut buffer[buffer_index + 1..])?;
            buffer_index += number_of_new_bytes;
            if buffer[..=buffer_index].ends_with(b"</room>") {return Ok(buffer_index)};
        } 
    }

    fn read_to_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, Box<dyn std::error::Error>> {
        match self.connection.read(buffer){
            Ok(0) => {
                return Err("Zero Bytes Read To Buffer".into()); //Err(ConnectionHandlerError::ZeroBytesReadToBuffer);
            },
            Ok(b) => {
                return Ok(b)
            },
            Err(e) => return Err("Error reading to buffer".into()), //Err(ConnectionHandlerError::Io(e)),
        }
    }
}