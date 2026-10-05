# Entwickler Dokumentation

## Ordner Struktur

## Was muss für ein neues Spiel geändert werden?

## Ausführen

## Testen

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