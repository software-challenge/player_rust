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
│       ├── parser.rs* # der konkrete ParserStrategy für dieses Spiel
│       ├── board.rs
│       ├── piece.rs
│       ├── color.rs
│       ├── team.rs
│       ├── move.rs
│       ├── gamestate.rs*
│       ├── gamerulelogic.rs*
│       └── constants.rs
│
└── util/             # entwicklerorientierte Werkzeuge, kein Teil des Spiels
    ├── mod.rs
    └── cmdl_args.rs  # Analyse der Befehlszeilenargumente
```
*muss in jedem Spiel vorhanden sein

Unter `games/` können mehrere Spielimplementierungen liegen. Für den Release wird jeweils nur das aktivierte Spiel verwendet.

## Ein neues Spiel implementieren

Für die Implementierung eines neuen Spiels werden die Dateien im jeweiligen Unterordner von `games` angelegt (siehe [Ordnerstruktur](#ordnerstruktur)).

Die Datei `parser.rs` enthält den Parser, der die Kommunikation mit dem Server in Spieldaten umwandelt. Weitere Informationen dazu stehen im Abschnitt [Parser](#parser).

Neue spielbezogene Typen und Funktionen, die Teil der öffentlichen API sein sollen, müssen außerdem in `prelude.rs` exportiert werden, damit sie über `socha::prelude::*` verfügbar sind.
Es sollte bei Anpassungen an dieser Datei überprüft werden, ob die Teilnehmer wie gewünscht Zugriff auf die API haben.

Für alle Anpassungen an ein neues Spiel sollten unbedingt passende Tests hinzugefügt werden.

### Parser

TBD

## Programmierstil

### Datenkapselung

Bei neuen Structs sollten Felder grundsätzlich privat bleiben und Zugriffe über eine passende öffentliche API erfolgen. So können wir Prüfungen und Regeln auch dann in den Methoden ergänzen, wenn sie heute noch nicht erforderlich sind. Die API-Nutzung in bestehenden Projekten muss dadurch voraussichtlich nicht stark verändert werden. Öffentliche Felder sollten eine bewusste Ausnahme bleiben, etwa bei einfachen Werttypen, bei denen direkter Zugriff Teil der gewünschten API ist.

Beispiele aus Blokus 2027:

- `Board` verbirgt sein Spielfeld und bietet Methoden zum Lesen und Ändern an.
- `GameState` und `Piece` haben private Felder, stellen aber auch Setter bereit, die Werte derzeit ungeprüft ändern. Die Methoden bieten einen zentralen Ansatzpunkt, falls solche Änderungen künftig geprüft werden müssen.
- `Coordinate` und `Move` haben öffentliche Felder und erlauben damit direkten Feldzugriff. Bei `Coordinate` ist das für einen einfachen Werttyp eine mögliche bewusste Ausnahme.

## Beispiele ausführen

Da es sich bei diesem Projekt um eine Bibliothek handelt, lässt es sich nicht direkt ausführen.
Im Ordner `examples` befinden sich Beispielimplementierungen von Spielern. Sie können mit `cargo run --example example_name` ausgeführt werden.

## Testen

Allgemeine Informationen zum Testen in Rust finden sich in der [Cargo-Dokumentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html).
Der CI-Workflow führt folgenden Testbefehl aus. Er testet alle Pakete im Workspace (`--workspace`) mit den Standard-Features; `--verbose` zeigt detailliertere Ausgaben an.

```sh
cargo test --workspace --verbose
```

## Was wird zum Entwickeln benötigt?

Zum entwickeln dieser Bibliothek wird nur Rust und Cargo (wird automatisch mit installiert) benötigt.
Es wird mindestens die Rust-Version <!-- rust-version -->1.88<!-- /rust-version --> benötigt, alle Versionen darüber, die zur <!-- edition -->2024<!-- /edition --> Edition gehören, sollten auch funktionieren.
Ob die Rust-Version ausreicht wird durch den [MSRV CI Job](#ci) bei jeder Pull Request und jeden Commit auf main überprüft.

## Workflows

Die Versionen von Ubuntu und Java sind fest gesetzt, damit neue Versionen nicht unvorhergesehen bestehende Workflows beschädigen.
Das heißt aber auch: Es sollte regelmäßig geprüft werden, ob diese Versionen aktualisiert werden müssen.

### CI

Die CI prüft den Code mit den folgenden Schritten:
- Sie führt die Tests mit den Standard-Features aus.
- Sie prüft mit [MSRV](https://crates.io/crates/cargo-msrv), ob die angegebene minimale Rust-Version korrekt ist.
  - Wenn nicht, wird die tatsächlich erforderliche Mindestversion automatisch ermittelt. Diese ist dem fehlgeschlagenen Workflow zu entnehmen.
- Sie führt [Clippy](https://doc.rust-lang.org/clippy/) aus, eine Code Analyse für Rust. In der CI werden Warnungen wie Fehler behandelt.
- Sie lässt alle Beispiele gegen den Java Random Player spielen.
- Sie prüft, ob die in `README.md` und `CONTRIBUTING.md` angegebene minimale Rust-Version, die Rust-Edition und die Beispielcodes zu den Angaben in `Cargo.toml` und den Beispielen passen.
- Sie prüft bei Pull Requests auf `main`, ob das Changelog und die Version in `Cargo.toml` angepasst wurden.

#### Example Integration Test

Dieser Task lässt alle Examples der Rust-Bibliothek gegen den Java Random Player spielen, um zu prüfen, ob sie funktionieren und ob mögliche Fehler erkannt werden, die von den normalen Tests nicht gefunden werden.

Dieser Task hatte in der Vergangenheit beim Herunterladen des [Java-Servers und Random Players](https://github.com/software-challenge/backend/releases) öfter Probleme.
Das wurde verbessert und in Download und eigentliche Ausführung aufgeteilt, damit besser angezeigt werden kann, wo genau ein Fehler aufgetreten ist.

Wenn beim Download ein 4xx-HTTP-Status auftritt, wird der Download-Task als fehlgeschlagen markiert und der Example Integration Test übersprungen. Wenn ein anderer Status als ein 2xx-HTTP-Status auftritt, wird der Example Integration Test ebenfalls übersprungen.

## Veröffentlichen

Eine neue Version wird über den Workflow "Release" in GitHub Actions veröffentlicht.
Dabei läuft die CI durch. Außerdem wird geprüft, ob die Versionsnummer in `Cargo.toml` aktualisiert wurde und ob ein passender Eintrag im Changelog vorhanden ist.

## Nightly-Veröffentlichung

Wenn es Änderungen auf `main` gibt, die noch nicht in einer vollständigen Release-Version oder einer Nightly-Version enthalten sind, wird automatisch um 4 Uhr eine Nightly-Version veröffentlicht.

## Contributing

Wir arbeiten mit Feature-Branches.

Wenn etwas verändert oder ergänzt werden soll, wird ein neuer Branch angelegt, der sich an die Namenskonvention der [Commits](#commits) anlehnt.

Ein Feature-Branch sollte möglichst nur Änderungen enthalten, die sich aus dem Namen des Branches ableiten lassen. Für kleine Änderungen kann es sinnvoll sein, diese zusammen mit anderen Änderungen im selben Branch zu machen, damit nicht unnötig viel Aufwand entsteht.

In einem Feature-Branch muss die Versionsnummer in `Cargo.toml` nach den Regeln der [Versionierung](#versionierung) angepasst werden. Zusätzlich muss ein Eintrag für diese Version im `Changelog.md` erstellt werden.

Das sorgt dafür, dass manuelle Veröffentlichungen und Nightly-Releases sauber funktionieren.

Für Feature-Branches wird automatisch ein Draft-PR erstellt. In diesem PR steht eine Checkliste, die geprüft werden sollte, auch wenn kein automatischer Draft-PR verwendet wird:
- [ ] Versionsnummer in `Cargo.toml` aktualisiert?
- [ ] Changelog-Eintrag für die Änderungen vorhanden?
- [ ] Versionsnummer früherer PRs für das nächste Release aktualisiert?
- [ ] Zusammenfassung, was der PR verändert?

## Versionierung

Das Versionsformat lautet:

`saison.breaking-changes.fixes-features-and-co`

Beispiel für `27.1.2`:
- `27`: Saison 2026/27
- `1`: Es wurde eine Änderung an der Schüler-API vorgenommen.
- `2`: Nach dieser Änderung wurden zwei Versionen mit normalen Änderungen veröffentlicht.