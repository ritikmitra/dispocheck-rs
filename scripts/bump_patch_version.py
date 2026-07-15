#!/usr/bin/env python3
"""Bump the patch version in Cargo.toml, e.g. 1.2.3 -> 1.2.4.

Deliberately dependency-free (no cargo-edit) so it runs on a bare CI
runner with only Python 3 available. Prints the new version to stdout.
"""
import re
import sys
from pathlib import Path

CARGO_TOML = Path(__file__).resolve().parent.parent / "Cargo.toml"
VERSION_RE = re.compile(r'^version\s*=\s*"(\d+)\.(\d+)\.(\d+)"', re.MULTILINE)


def main() -> int:
    text = CARGO_TOML.read_text()
    match = VERSION_RE.search(text)
    if not match:
        print("error: could not find version in Cargo.toml", file=sys.stderr)
        return 1

    major, minor, patch = (int(g) for g in match.groups())
    new_version = f"{major}.{minor}.{patch + 1}"

    new_text = text[: match.start()] + f'version = "{new_version}"' + text[match.end() :]
    CARGO_TOML.write_text(new_text)

    print(new_version)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
