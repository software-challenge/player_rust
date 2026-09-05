# Software Challenge 2026/27 Rust Client

## Allgemein

Das ist die offiziele Rust-Bibliothek für die Programmierung von Spielern für die [Software Challenge Germany](https://software-challenge.de/) auf [crates.io](https://crates.io/crates/socha).

## Wie wird das Library verwendet? 

In `examples/random_player` kann eine Beispiel-Implementierung für einen Zufallsspieler gefunden werden.

Um das Projekt zu erstellen führt ihr einmal ``cargo init`` in dem Ordner, in dem ihr euren Spieler programmieren wollt, aus.
Alternativ könnt ihr auch ``cargo new projekt_name`` in dem Ordner, in dem der Projektordner sein soll, ausführen.
Cargo erstellt dann für euch in der richigen Ordnerstruktur alle wichtigen Dateien.

Als nächstes muss die Bibliothek zu dem Projekt hinzugefügt werden.
Das macht ihr mit ``cargo add socha``.

Dann kommen wir schon zu Programmierung.
Öffnet im src Ordner, der von Cargo erstellt wurde, die main.rs Datei (Beispielsweise in Visual Studio Code).
In der Datei wurde durch Cargo schon in etwa folgender Code eingefügt:
```rs
fn main() {
    println!("Hello, world!");
}
```
Diesen Code könnt ihr einfach löschen und dann folgenden Code einfügen:
```rs
use socha::game::{gamestate::GameState, gamerulelogic, r#move::Move};
use socha::client::{Client, start_client_from_commandline_args};

struct Player {
    game_state: Option<socha::game::gamestate::GameState>,
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
        game_state.board.print_board();
        self.game_state = Some(game_state);
        println!("Game state updated!");
    }
}

fn main() {
    let client = Player { game_state: None };
    start_client_from_commandline_args(client).unwrap();
}
```

Dieser Code erstellt und startet beim Start des Programms einen neuen Spieler.
Dieser verbindet sich dann mit dem offenem Spiel auf ``localhost:13050``.
Das ist die Standart Adresse und Port, wenn keine Commandlinearguments gegeben sind. 
Diese Adresse und Port wird von dem GUI verwendet, wenn ein Spiel mit der Option "Eigener Computerspieler, manuell gestartet" gestartet wird.
Sobald das Spiel im GUI gestartet wurde schickt der Spieler dann immer den ersten Spielzug aus der Liste der gültigen Spielzüge an den Server.

Erstellt nun also im GUI ein neues Spiel, ein Spieler sollte entweder "Mensch", "Zufalls-Computerspieler" oder "Fortgeschrittener-Computerspieler" sein, die andere "Eigener Computerspieler, manuell gestartet".

Nun könnt ihr den Spieler aus dem Hauptordner mit folgendem Befehl starten: ``cargo run``. 
Der Hauptordner ist bei ``cargo init`` der Ordner, in dem der Befehl ausgeführt wurde, bei ``cargo new projekt_name`` der Ordner, der von Cargo erstellt wurde.

## Schnittstelle


## Spiel spezifische Dateien

- game/*
- connection/parser/*
- util/(manche)

## Erwähnung

Bis zu crates.io Version 0.2.2 wurde die Bibliothek von [Simon Creates](https://github.com/simoncreates/socha) zur Verfügung gestellt.