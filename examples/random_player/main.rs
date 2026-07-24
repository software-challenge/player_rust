use socha::connection::handler::ConnectionHandler;

fn main() {
    let connection = ConnectionHandler::new_from_commandline_args().unwrap();
}