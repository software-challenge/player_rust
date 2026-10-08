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
CONTRIBUTING_PATH = ROOT / "CONTRIBUTING.md"
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


def parse_file_metadata(path: Path, *names: str) -> dict[str, list[str]]:
    text = path.read_text(encoding="utf-8")
    metadata: dict[str, list[str]] = {}

    for name in names:
        marker = re.escape(name)
        matches = re.findall(
            rf"<!--\s*{marker}\s*-->(.*?)<!--\s*/{marker}\s*-->",
            text,
            re.DOTALL,
        )
        if not matches:
            raise ValueError(
                f"Expected at least one marker pair for '{name}' in '{path.name}', found none."
            )

        values = [value.strip() for value in matches]
        if any(not value for value in values):
            raise ValueError(f"HTML marker '{name}' in '{path.name}' is empty.")
        metadata[name] = values

    return metadata


def parse_document_metadata() -> tuple[list[str], list[str], list[str]]:
    readme_metadata = parse_file_metadata(README_PATH, "rust-version", "edition")
    contributing_metadata = parse_file_metadata(CONTRIBUTING_PATH, "rust-version")

    return (
        readme_metadata["rust-version"],
        readme_metadata["edition"],
        contributing_metadata["rust-version"],
    )


def main() -> int:
    try:
        cargo_rust_version, cargo_edition = parse_cargo_metadata()
    except (FileNotFoundError, ValueError, tomllib.TOMLDecodeError) as exc:
        print(f"Failed to read Cargo.toml: {exc}", file=sys.stderr)
        return 1

    try:
        readme_rust_versions, readme_editions, contributing_rust_versions = parse_document_metadata()
    except (FileNotFoundError, ValueError) as exc:
        print(str(exc), file=sys.stderr)
        return 1

    mismatches: list[str] = []
    for file_name, label, expected, actual_values in (
        ("README", "rust-version", cargo_rust_version, readme_rust_versions),
        ("README", "edition", cargo_edition, readme_editions),
        ("CONTRIBUTING.md", "rust-version", cargo_rust_version, contributing_rust_versions),
    ):
        for occurrence, actual in enumerate(actual_values, start=1):
            if actual != expected:
                mismatches.append(
                    f"{file_name} {label} marker #{occurrence} is '{actual}' "
                    f"but Cargo.toml declares '{expected}'."
                )

    if mismatches:
        for message in mismatches:
            print(message, file=sys.stderr)
        return 1

    print(
        f"README and CONTRIBUTING.md rust-version and README edition match Cargo.toml: "
        f"rust-version={cargo_rust_version}, edition={cargo_edition}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
