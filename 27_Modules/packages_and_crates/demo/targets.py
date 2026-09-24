"""Print each target of a package as kind, name and file: `cargo metadata` on
stdin, and an optional package name to keep on the command line."""
import json
import os
import sys

metadata = json.load(sys.stdin)
wanted = sys.argv[1] if len(sys.argv) > 1 and sys.argv[1] else None
for package in metadata["packages"]:
    if wanted is not None and package["name"] != wanted:
        continue
    root = os.path.dirname(package["manifest_path"])
    for target in package["targets"]:
        kind = target["kind"][0]
        path = os.path.relpath(target["src_path"], root)
        print(f"   {kind:<4} {target['name']:<20} {path}")
