use std::env;


pub struct CompetitionSystemParameters {
    host: Option<Box<str>>,
    port: Option<Box<str>>,
    reservation: Option<Box<str>,>
}

impl CompetitionSystemParameters {
    pub fn new(host: Option<Box<str>>, port: Option<Box<str>>, reservation: Option<Box<str>>) -> Self {
        CompetitionSystemParameters { host, port, reservation }
    }

    pub fn get_host(&self) -> &Option<Box<str>> {
        &self.host
    }

    pub fn get_port(&self) -> &Option<Box<str>> {
        &self.port
    }

    pub fn get_reservation(&self) -> &Option<Box<str>> {
        &self.reservation
    }
}

/// Retrieves competition system parameters from command line arguments.
///
/// This function parses command line arguments to find the following parameters:
/// - `host`: The host address for the competition system.
/// - `port`: The port number for the competition system.
/// - `reservation`: A reservation identifier for the competition system.
///
/// The function looks for specific flags in the command line arguments:
/// - `-h` or `--host` to specify the host address.
/// - `-p` or `--port` to specify the port number.
/// - `-r` or `--reservation` to specify the reservation identifier.
///
/// The function returns a tuple containing three `Option<Box<str>>` values:
/// - The first element is the host address, if provided.
/// - The second element is the port number, if provided.
/// - The third element is the reservation identifier, if provided.
pub fn get_competition_system_parameters() -> CompetitionSystemParameters{
    let args: Vec<String> = env::args().collect();
    let mut host = None;
    let mut port = None;
    let mut reservation = None;
    let mut iter = args.into_iter();
    while let Some(flag) = iter.next() {
        match flag.as_str() {
            "-h" | "--host" => host = iter.next().map(String::into_boxed_str),
            "-p" | "--port" => port = iter.next().map(String::into_boxed_str),
            "-r" | "--reservation" => reservation = iter.next().map(String::into_boxed_str),
            _ => {}
        }
    }
    
    CompetitionSystemParameters::new(host, port, reservation)
}