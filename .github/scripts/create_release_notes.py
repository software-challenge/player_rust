import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CHANGELOG_PATH = ROOT / "CHANGELOG.md"


def main() -> int:
    if len(sys.argv) not in (2, 3):
        print("Usage: create_release_notes.py CHANGELOG_VERSION [CRATE_VERSION]", file=sys.stderr)
        return 2

    changelog_version = sys.argv[1]
    crate_version = sys.argv[2] if len(sys.argv) == 3 else changelog_version
    changelog = CHANGELOG_PATH.read_text(encoding="utf-8")
    sections = list(re.finditer(r"(?m)^## \[([^\]]+)\]\s*$", changelog))
    matches = [section for section in sections if section.group(1) == changelog_version]
    if len(matches) != 1:
        print(f"Expected exactly one CHANGELOG.md section for version {changelog_version}.", file=sys.stderr)
        return 1

    section = matches[0]
    next_section = next((item for item in sections if item.start() > section.start()), None)
    notes = changelog[section.end():next_section.start() if next_section else None].strip()
    entries = [
        line for line in notes.splitlines()
        if line.strip() and not line.lstrip().startswith("#")
    ]
    if not entries:
        print(f"CHANGELOG.md section for version {changelog_version} has no release notes.", file=sys.stderr)
        return 1

    release_notes = f"[Crate](https://crates.io/crates/socha/{crate_version})\n\n{notes}\n"
    (ROOT / "release-notes.md").write_text(release_notes, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
