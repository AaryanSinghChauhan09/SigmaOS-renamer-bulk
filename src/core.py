"""Bulk regex file renamer for SigmaOS-renamer-bulk (inspired by bulky).

Principles: dry-run by default (defensive), fail-fast on collisions,
arrange-act-assert flow (plan -> apply -> report), ZERO-COPY streaming of paths.
"""
import argparse, os, re, sys, tempfile

ENGINE = {
    "name": "sigma-renamer",
    "summary": "Bulk regex file renamer with preview",
    "language": "python",
    "verbs": ["preview", "apply"],
    "principles": ["Fail-Fast", "Defensive Programming", "Least Astonishment",
                   "KISS", "DRY", "Deterministic Testing"],
}

cli_doc = """\
$ sigma-renamer preview /dir --pattern '^img_' --replace 'photo-'
img_001.jpg -> photo-001.jpg
$ sigma-renamer apply  /dir --pattern '\\.jpe?g$' --replace '.jpg'
"""


class RenameError(Exception):
    pass


def plan(directory, pattern, replacement, extension=None):
    """Return list of (src, dst) pairs without touching the filesystem."""
    try:
        rx = re.compile(pattern)
    except re.error as e:
        raise RenameError(f"bad regex: {e}") from e
    pairs = []
    for entry in sorted(os.listdir(directory)):
        src = os.path.join(directory, entry)
        if not os.path.isfile(src):
            continue
        if extension and not entry.lower().endswith(extension.lower()):
            continue
        new = rx.sub(replacement, entry)
        if new != entry:
            pairs.append((src, os.path.join(directory, new)))
    return pairs


def check_collisions(pairs):
    seen, dupes = set(), []
    for _, dst in pairs:
        base = os.path.basename(dst)
        if base in seen:
            dupes.append(base)
        seen.add(base)
        if os.path.exists(dst):
            dupes.append(dst)
    if dupes:
        raise RenameError("collision(s): " + ", ".join(sorted(set(dupes))))


def apply(pairs, dry_run=True):
    """Apply planned renames. Returns count applied."""
    check_collisions(pairs)
    done = 0
    for src, dst in pairs:
        if not dry_run:
            os.rename(src, dst)
        done += 1
    return done


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog=ENGINE["name"], description=ENGINE["summary"])
    sub = p.add_subparsers(dest="cmd", required=True)
    for verb in ("preview", "apply"):
        sp = sub.add_parser(verb, help=f"{verb} renames")
        sp.add_argument("directory")
        sp.add_argument("--pattern", required=True)
        sp.add_argument("--replace", required=True, dest="replacement")
        sp.add_argument("--extension", default=None)
    return p


def main(argv=None) -> int:
    args = build_parser().parse_args(argv)
    try:
        pairs = plan(args.directory, args.pattern, args.replacement, args.extension)
        dry = args.cmd == "preview"
        n = apply(pairs, dry_run=dry)
        for src, dst in pairs:
            print(f"{os.path.basename(src)} -> {os.path.basename(dst)}")
        verb = "would rename" if dry else "renamed"
        print(f"{verb} {n} file(s)", file=sys.stderr)
        return 0
    except RenameError as e:
        print(f"error: {e}", file=sys.stderr)
        return 1


def self_test() -> None:
    with tempfile.TemporaryDirectory() as d:
        for n in ("img_001.jpg", "img_002.jpg", "notes.txt"):
            open(os.path.join(d, n), "w").close()
        pairs = plan(d, r"^img_", r"photo-", extension=".jpg")
        names = sorted(os.path.basename(s) for s, _ in pairs)
        assert names == ["img_001.jpg", "img_002.jpg"], names
        check_collisions(pairs)
        assert apply(pairs, dry_run=True) == 2
        assert os.path.exists(os.path.join(d, "img_001.jpg"))  # dry-run touched nothing
        assert apply(pairs, dry_run=False) == 2
        assert os.path.exists(os.path.join(d, "photo-001.jpg"))
        assert os.path.exists(os.path.join(d, "photo-002.jpg"))
    try:
        plan("/nonexistent-dir-zz", "(", "")
    except RenameError:
        pass
    else:
        raise AssertionError("bad regex not caught")


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
