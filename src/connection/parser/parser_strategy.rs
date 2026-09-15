use xml::EventReader;

use crate::connection::parser::message::Message;

pub trait ParserStrategy {
    fn parse_memento(parser: EventReader<&[u8]>) -> Box<Message>;
}