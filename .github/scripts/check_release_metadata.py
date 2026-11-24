import argparse
import os
import re
import subprocess
import urllib.error
import urllib.request
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CARGO_PATH = ROOT / "Cargo.toml"
CHANGELOG_PATH = ROOT / "CHANGELOG.md"


def parse_package_version() -> str:
    text = CARGO_PATH.read_text(encoding="utf-8")
    match = re.search(r"(?ms)^\[package\]\s*\n(.*?)^(?=\[|\Z)", text)
    if match is None:
        raise SystemExit("Could not find the [package] block in Cargo.toml.")

    version_match = re.search(r'(?m)^version\s*=\s*"([^"]+)"', match.group(1))
    if version_match is None:
        raise SystemExit("Could not find the package version in Cargo.toml.")
    return version_match.group(1)


def latest_full_release() -> str | None:
    tag_output = subprocess.run(
        ["git", "tag", "--list", "v*", "--sort=-version:refname"],
        check=True,
        capture_output=True,
        text=True,
    ).stdout
    for tag in tag_output.splitlines():
        if re.fullmatch(r"v[0-9]+\.[0-9]+\.[0-9]+", tag):
            return tag[1:]
    return None


def parse_semver(version: str) -> tuple[int, int, int]:
    if not re.fullmatch(r"[0-9]+\.[0-9]+\.[0-9]+", version):
        raise SystemExit(f"Unexpected full release version: {version}")
    return tuple(map(int, version.split(".")))


def get_changelog_notes(version: str) -> str:
    changelog = CHANGELOG_PATH.read_text(encoding="utf-8")
    sections = list(re.finditer(r"(?m)^## \[([^\]]+)\]\s*$", changelog))
    matches = [section for section in sections if section.group(1) == version]
    if len(matches) != 1:
        raise SystemExit(f"Expected exactly one CHANGELOG.md section for version {version}; found {len(matches)}.")

    section = matches[0]
    next_section = next((item for item in sections if item.start() > section.start()), None)
    notes = changelog[section.end():next_section.start() if next_section else None].strip()
    entries = [line for line in notes.splitlines() if line.strip() and not line.lstrip().startswith("#")]
    if not entries:
        raise SystemExit(f"CHANGELOG.md section for version {version} has no release notes.")
    return notes


def write_release_notes(section_version: str, crate_version: str) -> None:
    notes = get_changelog_notes(section_version)
    release_notes = f"[Crate](https://crates.io/crates/socha/{crate_version})\n\n{notes}\n"
    (ROOT / "release-notes.md").write_text(release_notes, encoding="utf-8")


def check_crate_unused(version: str, kind: str) -> None:
    crate_url = f"https://crates.io/api/v1/crates/socha/{version}"
    request = urllib.request.Request(crate_url, headers={"User-Agent": f"player_rust-{kind}"})
    try:
        urllib.request.urlopen(request, timeout=20)
    except urllib.error.HTTPError as error:
        if error.code != 404:
            raise SystemExit(f"Could not check crates.io (HTTP {error.code}).")
    except Exception as error:
        raise SystemExit(f"Could not check crates.io: {error}")
    else:
        raise SystemExit(f"socha {version} is already published on crates.io.")


def check_github_tag_unused(tag: str) -> None:
    repository = os.environ["GITHUB_REPOSITORY"]
    token = os.environ["GH_TOKEN"]
    tag_url = f"https://api.github.com/repos/{repository}/git/ref/tags/{tag}"
    request = urllib.request.Request(
        tag_url,
        headers={
            "Accept": "application/vnd.github+json",
            "Authorization": f"Bearer {token}",
            "X-GitHub-Api-Version": "2022-11-28",
        },
    )
    try:
        urllib.request.urlopen(request, timeout=20)
    except urllib.error.HTTPError as error:
        if error.code != 404:
            raise SystemExit(f"Could not check GitHub tag (HTTP {error.code}).")
    except Exception as error:
        raise SystemExit(f"Could not check GitHub tag: {error}")
    else:
        raise SystemExit(f"GitHub tag {tag} already exists.")


def require_bumped(version: str, latest: str | None) -> None:
    if latest is not None and parse_semver(version) <= parse_semver(latest):
        raise SystemExit(f"Cargo.toml version {version} is not newer than the newest full release {latest}.")


def require_changelog_changed() -> None:
    base_name = os.environ.get("GITHUB_BASE_REF", "main")
    candidates = (
        f"refs/remotes/origin/{base_name}",
        f"origin/{base_name}",
        base_name,
    )

    base = next(
        (
            candidate
            for candidate in candidates
            if subprocess.run(
                ["git", "rev-parse", "--verify", "--quiet", candidate],
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
            ).returncode == 0
        ),
        None,
    )
    if base is None:
        raise SystemExit(f"Could not find the pull request base branch {base_name!r}.")

    result = subprocess.run(
        ["git", "diff", "--quiet", base, "HEAD", "--", "CHANGELOG.md"],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.PIPE,
        text=True,
    )
    if result.returncode == 0:
        raise SystemExit(f"CHANGELOG.md has no changes compared to {base_name}.")
    if result.returncode != 1:
        raise SystemExit(result.stderr.strip() or "Could not compare CHANGELOG.md with the pull request base branch.")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices=("pull-request", "release", "nightly"))
    parser.add_argument("--nightly-date")
    parser.add_argument("--check-unused", action="store_true")
    args = parser.parse_args()

    subprocess.run(["git", "fetch", "--tags", "--force"], check=True)
    version = parse_package_version()
    latest = latest_full_release()

    if args.mode == "nightly":
        if not re.fullmatch(r"[0-9]+\.[0-9]+\.[0-9]+", version):
            raise SystemExit(f"Unexpected package version format: {version}")
        base = parse_semver(version)
        if latest is not None and base <= parse_semver(latest):
            raise SystemExit(
                f"Cargo.toml version {version} is not newer than the newest full release {latest}. "
                "Nightly publication is not allowed until the version is bumped."
            )
        if not args.nightly_date or not re.fullmatch(r"[0-9]{8}", args.nightly_date):
            raise SystemExit("A valid --nightly-date is required in nightly mode.")
        nightly_version = f"{version}-nightly.{args.nightly_date}"
        write_release_notes(version, nightly_version)
        print(nightly_version)
        if args.check_unused:
            check_crate_unused(nightly_version, "nightly")
            check_github_tag_unused(f"nightly-{args.nightly_date}")
        return 0

    parse_semver(version)
    require_bumped(version, latest)

    if args.mode == "release":
        write_release_notes(version, version)
        print(version)
        if args.check_unused:
            check_crate_unused(version, "release")
            check_github_tag_unused(f"v{version}")
    else:
        require_changelog_changed()
        print(f"Version and changelog checks passed for {version}; previous full release was {latest or 'none'}.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
