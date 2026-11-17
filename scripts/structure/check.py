#!/usr/bin/env python3
"""Guard Rust source/test sizes and capability-folder structure."""
import argparse
from collections import Counter, defaultdict
import os
from pathlib import Path, PurePosixPath
import re
import subprocess
import sys

from rust import is_test, measure

ROOT = Path(__file__).resolve().parents[2]
SOURCE_LIMIT, SOURCE_HARD = 800, 1500
TEST_HARD = 1000
FOLDER_SOFT, FOLDER_HARD = 40, 60


def rust_paths():
    result = subprocess.check_output(
        ["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z"], cwd=ROOT
    )
    return sorted(
        path for path in result.decode().split("\0")
        if path.endswith(".rs") and (ROOT / path).is_file()
    )


def folder_sizes(paths):
    folders = defaultdict(list)
    subdirs = defaultdict(set)
    for path in map(PurePosixPath, paths):
        for parent in path.parents:
            if parent.parent != parent:
                subdirs[str(parent.parent)].add(parent.name)
        if is_test(str(path)) or path.parent.name in {"examples", "benches", "bin"}:
            continue
        folders[str(path.parent)].append(path.stem)
    result = {}
    for folder, stems in folders.items():
        if len(stems) > FOLDER_SOFT:
            result[("folder", folder)] = len(stems)
        families = Counter(
            re.split(r"[_-]", stem, maxsplit=1)[0]
            for stem in stems if "_" in stem or "-" in stem
        )
        for name, count in families.items():
            has_base = name in stems or name in subdirs[folder]
            if count >= (2 if has_base else 3):
                result[("family", f"{folder}/{name}_*")] = count
    return result


def diagnostics(sizes, baseline=None, changed_paths=None):
    baseline = baseline or {}
    changed_paths = changed_paths or set()
    rows = []
    for path, size in sorted(sizes.items()):
        if size.source > SOURCE_LIMIT:
            old = baseline.get(path)
            legacy_shrinking = old is not None and old.source > SOURCE_LIMIT and size.source < old.source
            legacy_unchanged = old is not None and old.source > SOURCE_LIMIT and path not in changed_paths
            if size.source > SOURCE_HARD:
                rows.append(("error", path, f"{size.source} non-test lines; hard limit {SOURCE_HARD}"))
            elif legacy_shrinking or legacy_unchanged:
                rows.append(("warning", path, f"{size.source} non-test lines; existing over-limit file should shrink"))
            else:
                rows.append(("error", path, f"{size.source} non-test lines; limit {SOURCE_LIMIT}"))
        if size.tests > TEST_HARD:
            rows.append(("error", path, f"{size.tests} test lines; limit {TEST_HARD}"))
    for (kind, path), count in sorted(folder_sizes(sizes.keys()).items()):
        if kind == "folder":
            level = "error" if count > FOLDER_HARD else "warning"
            limit = FOLDER_HARD if level == "error" else FOLDER_SOFT
            rows.append((level, path, f"{count} Rust source files; limit {limit}"))
        else:
            rows.append(("warning", path, f"{count} prefixed sibling modules; group this capability into a directory"))
    return rows


def git_output(args):
    result = subprocess.run(args, cwd=ROOT, capture_output=True, text=True)
    if result.returncode:
        raise RuntimeError(result.stderr.strip() or f"command failed: {' '.join(args)}")
    return result.stdout


def baseline_sizes(base, paths):
    result = {}
    for path in paths:
        previous = subprocess.run(
            ["git", "show", f"{base}:{path}"], cwd=ROOT, capture_output=True, text=True
        )
        if previous.returncode == 0:
            result[path] = measure(path, previous.stdout)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base", default="HEAD", help="baseline revision for legacy file-size checks")
    args = parser.parse_args()
    paths = rust_paths()
    sizes = {path: measure(path, (ROOT / path).read_text()) for path in paths}
    try:
        changed = set(git_output(["git", "diff", "--name-only", args.base, "--"]).splitlines())
        baseline = baseline_sizes(args.base, paths)
    except RuntimeError as error:
        print(f"ERROR {error}")
        return 1
    rows = diagnostics(sizes, baseline, changed)
    for level, path, message in rows:
        print(f"{level.upper():7} {path}: {message}")
        if level == "error" and os.environ.get("GITHUB_ACTIONS") == "true":
            print(f"::error file={path}::{message}")
        elif level == "warning" and os.environ.get("GITHUB_ACTIONS") == "true":
            print(f"::warning file={path}::{message}")
    errors = sum(level == "error" for level, _, _ in rows)
    warnings = len(rows) - errors
    print(f"Structure: {len(paths)} Rust files, {errors} errors, {warnings} warnings")
    return bool(errors)


if __name__ == "__main__":
    sys.exit(main())
