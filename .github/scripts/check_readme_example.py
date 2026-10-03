from difflib import unified_diff
from pathlib import Path
import re
import sys


ROOT = Path(__file__).resolve().parents[2]
README_PATH = ROOT / "README.md"
EXAMPLE_PATH = ROOT / "examples" / "basic_player" / "main.rs"


def main() -> int:
    readme = README_PATH.read_text(encoding="utf-8")

    section = re.search(r"(?ms)^## Eigenen Spieler erstellen\s*$([\s\S]*?)(?=^## |\Z)", readme)
    if section is None:
        print("Could not find the 'Eigenen Spieler erstellen' section in README.md", file=sys.stderr)
        return 1

    code_block = re.search(r"(?ms)^```rust[ \t]*\r?\n([\s\S]*?)^```\s*$", section.group(1))
    if code_block is None:
        print("Could not find a Rust code block in the README player section", file=sys.stderr)
        return 1

    readme_example = code_block.group(1).splitlines()
    source_example = EXAMPLE_PATH.read_text(encoding="utf-8").splitlines()
    
    if readme_example != source_example:
        diff = unified_diff(
            source_example,
            readme_example,
            fromfile="examples/basic_player/main.rs",
            tofile="README.md Rust example",
            lineterm="",
        )
        print("README Rust example differs from examples/basic_player/main.rs:", file=sys.stderr)
        print("\n".join(diff), file=sys.stderr)
        return 1

    print("README Rust example matches examples/basic_player/main.rs")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())