use std::env;

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
pub fn get_competition_system_parameters() -> (Option<Box<str>>, Option<Box<str>>, Option<Box<str>>) {
    let args: Vec<String> = env::args().collect();
    let mut host = None;
    let mut port = None;
    let mut reservation = None;
    println!("{:?}", args);
    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "-h" | "--host" => host = Some(Box::from(args[i + 1].as_str())),
            "-p" | "--port" => port = Some(Box::from(args[i + 1].as_str())),
            "-r" | "--reservation" => reservation = Some(Box::from(args[i + 1].as_str())),
            _ => {}
        }
    }
    return (host, port, reservation)
}