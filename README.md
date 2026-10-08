# Software-Challenge 2026/27 Rust Client

## Allgemein

> [!WARNING]
>
> Diese Bibliothek befindet sich noch in der Entwicklung. Es können noch kleinere Änderungen an der Schnittstelle auftreten. Sollte das der Fall sein, werden wir im Release darauf hinweisen.

Das ist die offizielle Rust-Bibliothek für die Programmierung von Spielern für die [Software-Challenge Germany](https://software-challenge.de/) auf [crates.io](https://crates.io/crates/socha).

## Features

- Vollständige Kommunikation mit Spielservern
- Vollständige Berechnung von Spielzügen
- Simulation von Spielzügen auf GameStates (Beispielsweise für den Minimax Algorithmus vorausgesetzt)
- Simulation vom letzten Zug statt Auslesen des gesamten neuen GameStates für bessere Performance
- Viele Funktionen, welche die Entwicklung erleichtern
- Kapselung auf alle größeren Datentypen
  - Der Rust Compiler inlined solche Funktionen. Dadurch hat man direkten Zugriff auf die Variablen (das ist schneller).

## Kontakt und Support

Eröffnet bei Fehlern oder euer Meinung nach fehlenden Funktionen gerne ein Issue auf GitHub und/oder schreibt es in den [Software-Challenge Discord](https://discord.gg/jhyF7EU) im "Probleme" Kanal.
Außerdem könnt ihr euch an

- SturmEnte
- NichtNil5

auf Discord wenden.

In `examples` können Beispiel-Implementierungen für Spieler gefunden werden.

## Eigenen Spieler erstellen

1. Installiert Rust (mindestens Version <!-- rust-version -->1.88<!-- /rust-version --> für <!-- edition -->2024<!-- /edition --> Edition) und Cargo (wird mit Rust installiert).
2. Erzeugt über die Kommandozeile mit `cargo new best_player` ein neues Cargo-Projekt.
3. Setzt eine beliebige Rust Entwicklungsumgebung auf und importiert das Projekt.
4. Kopiert anschließend den Zufallsspieler (siehe `examples/basic_player`) oder folgende Spielervorlage in `main.rs`:

```rust
use socha::prelude::*;

struct Player {
    game_state: Option<GameState>,
}

impl Client for Player {
    fn on_move_request(&mut self) -> Option<Move> {
        println!("Received a move request!");
      gamerulelogic::get_possible_moves(&self.game_state.as_mut().unwrap()).first().cloned()
    }

    fn on_game_over(&mut self) {
        println!("Game over!");
    }

    fn on_game_state_updated(&mut self, game_state: GameState ) {
        game_state.get_board().print_board();
        self.game_state = Some(game_state);
    }
}

fn main() {
    let client = Player { game_state: None };
    start_client_from_commandline_args(client).unwrap();
}
```

5. Führt `cargo add socha` aus, um die Bibliothek zu eurem Projekt hinzuzufügen.
6. Startet euren Spieler mit `cargo run` und verbindet ihn mit dem Spielserver (siehe den [Ein-neues-Spiel-erstellen](https://docs.software-challenge.de/grundlagen/server#ein-neues-spiel-erstellen) Guide mit einem "Manuell gestarteten Computerspieler")

## Start Argumente

Die folgenden Befehle könnne beim starten des Spielers mit angehangen werden.\
Diese Befehle sind vorallem für das Contest-System relevant, weil es damit dem Spieler die benötigten Informationen übergibt.

| **Befehl** | **Beschreibung** | **Standart** |
| :--- | :--- | :---: | 
| **-h, --host** | Der Host, zu dem eine Verbindung hergestellt werden soll. | 'localhost' |
| **-p, --port** | Der Port des Hosts. | 13050 | 
| **-r, --reservation** | Reservierungscode für ein vorbereitetes Spiel. | / |

## API-Dokumentation

Die öffentliche Rust-API wird direkt über Rustdoc-Kommentare in den Modulen und Typen dokumentiert.
Wenn ihr die Dokumentation lokal anschauen wollt, startet einfach `cargo doc --open` im Projekt oder nutzt die generierte Doku auf [docs.rs](https://docs.rs/socha/latest/socha/).

Die wichtigsten Einstiegspunkte sind:

- `socha::client::Client`
- `socha::game::board::Board`
- `socha::game::gamestate::GameState`
- `socha::game::gamerulelogic`
- `socha::game::piece::{Piece, PieceType}`
- `socha::game::coordinate::Coordinate`

Für fast alles was ihr machen wollt, sollte durch den Import von `socha::prelude::*` alles wichtige Importiert werden.

## Nutzung und Anpassung

Für die Teilnahme an der Software-Challenge dürfen alle Dateien aus diesem Repository beliebig verwendet und verändert werden.

## Erwähnung

Bis zur crates.io Version 0.2.2 wurde die Bibliothek von [Simon Creates](https://github.com/simoncreates/socha) zur Verfügung gestellt.