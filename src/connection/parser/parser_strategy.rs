use crate::connection::parser::message::Message;

pub trait ParserStrategy {
    fn parse_memento(xml: &str) -> Result<Box<Message>, Box<dyn std::error::Error>>;
}