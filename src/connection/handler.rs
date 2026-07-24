use std::{io::{Read, Write}, net::TcpStream};

use crate::game::{board::Board, gamestate::GameState};

pub struct ConnectionHandler {
    pub(super) connected: bool,
    pub(super) connection: TcpStream,
    pub(super) room_id: Option<Box<str>>,
    pub(super) bord:Option<Board>,
    pub(super) game_state: Option<GameState>,
    //pub(super) last_game_message: GameMessage,
}

impl ConnectionHandler {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(
            ConnectionHandler{
                connected: false,
                connection: TcpStream::connect("127.0.0.1:13050")?,
                room_id: None,
                bord: None,
                game_state: None,
            }
        )
    }

    pub fn join(&mut self, reservation_code: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        
        let mut buffer = [0; 200];

        match reservation_code {
            Some(rc) => self.connection.write(format!("<protocol><joinPrepared reservationCode=\"{}\"/>", rc).as_bytes())?,
            None => self.connection.write(b"<protocol><join/>")?
        };

        let last_none_zero_index = self.read_to_buffer(&mut buffer)? -1;

        //if buffer.is_empty() {return Err(ConnectionHandlerError::ZeroBytesReadToBuffer);}

        if !buffer.starts_with(b"<protocol>"){return Err("Invalid XML format".into());}

        //let parser = EventReader::new(&buffer[10..=last_none_zero_index]);
        
        //self.room_id = Some(parse_joined(parser)?);
        self.connected = true; 
        return Ok(());
    }

    pub fn read_to_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, Box<dyn std::error::Error>> {
        match self.connection.read(buffer){
            Ok(0) => {
                return Err("No bytes read to buffer".into()); //Err(ConnectionHandlerError::ZeroBytesReadToBuffer);
            },
            Ok(b) => {
                #[cfg(feature = "log_incoming_xml")]
                if self.xml_input_file.is_some() {
                    self.xml_input_file.as_mut().unwrap().write(&buffer[..=b - 1])?;
                }

                return Ok(b)
            },
            Err(e) => return Err("Error reading to buffer".into()), //Err(ConnectionHandlerError::Io(e)),
        }
    }
}