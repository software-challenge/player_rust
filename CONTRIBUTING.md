# Entwicklerdokumentation

## Commits

Commits sollten in folgendem Format geschrieben werden: `prefix: Kurze Beschreibung`.
Der Prefix wird vollständig kleingeschrieben. Die Beschreibung beginnt mit einem Großbuchstaben.

Wir verwenden folgende Prefixe. Wenn keiner passt, verwende den am besten passenden oder ergänze einen neuen Prefix im gleichen Format.

| **Prefix** | **Beschreibung** |
| :--- | :--- | 
| feat | Ergänzt eine neue Funktion oder Funktionalität. |
| fix | Behebt einen Fehler oder eine Warnung im Code. |
| refactor | Strukturiert oder verbessert den Code, ohne dessen Verhalten zu ändern. |
| perf | Verbessert die Performance des betroffenen Codes, ohne dessen Verhalten zu ändern. |
| test | Fügt Tests hinzu, ändert oder entfernt sie. |
| build | Ändert den Build-Prozess, Abhängigkeiten oder die CI-Konfiguration. |
| docs | Änderungen an der Readme, der Dokumentation oder ähnlichem. |
| chore | Umfasst sonstige Wartungsarbeiten und Änderungen, die keinem anderen Prefix zuzuordnen sind. |

## Ordnerstruktur

```
src/
├── lib.rs            # deklariert die Module, sonst nichts
├── client.rs         # das Client-Trait + die Spielschleife (spielunabhängig)
├── prelude.rs        # re-exportiert die öffentliche API für einfachen Import
├── result.rs         # das Resultat
│
├── connection/       # alles rund um die Kommunikation mit dem Server
│   ├── mod.rs
│   ├── handler.rs    # ConnectionHandler (TCP, Verbinden, Lesen, Senden)
│   └── parser/       # die Protokoll-Parser (spielunabhängig)
│       ├── mod.rs
│       ├── message.rs         # das Enum Message
│       ├── parser_strategy.rs # das Trait ParserStrategy
│       ├── parse_joined.rs    # verarbeitet die Join-/Welcome-Nachricht
│       └── parse_result.rs    # verarbeitet die Spiel-Ergebnisnachricht
│
├── common/           # gemeinsam genutzte, spielunabhängige Bausteine
│   ├── mod.rs
│   ├── direction.rs  # z. B. Rotation / Richtung
│   ├── coordinate.rs # z. B. Coordinate
│   └── ...           # alles, das von den meisten (aber nicht allen) Spielen wiederverwendet wird
│
├── games/            # Spielimplementierungen; nur das aktivierte Spiel wird gebaut
│   ├── mod.rs
│   └── <spielname>/  # alles, das für dieses Spiel spezifisch ist (Beispiel Blokus 27)
│       ├── mod.rs
│       ├── parser.rs # der konkrete ParserStrategy für dieses Spiel
│       ├── board.rs
│       ├── piece.rs
│       ├── color.rs
│       ├── team.rs
│       ├── move.rs
│       ├── gamestate.rs
│       ├── gamerulelogic.rs
│       └── constants.rs
│
└── util/             # entwicklerorientierte Werkzeuge, kein Teil des Spiels
    ├── mod.rs
    └── cmdl_args.rs  # Analyse der Befehlszeilenargumente
```

Unter `games/` können mehrere Spielimplementierungen liegen. Für den Release wird jeweils nur das aktivierte Spiel verwendet.

## Ein neues Spiel implementieren

