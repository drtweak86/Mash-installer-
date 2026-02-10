#!/usr/bin/env python3
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
MARKDOWN = list(ROOT.glob("docs/**/*.md"))
LINK_RE = re.compile(r"\[.*?\]\((.*?)\)")

def is_external(link: str) -> bool:
    return link.startswith(("http://", "https://", "mailto:", "javascript:", "ftp://"))

def main() -> int:
    missing = []
    for md_path in MARKDOWN:
        text = md_path.read_text(encoding="utf-8")
        for target in LINK_RE.findall(text):
            target = target.strip()
            if not target or target.startswith("#") or is_external(target):
                continue
            target_path = pathlib.Path(target.split("#", 1)[0])
            resolved = (md_path.parent / target_path).resolve()
            if not resolved.exists():
                missing.append((md_path.relative_to(ROOT), target))

    if missing:
        print("Broken documentation references detected:")
        for source, target in missing:
            print(f"  {source} -> {target}")
        return 1

    print("Documentation link check passed.")
    return 0

if __name__ == "__main__":
    sys.exit(main())
