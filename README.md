# Software-Challenge 2026/27 Rust Client

## Allgemein

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

## Eigenen Spieler erstellen

1. Installiert Rust (mindestens Version 1.85 für 2024 edition) und Cargo (wird mit Rust installiert).
2. Klont das Repository auf euren Rechner.
3. Setzt eine beliebige Rust Entwicklungsumgebung auf und importiert das Projekt.
4. Wechselt in der Kommandozeile in den `examples/random_player` Ordner.
5a. Führt `cargo init` aus, um ein neues Cargo-Projekt zu erstellen, welches den Zufallsspieler enthält. `random_player` ist in diesem Fall das Projektverzeichnis, in dem die folgenden `cargo` Befehle ausgeführt werden müssen.
5b. Überspringt Schritte 2, 4: Alternativ könnt ihr auch `cargo new projekt_name` ausführen, um ein neues Cargo-Projekt zu erstellen.
In diesem Fall wäre `projekt_name` das Projektverzeichnis, in dem die folgenden `cargo` Befehle ausgeführt werden müssen.
Kopiert anschließend den Zufallsspieler oder folgende Spielervorlage in `main.rs`:

```rust
use socha::prelude::*;

struct Player {
    game_state: Option<GameState>,
}

impl Client for Player {
    fn on_move_request(&mut self) -> Option<Move> {
        println!("Received a move request!");
        get_possible_moves(&self.game_state.as_mut().unwrap()).first().cloned()
    }

    fn on_game_over(&mut self) {
        println!("Game over!");
    }

    fn on_game_state_updated(&mut self, game_state: GameState ) {
        game_state.get_board().print_board();
        self.game_state = Some(game_state);
        println!("Game state updated!");
    }
}

fn main() {
    let client = Player { game_state: None };
    start_client_from_commandline_args(client).unwrap();
}
```

