use socha::connection::handler::ConnectionHandler;

#[test]
fn connection_test() {
    let mut connection = ConnectionHandler::new().unwrap();
    connection.join(None).unwrap();
}