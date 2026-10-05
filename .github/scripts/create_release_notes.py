import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CHANGELOG_PATH = ROOT / "CHANGELOG.md"


def main() -> int:
    if len(sys.argv) != 2:
        print("Usage: create_release_notes.py VERSION", file=sys.stderr)
        return 2

    version = sys.argv[1]
    changelog = CHANGELOG_PATH.read_text(encoding="utf-8")
    sections = list(re.finditer(r"(?m)^## \[([^\]]+)\]\s*$", changelog))
    matches = [section for section in sections if section.group(1) == version]
    if len(matches) != 1:
        print(f"Expected exactly one CHANGELOG.md section for version {version}.", file=sys.stderr)
        return 1

    section = matches[0]
    next_section = next((item for item in sections if item.start() > section.start()), None)
    notes = changelog[section.end():next_section.start() if next_section else None].strip()
    entries = [
        line for line in notes.splitlines()
        if line.strip() and not line.lstrip().startswith("#")
    ]
    if not entries:
        print(f"CHANGELOG.md section for version {version} has no release notes.", file=sys.stderr)
        return 1

    release_notes = f"[Crate](https://crates.io/crates/socha/{version})\n\n{notes}\n"
    (ROOT / "release-notes.md").write_text(release_notes, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