Für die Implementierung eines neuen Spiels werden die Dateien im jeweiligen Unterordner von `games` angelegt oder angepasst (siehe [Ordnerstruktur](#ordnerstruktur)).

Die Datei `parser.rs` enthält den Parser, der die Kommunikation mit dem Server in Spieldaten umwandelt. Weitere Informationen dazu stehen im Abschnitt [Parser](#parser).

Die Dateien `board.rs`, `gamerulelogic.rs` und `gamestate.rs` enthalten zentrale Bestandteile der Spielimplementierung. Sie sollten bei der Entwicklung neuer Spiele nicht gelöscht werden, damit die API langfristig möglichst einheitlich bleibt.

Neue spielbezogene Typen und Funktionen, die Teil der öffentlichen API sein sollen, müssen außerdem in `prelude.rs` exportiert werden, damit sie über `socha::prelude::*` verfügbar sind.
Es sollte bei Anpassungen an dieser Datei überprüft werden, ob die Teilnehmer wie gewünscht Zugriff auf die API haben.

Für alle Anpassungen an ein neues Spiel sollten unbedingt passende Tests hinzugefügt werden.

### Parser

TBD

## Beispiele ausführen

Da es sich bei diesem Projekt um eine Bibliothek handelt, lässt es sich nicht direkt ausführen.
Im Ordner `examples` befinden sich Beispielimplementierungen von Spielern. Sie können mit `cargo run --example example_name` ausgeführt werden.

## Testen

Allgemeine Informationen zum Testen in Rust finden sich in der [Cargo-Dokumentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html).
Die folgenden Befehle sind für dieses Projekt besonders relevant, weil sie die Tests für alle Pakete im Workspace (`--workspace`) mit unterschiedlichen Feature-Konfigurationen ausführen. `--verbose` zeigt dabei detailliertere Ausgaben an.

- `cargo test --workspace --verbose` führt die Tests mit den Standard-Features aus.
- `cargo test --workspace --no-default-features --verbose` führt sie ohne optionale Features aus. Damit wird geprüft, ob das Projekt auch ohne diese Features funktioniert.
- `cargo test --workspace --all-features --verbose` aktiviert alle Features gleichzeitig. So werden auch Codepfade und mögliche Wechselwirkungen geprüft, die bei der Standardkonfiguration nicht aktiv sind.

## CI

Die CI validiert den Code mit folgenden Schritten:
- Sie führt die Tests mit den Standard-Features aus.
- Sie prüft mit [MSRV](https://crates.io/crates/cargo-msrv), ob die angegebene minimale Rust-Version korrekt ist.
  - Falls nicht, wird automatisch die tatsächlich erforderliche Mindestversion ermittelt. Diese ist dem fehlgeschlagenen Workflow zu entnehmen.
- Sie prüft, ob die in der README angegebene minimale Rust-Version, die Rust-Edition und der Beispielcode mit den Angaben in `Cargo.toml` und den Beispielen übereinstimmen.
- Sie führt [Clippy](https://doc.rust-lang.org/clippy/) aus, ein Werkzeug zur statischen Analyse von Rust-Code. Es weist unter anderem auf häufige Fehler und Verbesserungsmöglichkeiten hin. In der CI werden Warnungen als Fehler behandelt.

## Veröffentlichen

Eine neue Version wird über den Workflow "Release" in den GitHub Actions veröffentlicht.

### Schritte

1. Die neue Version muss im Release-Workflow angegeben werden.
2. Der Workflow aktualisiert automatisch die Cargo-Dateien.
3. Verschiedene Tests werden ausgeführt, um sicherzustellen, dass die neue Version korrekt funktioniert.
4. In ``CHANGELOG.md`` muss ein Eintrag für die neue Version vorhanden sein, der die Änderungen der Version beschreibt. Fehlt dieser Eintrag, kann die Veröffentlichung nicht erfolgen.
5. Wenn alle Voraussetzungen erfüllt sind, pusht der Workflow die aktualisierten Cargo-Dateien in den ``main``-Branch und veröffentlicht die neue Version auf Crates.io und GitHub.

### Versionierung

Das Versionsformat lautet:

saison.breaking-changes.fixes-features-and-co

Beispiel für `27.1.2`:
- `27`: Saison 2026/27
- `1`: Es wurde eine Änderung an der Schüler-API vorgenommen
- `2`: Nach dieser Änderung wurden zwei Versionen mit normalen Änderungen veröffentlicht