6. Führt `cargo add socha` aus, um die Bibliothek zu eurem Projekt hinzuzufügen.
7. Startet euren Spieler mit `cargo run` und verbindet ihn mit dem Spielserver (siehe den [Ein-neues-Spiel-erstellen](https://docs.software-challenge.de/grundlagen/server#ein-neues-spiel-erstellen) Guide mit einem "Manuell gestarteten Computerspieler")

## Schnittstelle

Es stehen euch verschiedene Methoden und Datenstrukturen zur Verfügung. Hier werden die wichtigsten einmal erklärt.
Grundsätzlich sollte in der Regel nur alles im ``socha::game`` für euch relevant sein.

### Client

``socha::client``

Der Client ist für den Gameflow und das Übermitteln von Spielereignissen an euer Programm zuständig.
Mit dem Client-trait müsst ihr ein eigenes Struct mit den entsprechenden Funktionen erstellen.
Euren Client könnt ihr dann an eine der Startfunktionen übermitteln.
Wie euer Client aussehen kann, könnt ihr [hier](#eigenen-spieler-erstellen) sehen.

### Board

``socha::game::board::Board``

Das Board ist das Spielfeld.
Es wird im [Gamestate](#game-state) verwendet, um das aktuelle Spielfeld zu speichern.
Es stehen hier außerdem einige Methoden zu Verfügung.

#### print_board

Hiermit kann das Board in die Konsole ausgegeben werden.

#### get_cell(x: usize, y: usize) -> Option\<Color\>

Hiermit kann eine Zelle mit den gegebenen Koordinaten vom Board ausgelesen werden.

#### set_cell(x: usize, y: usize, color: Color) -> bool

Hiermit kann eine Zelle auf die angegebene Farbe gesetzt werden.
Wenn die Zelle auf dem Spielfeld ist und somit die Zelle auf die Farbe gesetzt wurde, wird ``true`` zurückgegeben, ansonsten wird ``false`` zurückgegeben.

#### place_piece(x: usize, y: usize, color: Color, piece: Piece) -> bool

Hiermit kann ein Spielstein auf dem Spielfeld platziert werden.
Der Spielstein wird nur platziert, wenn alle Koordinaten des Spielsteins noch nicht belegt und innerhalb vom Spielfeld sind.
Wenn der Spielstein erfolgreich platziert wurde, gibt die Funktion ``true`` zurück, ansonsten ``false``.
Diese Überprüfung verbraucht etwas mehr Leistung als die unchecked Variante; wenn ihr euch sicher seid, dass der Spielstein dort platziert werden darf, nutzt die unchecked Variante.

#### place_piece_unchecked(x: usize, y: usize, color: Color, piece: Piece)

Diese Funktion tut das gleiche wie die `place_piece` Funktion, nur dass keine Überprüfung stattfindet, ob der Spielstein platziert werden kann.

### Color

``socha::game::color::Color``

`Color` ist ein Enum mit allen 4 Farben aus dem Spiel.
Es wird beispielsweise im `GameState` genutzt, um Anzugeben welche Farbe einen Zug tätigen soll.
Außerdem wird `Color` im Board verwendet, um die Spielfelder mit der belegten Farbe zu markieren.

#### from_string(s: &str) -> Self
Mit dieser Funktion kann ein String in Großbuchstaben in das `Color` Enum umgewandelt werden.
Wichtig: Wenn der String nicht "BLUE", "YELLOW", "RED", "GREEN" übereinstimmt, schlägt die Methode fehl.

### Constants
``socha::game::constants``

In Constants können Konstanten vom Spiel gefunden werden.
Für Blokus sind ``BOARD_SIZE`` und ``BOARD_SIZE_I`` vorhanden. Beide sind ``20``, der Unterschied ist, dass ``BOARD_SIZE`` ``usize`` und ``BOARD_SIZE_I`` ``isize``.

### Coordinate
``socha::game::coordinate``

`Coordinate` enthält zum einen das Struct ``Coordinate``, welches überall wo mit Koordinaten gearbeitet wird verwendet wird.
Außerdem enthält es einige Funktionen, mit denen ein Vektor von ``Coordinate`` transformiert werden kann.

#### Coordinate

- x: isize
- y: isize

Das Struct `Coordinate` enthält die Funktionen ``add``, ``subtract``, ``multiply``, ``divide`` mit denen Koordinaten berechnet werden können.
Außerdem die Funktionen ``rotate`` und ``flip_on_vertical`` mit denen die Koordinate rotiert und gespiegelt werden können.

#### normalize_coordinates(coordinates: &Vec\<Coordinate\>) -> Vec\<Coordinate\>

Normalisiert die gegebenen Koordinaten, das heißt sie werden so verschoben, dass die Koordinate links-oben im Ursprung (0,0) ist.

#### rotate_coordinates(coordinates: Vec\<Coordinate\>, rotation: &Rotation) -> Vec\<Coordinate\>

Rotiert die Koordinaten nach der angegebenen Rotation.
Die Koordinaten werden danach nicht normalisiert, die Koordinate links-oben liegt danach also wahrscheinlich nicht im Ursprung.

#### flip_coordinates(coordinates: Vec\<Coordinate\>) -> Vec\<Coordinate\>

Spiegelt die gegebenen Koordinaten auf der y-Achse.
Die Koordinaten werden danach nicht normalisiert, die oberste-linke Koordinate liegt danach also wahrscheinlich nicht im Ursprung.

### Game Rule Logic

``socha::game::gamerulelogic``

Dieses Modul enthält Funktionen zum Berechnen und Überprüfen von Spielzügen.
Die Funktionen geben keine Aussetz-Züge zurück, außer bei der direkten Prüfung eines ``Move`` mit ``skip = true``.

#### get_possible_moves(gamestate: &GameState) -> Vec\<Move\>

Gibt alle möglichen Spielzüge für das aktuelle Team im angegebenen `GameState` zurück.
In der ersten Runde werden die möglichen Startzüge berechnet, in allen folgenden Runden die möglichen normalen Platzierungszüge.
Aussetz-Züge sind nicht enthalten.

#### get_possible_start_moves(gamestate: &GameState) -> Vec\<Move\>

Gibt alle möglichen Startzüge für das aktuelle Team zurück.
Dabei wird der Start-Spielstein in allen unterschiedlichen Rotationen und Spiegelungen an den Spielfeldrändern geprüft.

#### get_possible_set_moves(gamestate: &GameState) -> Vec\<Move\>

Gibt alle möglichen normalen Platzierungszüge für das aktuelle Team zurück.
Dafür werden alle noch verfügbaren Spielsteine und alle gültigen Eckfelder berücksichtigt.

#### get_possible_moves_for_piece(gamestate: &GameState, piece: &PieceType, valid_fields: &[Coordinate]) -> Vec\<Move\>

Gibt alle möglichen Platzierungszüge für den angegebenen Spielsteintyp zurück.
Die übergebenen ``valid_fields`` werden als mögliche Eckfelder verwendet.
Diese Funktion ist nur für Spielzüge nach der ersten Runde vorgesehen.

#### get_valid_fields(board: &Board, color: &Color) -> Vec\<Coordinate\>

Gibt alle freien Spielfelder zurück, die diagonal an einen Spielstein der angegebenen Farbe angrenzen.
Felder außerhalb des Spielfelds, belegte Felder und Felder mit direktem Kantenkontakt zu einem eigenen Spielstein werden ausgeschlossen.

#### get_colored_fields(board: &Board, color: &Color) -> Vec\<Coordinate\>

Gibt alle Koordinaten zurück, die auf dem Board mit der angegebenen Farbe belegt sind.

#### is_valid_move(gamestate: &GameState, m: &Move) -> bool

Prüft, ob ein Spielzug im angegebenen `GameState` gültig ist.
Ein Aussetz-Zug ist direkt gültig.
Bei einem Platzierungszug wird geprüft, ob der Spielstein noch verfügbar ist, alle Koordinaten innerhalb des Spielfelds liegen und keine belegten Felder verwendet werden.
Außerdem darf der neue Spielstein keinen direkten Kantenkontakt zu einem eigenen Spielstein haben.
Sobald das Team bereits einen Spielstein auf dem Board besitzt, muss mindestens ein Eckkontakt zu einem eigenen Spielstein vorhanden sein.

### Game State

``socha::game::gamestate::GameState``

Der `GameState` enthält alle Informationen zu einem Spielstand.

#### Felder

- `starting_piece: PieceType` - Der Startspielstein für dieses Spiel
- `is_starting_team_one: bool` - Gibt an, ob Team 1 (Blau/Rot) oder Team 2 (Gelb/Grün) startet
- `board: Board` - Das aktuelle Spielfeld
- `turn: u8` - Die aktuelle Zugnummer
- `round: u8` - Die aktuelle Rundennummer
- `current_turn_color: Color` - Die Farbe des Spielers, der am Zug ist
- `pieces: [Vec<PieceType>; 4]` - Die verfügbaren Spielsteine für jede Farbe (Blau, Gelb, Rot, Grün)

#### new(starting_piece, is_starting_team_one, board, turn, round, current_turn_color, blue_pieces, yellow_pieces, red_pieces, green_pieces) -> GameState

Erstellt einen neuen `GameState` mit den angegebenen Werten.

#### apply_move(&mut self, m: &Move, turn: u8) -> bool

Wendet einen Spielzug auf den `GameState` an und validiert ihn vorher.
Gibt ``true`` zurück, wenn der Zug gültig war und angewendet wurde, ansonsten ``false``.

#### apply_move_unchecked(&mut self, m: &Move, turn: u8)

Wendet einen Spielzug auf den `GameState` an, ohne ihn zu validieren.
Dies ist schneller, aber kann zu inkonsistenten Spielständen führen, wenn ein ungültiger Zug angewendet wird.

#### get_current_turn_color(&self) -> &Color

Gibt die Farbe des Spielers zurück, der am Zug ist.

#### set_current_turn_color(&mut self, color: Color)

Setzt die Farbe des Spielers, der am Zug ist.

#### get_turn(&self) -> &u8

Gibt die aktuelle Zugnummer zurück.

#### set_turn(&mut self, turn: u8)

Setzt die Zugnummer.

#### get_round(&self) -> &u8

Gibt die aktuelle Rundennummer zurück.

#### set_round(&mut self, round: u8)

Setzt die Rundennummer.

#### get_starting_piece(&self) -> &PieceType

Gibt den Startspielstein zurück.

#### set_starting_piece(&mut self, piece: PieceType)

Setzt den Startspielstein.

#### is_starting_team_one(&self) -> &bool

Gibt zurück, ob Team 1 startet.

#### set_is_starting_team_one(&mut self, is_starting_team_one: bool)

Setzt, ob Team 1 startet.

#### get_board(&self) -> &Board

Gibt eine Referenz auf das Spielfeld zurück.

#### set_board(&mut self, board: Board)

Setzt das Spielfeld.

#### get_color_pieces(&self, color: &Color) -> &[PieceType]

Gibt die verfügbaren Spielsteine für die angegebene Farbe zurück.

#### set_color_pieces(&mut self, color: &Color, pieces: Vec<PieceType>)

Setzt die verfügbaren Spielsteine für die angegebene Farbe.

### Move

``socha::game::move::Move``

Ein ``Move`` beschreibt einen Spielzug.
Die enthaltenen Felder sind öffentlich und können direkt ausgelesen werden.

#### color: Color

Gibt die Farbe des Spielers an, der den Zug ausführt.

#### piece: PieceType

Gibt den Typ des Spielsteins an, der platziert werden soll.

#### x: usize

Gibt die X-Koordinate der Position des Spielsteins an.

#### y: usize

Gibt die Y-Koordinate der Position des Spielsteins an.

#### is_flipped: bool

Gibt an, ob der Spielstein gespiegelt werden soll.

#### rotation: Rotation

Gibt die Rotation des Spielsteins an.

#### skip: bool

Gibt an, ob der Spieler in diesem Zug aussetzt.

#### new(color: Color, piece: PieceType, x: usize, y: usize, is_flipped: bool, rotation: Rotation, skip: bool) -> Move

Erstellt einen neuen Spielzug mit den angegebenen Werten.


### Piece

``socha::game::piece::Piece``

Ein ``Piece`` beschreibt einen Spielstein mit einem ``PieceType``, einer ``Rotation`` und der Information, ob er gespiegelt ist.

#### new(piece_type: PieceType, rotation: Rotation, is_flipped: bool) -> Piece

Erstellt einen neuen Spielstein mit dem angegebenen Typ, der Rotation und der Spiegelung.

#### get_coordinates() -> Vec\<Coordinate\>

Gibt die Koordinaten des Spielsteins nach Rotation und Spiegelung zurück.
Die Koordinaten werden anschließend normalisiert, sodass sie bei ``(0, 0)`` beginnen und nur positive Werte enthalten.

#### get_piece_type() -> &PieceType

Gibt eine Referenz auf den Typ des Spielsteins zurück.

#### set_piece_type(piece_type: PieceType)

Setzt den Typ des Spielsteins.

#### get_rotation() -> &Rotation

Gibt eine Referenz auf die Rotation des Spielsteins zurück.

#### set_rotation(rotation: Rotation)

Setzt die Rotation des Spielsteins.

#### is_flipped() -> &bool

Gibt eine Referenz darauf zurück, ob der Spielstein gespiegelt ist.

#### set_flipped(is_flipped: bool)

Setzt, ob der Spielstein gespiegelt werden soll.

### PieceType

``socha::game::piece::PieceType``

``PieceType`` beschreibt die Form eines Spielsteins.
Es stehen folgende Typen zur Verfügung:

``Mono``, ``Domino``, ``TrioL``, ``TrioI``, ``TetroO``, ``TetroT``, ``TetroI``, ``TetroL``, ``TetroZ``, ``PentoL``, ``PentoT``, ``PentoV``, ``PentoS``, ``PentoZ``, ``PentoI``, ``PentoP``, ``PentoW``, ``PentoU``, ``PentoR``, ``PentoX`` und ``PentoY``.

#### all_variants(filter: bool) -> Vec\<(Vec\<Coordinate\>, (Rotation, bool))\>

Gibt alle möglichen Varianten des Spielsteintyps zurück.
Jeder Eintrag enthält die Koordinaten sowie die zugehörige Rotation und Spiegelung.
Wenn ``filter`` auf ``true`` gesetzt wird, werden geometrisch identische Varianten nur einmal zurückgegeben.

#### base_coordinates() -> &'static [Coordinate]

Gibt die unveränderten Basis-Koordinaten des Spielsteintyps zurück.
Diese Koordinaten enthalten noch keine Rotation oder Spiegelung, beginnen bei ``(0, 0)`` und wachsen nur in positive Richtung.

#### to_string() -> String

Mit ``to_string`` kann ein Spielsteintyp in seinen Namen in Großbuchstaben umgewandelt werden, zum Beispiel ``"MONO"`` oder ``"PENTO_X"``.

#### parse::\<PieceType\>() -> Result\<PieceType, ParsePieceTypeError\>

Ein Spielsteintyp kann über das ``FromStr``-Trait aus einem String geparst werden, zum Beispiel mit ``"PENTO_X".parse::\<PieceType\>()``.
Unterstützt werden die Namen aus der Liste der Varianten in Großbuchstaben.
Wenn der String keinem gültigen Spielsteintyp entspricht, wird ein ``ParsePieceTypeError`` zurückgegeben.

### Rotation
``socha::game::rotation::Rotation``

`Rotation` ist ein Enum mit dem die Rotation eines Spielsteins angegeben wird.
Es stehen die vier Varianten ``None``, ``Right``, ``Mirror`` und ``Left`` zur Verfügung.

#### from_string(rotation_string: &str) -> Result\<Rotation, Box<dyn std::error::Error\>\>

Mit dieser Funktion kann ein String in Großbuchstaben in das `Rotation`-Enum umgewandelt werden.
Die gültigen Strings sind ``"NONE"``, ``"RIGHT"``, ``"MIRROR"`` und ``"LEFT"``.
Wenn der String keinem dieser Werte entspricht, wird ein Fehler zurückgegeben.

#### from_number(rotation_number: u8) -> Result\<Rotation, Box<dyn std::error::Error\>\>

Mit dieser Funktion kann eine Zahl in das `Rotation`-Enum umgewandelt werden.
Die Zuordnung ist ``0`` zu ``None``, ``1`` zu ``Right``, ``2`` zu ``Mirror`` und ``3`` zu ``Left``.
Wenn die Zahl keinem dieser Werte entspricht, wird ein Fehler zurückgegeben.

#### to_string() -> String

Mit ``to_string`` kann eine Rotation wieder in einen String umgewandelt werden.
Das Ergebnis ist immer der jeweilige String in Großbuchstaben: ``"NONE"``, ``"RIGHT"``, ``"MIRROR"`` oder ``"LEFT"``.

## Spiel spezifische Dateien im Library

- game/*
- connection/parser/*

## Nutzung und Anpassung

Für die Teilnahme an der Software-Challenge dürfen alle Dateien aus diesem Repository beliebig verwendet und verändert werden.

## Erwähnung

Bis zur crates.io Version 0.2.2 wurde die Bibliothek von [Simon Creates](https://github.com/simoncreates/socha) zur Verfügung gestellt.