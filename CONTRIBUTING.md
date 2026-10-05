# Entwickler Dokumentation

## Ordner Struktur

## Was muss für ein neues Spiel geändert werden?

## Ausführen

## Testen

## Veröffentlichen

Eine neue Version muss mit dem Workflow "Release" bei den Actions veröffentlicht werden.
Dafür musss die neue Version angegeben werden, der Workflow aktualisiert automatisch die Cargo Dateien.
Es werden dann verschiedene Tests ausgeführt um zu verifizieren, dass die neue Version funktioniert.
Es wird außerdem vorrausgesetzt, dass es in ``CHANGELOG.md`` einen Eintrag für die neue Version gibt, in dem tatsächlich Änderungen stehen. Sollte dies nicht der Fall sein kann nicht gepublished werden.
Wenn alle Vorraussetzungen erfüllt sind, pusht der Workflow die geupdateten Cargo Dateien in den main-Branch und veröffentlicht die neue Version auf Crates.io und Github.