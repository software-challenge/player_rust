use crate::{
    client::Client,
    games::blokus2027::piece,
};

/// Per-game initialization, run once before the client starts communicating
/// with the server.
///
/// Every game has its own `initialize` implementation; the generic client
/// only calls it once before the message loop.
// `client` is not used by blokus2027 yet; the parameter is part of the
// per-game hook so future games can set up their client here.
pub fn initialize<C: Client>(_client: &mut C) {
    piece::populate_piece_variants();
}
