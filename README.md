# Software Challenge 2026/27 Rust Client

## Allgemein

Das ist die offiziele Rust-Bibliothek für die Programmierung von Spielern für die [Software Challenge Germany](https://software-challenge.de/) auf [crates.io](https://crates.io/crates/socha).

## Features

- Vollständige Kommunikation mit Spielservern
- Vollständige Berechnung von Spielzügen
- Simulation von Spielzügen auf Gamestates (Beispielsweise für den Minimax Algorithmus vorrausgesetzt)
- Simulation vom letzten Zug statt auslesen des gesamten neuen Gamestates für bessere Performance
- Viele Funktionen die die Entwicklung erleichtern
- Kapselung auf alle größeren Datentypen
    - Der Rustcompiler inlined solche Funktionen, die Performance ist also exakt gleich wie wenn man direkt Zugriff auf die Variablen hat

## Fehler und fehlende Features

Eröffnet bei Fehlern oder euer Meinung nach fehlenden Funktionen gerne ein Issue und/oder schreibt es in den Discord Probleme Kanal.
Außerdem könnt ihr euch an:
- SturmEnte
- NichtNil5
auf Discord wenden.

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
```rust
fn main() {
    println!("Hello, world!");
}
```
Diesen Code könnt ihr einfach löschen und dann folgenden Code einfügen:
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

Dieser Code erstellt und startet beim Start des Programms einen neuen Spieler.
Dieser verbindet sich dann mit dem offenem Spiel auf ``localhost:13050``.
Das ist die Standart Adresse und Port, wenn keine Commandlinearguments gegeben sind. 
Diese Adresse und Port wird von dem GUI verwendet, wenn ein Spiel mit der Option "Eigener Computerspieler, manuell gestartet" gestartet wird.
Sobald das Spiel im GUI gestartet wurde schickt der Spieler dann immer den ersten Spielzug aus der Liste der gültigen Spielzüge an den Server.

Erstellt nun also im GUI ein neues Spiel, ein Spieler sollte entweder "Mensch", "Zufalls-Computerspieler" oder "Fortgeschrittener-Computerspieler" sein, die andere "Eigener Computerspieler, manuell gestartet".

Nun könnt ihr den Spieler aus dem Hauptordner mit folgendem Befehl starten: ``cargo run``. 
Der Hauptordner ist bei ``cargo init`` der Ordner, in dem der Befehl ausgeführt wurde, bei ``cargo new projekt_name`` der Ordner, der von Cargo erstellt wurde.

## Schnittstelle

Es stehen euch verschiedene Methoden und Datenstrukturen zur Verfügung. Hier werden die wichtigsten einmal erklärt.
Grundsätzlich sollte in der Regel nur alles im ``socha::game`` für euch relevant sein.

### Client
``socha::client``

Der Client ist für den Gameflow und das Übermitteln von Spielereignissen an euer Programm zustädnig.
Mit dem Client-trait müsst ihr ein eigenes Struct mit den entsprechenden Funktionen erstellen.
Euren Client könnt ihr dann an eine der Startfunktionen übermitteln.
Wie euer Client aussehen kann könnt ihr [hier](#wie-wird-das-library-verwendet) sehen.

### Board
``socha::game::board::Board``

Das Board ist das Spielfeld.
Es wird im [Gamestate](#game-state) verwendet um das aktuelle Spielfeld zu speicher.
Es stehen hier außerdem einige Methoden zu verfügung.

#### print_board

Hiermit kann das Board in die Konsole ausgegeben werden.

#### get_cell(x: usize, y: usize) -> Option\<Team\>

Hiermit kann eine Zelle mit den gegebenen Koordinaten vom Board ausgelesen werden.

#### set_cell(x: usize, y: usize, team: Team) -> bool

Hiermit kann eine Zelle auf das angegebene Team gesetzt werden.
Wenn die Zelle auf dem Spielfeld ist und somit die Zelle auf das Team gesetzt wurde, wird ``true`` zurückgegeben, ansonsten wird false zurückgegen.

#### place_piece(x: uszize, y: usize. team: Team, piece: Piece) -> bool

Hiermit kann ein Spielstein auf dem Spielfeld plaziert werden.
Der Spielstein wird nur platziert, wenn alle Koordinaten des Spielsteins noch nicht belegt und innehalb vom Spielfeld sind.
Wenn der Spielstein erfolgreich platziert wurde, gibt die Funktion true zurück, ansonsten false.
Diese Überprüfung verbraucht etwas mehr Leistung als die unchecked Variante, wenn ihr euch sicher seid, dass der Spielstein dort platziert werden darf, nutzt die unchecked variante.

#### place_piece_unchecked(x: uszize, y: usize. team: Team, piece: Piece)

Diese Funktion tut das gleiche wie die place_piece Funktion, nur dass keine Überprüfung stattfindet, ob der Spielstein patziert werden kann.

### Color
``socha::game::color::Color``

Color ist ein enum mit allen 4 Farben aus dem Spiel.
Es wird beispielsweise im Game State genutzt um anzugeben welche Farbe einen Zug tätigen soll.
Außerdem wird Color im Board verwendet um die Spielfelder mit der belegten Farbe zu makieren.

#### from_string(s: &str) -> Self
Mit dieser Funktion kann ein String in Großbuchstaben in das Color Enum umgewandelt werden.
Wichtig: Wenn der String nicht "BLUE", "YELLOW", "RED", "GREEN" übereinstimmt, panict der Code.

### Constants
``socha::game::constants``

In Constants können Konstanten vom Spiel gefunden werden.
Für Blokus sind ``BOARD_SIZE`` und ``BOARD_SIZE_I`` vorhanden. Beide sind ``20``, der Unterschied ist, dass ``BOARD_SIZE`` ``usize`` und ``BOARD_SIZE_I`` ``isize``.

### Coordinate
``socha::game::coordinate``

Coordinate enthält zum einen das Struct ``Coordinate`` welches überall wo mit Coordinaten gearbeitet wird verwendet wird.
Außerdem enthält es einige Funktionen mit den ein Vektor von ``Coordinate`` transformiert werden kann.

#### Coordinate

- x: isize
- y: isize

Das Struct Coordinate enthält die Funktionen ``add``, ``subtract``, ``multiply``, ``diivde`` mit denen Coordinaten verechnet werden können.
Außerdem di Funktionen ``rotate`` und ``flip_on_vertical`` mit den die Coordinate rotiert und gespiegelt werden können.

#### normalize_coordinates(coordinates: &Vec\<Coordinate\>) -> Vec\<Coordinate\>

Normalisiert die gegebenen Koordinaten, bedeuetet es verschiebt sie so dass die oberste-linkste Koordinate im Usprung (0,0) ist.

#### rotate_coordinates(coordinates: Vec\<Coordinate\>, rotation: &Rotation) -> Vec\<Coordinate\>

Rotiert die Koordinaten nach der angegebenen Rotation.
Die Koordinaten werden dannach nicht normalisiert, die oberste-linkste Koordinate liegt dannach also wahrscheinlich nicht im Urpsrung.

#### flip_coordinates(coordinates: Vec\<Coordinate\>) -> Vec\<Coordinate\>

Spiegel die gegebenen Koordinaten auf der Ordinate.
Die Koordinaten werden dannach nicht normalisiert, die oberste-linkste Koordinate liegt dannach also wahrscheinlich nicht im Urpsrung.

### Game Rule Logic
``socha::game::gamerulelogic``

Dieses Modul enthält Funktionen zum Berechnen und Überprüfen von Spielzügen.
Die Funktionen geben keine Aussetz-Züge zurück, außer bei der direkten Prüfung eines ``Move`` mit ``skip = true``.

#### get_possible_moves(gamestate: &GameState) -> Vec\<Move\>

Gibt alle möglichen Spielzüge für das aktuelle Team im angegebenen Game State zurück.
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

#### get_colored_fiels(board: &Board, color: &Color) -> Vec\<Coordinate\>

Gibt alle Koordinaten zurück, die auf dem Board mit der angegebenen Farbe belegt sind.
Der Name der Funktion ist ``fiels`` und entspricht der aktuellen API.

#### is_valid_move(gamestate: &GameState, m: &Move) -> bool

Prüft, ob ein Spielzug im angegebenen Game State gültig ist.
Ein Aussetz-Zug ist direkt gültig.
Bei einem Platzierungszug wird geprüft, ob der Spielstein noch verfügbar ist, alle Koordinaten innerhalb des Spielfelds liegen und keine belegten Felder verwendet werden.
Außerdem darf der neue Spielstein keinen direkten Kantenkontakt zu einem eigenen Spielstein haben.
Sobald das Team bereits einen Spielstein auf dem Board besitzt, muss mindestens ein Eckkontakt zu einem eigenen Spielstein vorhanden sein.

### Game State
``socha::game::gamestate::GameState``

Der Game State enthält alle Informationen zu einem Spielstand.
!!! error Unfertig

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

Ein Spielsteintyp kann über das ``FromStr``-Trait aus einem String geparst werden, zum Beispiel mit ``"PENTO_X".parse::\<PieceType\>()``. Unterstützt werden die Namen aus der Liste der Varianten in Großbuchstaben.
Wenn der String keinem gültigen Spielsteintyp entspricht, wird ein ``ParsePieceTypeError`` zurückgegeben.

### Rotation
``socha::game::rotation::Rotation``

Rotation ist ein Enum mit dem die Rotation eines Spielsteins angegeben wird.
Es stehen die vier Varianten ``None``, ``Right``, ``Mirror`` und ``Left`` zur Verfügung.

#### from_string(rotation_string: &str) -> Result\<Rotation, Box<dyn std::error::Error\>\>

Mit dieser Funktion kann ein String in Großbuchstaben in das Rotation-Enum umgewandelt werden.
Die gültigen Strings sind ``"NONE"``, ``"RIGHT"``, ``"MIRROR"`` und ``"LEFT"``.
Wenn der String keinem dieser Werte entspricht, wird ein Fehler zurückgegeben.

#### from_number(rotation_number: u8) -> Result\<Rotation, Box<dyn std::error::Error\>\>

Mit dieser Funktion kann eine Zahl in das Rotation-Enum umgewandelt werden.
Die Zuordnung ist ``0`` zu ``None``, ``1`` zu ``Right``, ``2`` zu ``Mirror`` und ``3`` zu ``Left``.
Wenn die Zahl keinem dieser Werte entspricht, wird ein Fehler zurückgegeben.

#### to_string() -> String

Mit ``to_string`` kann eine Rotation wieder in einen String umgewandelt werden.
Das Ergebnis ist immer der jeweilige String in Großbuchstaben: ``"NONE"``, ``"RIGHT"``, ``"MIRROR"`` oder ``"LEFT"``.

## Spiel spezifische Dateien im Library

- game/*
- connection/parser/*

## Nutzung und Anpassung

Für die Teilnahme an der Software Challenge dürfen alle Dateien aus diesem Repository beliebig verwendet und verändert werden.

## Erwähnung

Bis zur crates.io Version 0.2.2 wurde die Bibliothek von [Simon Creates](https://github.com/simoncreates/socha) zur Verfügung gestellt.