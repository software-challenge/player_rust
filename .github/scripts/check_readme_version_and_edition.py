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


def parse_readme_metadata() -> tuple[list[str], list[str]]:
    text = README_PATH.read_text(encoding="utf-8")

    def extract_values(name: str) -> list[str]:
        marker = re.escape(name)
        matches = re.findall(
            rf"<!--\s*{marker}\s*-->(.*?)<!--\s*/{marker}\s*-->",
            text,
            re.DOTALL,
        )
        if not matches:
            raise ValueError(
                f"Expected at least one README HTML marker pair for '{name}', found none."
            )

        values = [value.strip() for value in matches]
        if any(not value for value in values):
            raise ValueError(f"README HTML marker for '{name}' is empty.")
        return values

    return extract_values("rust-version"), extract_values("edition")


def main() -> int:
    try:
        cargo_rust_version, cargo_edition = parse_cargo_metadata()
    except (FileNotFoundError, ValueError, tomllib.TOMLDecodeError) as exc:
        print(f"Failed to read Cargo.toml: {exc}", file=sys.stderr)
        return 1

    try:
        readme_rust_versions, readme_editions = parse_readme_metadata()
    except (FileNotFoundError, ValueError) as exc:
        print(str(exc), file=sys.stderr)
        return 1

    mismatches: list[str] = []
    for label, expected, actual_values in (
        ("rust-version", cargo_rust_version, readme_rust_versions),
        ("edition", cargo_edition, readme_editions),
    ):
        for occurrence, actual in enumerate(actual_values, start=1):
            if actual != expected:
                mismatches.append(
                    f"README {label} marker #{occurrence} is '{actual}' "
                    f"but Cargo.toml declares '{expected}'."
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
