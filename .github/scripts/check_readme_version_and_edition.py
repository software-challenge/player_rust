from __future__ import annotations

import re
import sys
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:  # pragma: no cover
    import tomli as tomllib

ROOT = Path(__file__).resolve().parents[2]
README_PATH = ROOT / "README.md"
CARGO_PATH = ROOT / "Cargo.toml"


def parse_cargo_metadata() -> tuple[str, str]:
    with CARGO_PATH.open("rb") as cargo_file:
        cargo_data = tomllib.load(cargo_file)

    package = cargo_data.get("package", {})
    rust_version = str(package.get("rust-version", "")).strip()
    edition = str(package.get("edition", "")).strip()

    if not rust_version or not edition:
        raise ValueError("Cargo.toml is missing the package rust-version or edition field.")

    return rust_version, edition


def parse_readme_metadata() -> tuple[str, str]:
    text = README_PATH.read_text(encoding="utf-8")
    section = re.search(
        r"(?ms)^## Eigenen Spieler erstellen\s*$\n(.*?)(?=^## |\Z)",
        text,
    )
    if section is None:
        raise ValueError("Could not find the 'Eigenen Spieler erstellen' section in README.md.")

    match = re.search(
        r"mindestens\s+Version\s+(?P<version>\d+\.\d+)\s+für\s+(?P<edition>\d{4})\s+Edition",
        section.group(1),
        re.IGNORECASE,
    )
    if match is None:
        raise ValueError(
            "Could not find the README Rust version/edition line in the 'Eigenen Spieler erstellen' section."
        )

    return match.group("version"), match.group("edition")


def main() -> int:
    try:
        cargo_rust_version, cargo_edition = parse_cargo_metadata()
    except (FileNotFoundError, ValueError, tomllib.TOMLDecodeError) as exc:
        print(f"Failed to read Cargo.toml: {exc}", file=sys.stderr)
        return 1

    try:
        readme_rust_version, readme_edition = parse_readme_metadata()
    except (FileNotFoundError, ValueError) as exc:
        print(str(exc), file=sys.stderr)
        return 1

    mismatches: list[str] = []
    for label, expected, actual in (
        ("rust-version", cargo_rust_version, readme_rust_version),
        ("edition", cargo_edition, readme_edition),
    ):
        if actual != expected:
            mismatches.append(
                f"README {label} is '{actual}' but Cargo.toml declares '{expected}'."
            )

    if mismatches:
        for message in mismatches:
            print(message, file=sys.stderr)
        return 1

    print(
        f"README rust-version and edition match Cargo.toml: rust-version={cargo_rust_version}, edition={cargo_edition}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
