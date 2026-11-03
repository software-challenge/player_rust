use crate::{
    connection::parser::message::Message,
    games::active::r#move::Move,
};

pub trait ParserStrategy {
    /// Parses the memento message received from the server into a `Message`.
    fn parse_memento(xml: &str) -> Result<Box<Message>, Box<dyn std::error::Error>>;

    /// Serializes a move into the XML message that is sent to the server.
    fn move_to_xml(&self, room_id: &str, r#move: &Move) -> String;
}
