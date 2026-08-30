#!/usr/bin/env python3
"""Compile and run every card snippet; diff stdout against the card's `expect`.

Same discipline as the rust-learning-library's own tools/run_examples.py:
rustc --edition 2024, no crates, exact output match.
"""
import importlib.util, pathlib, re, subprocess, sys

HERE = pathlib.Path(__file__).parent
SNIP = HERE / "snippets"; SNIP.mkdir(exist_ok=True)
BIN  = HERE / "bin";      BIN.mkdir(exist_ok=True)
EDITION = "2024"

def load(name):
    spec = importlib.util.spec_from_file_location(name, HERE / f"{name}.py")
    m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m); return m

def check(card):
    """-> (status, detail).  status in {ok, MISMATCH, BUILD-FAIL, WRONG-ERROR, RUNTIME}"""
    code = card.get("code")
    if not code:
        return ("skip", "no code")
    src = SNIP / f"{card['id']}.rs"
    src.write_text(code + "\n", encoding="utf-8")
    binary = BIN / card["id"]
    build = subprocess.run(
        ["rustc", "--edition", EDITION, str(src), "-o", str(binary)],
        capture_output=True, text=True)

    want_fail = card.get("fails")
    if want_fail:
        if build.returncode == 0:
            return ("WRONG-ERROR", f"expected {want_fail}, but it COMPILED")
        codes = sorted(set(re.findall(r"\[(E\d{4})\]", build.stderr)))
        if want_fail not in codes:
            first = next((l for l in build.stderr.splitlines() if l.startswith("error")), "")
            return ("WRONG-ERROR", f"expected {want_fail}, got {codes or 'none'} | {first}")
        return ("ok", f"fails with {want_fail} as claimed")

    if build.returncode != 0:
        first = "\n".join(build.stderr.splitlines()[:6])
        return ("BUILD-FAIL", first)

    run = subprocess.run([str(binary)], capture_output=True, text=True, timeout=30)
    if run.returncode != 0:
        return ("RUNTIME", f"exit {run.returncode}: {run.stderr.strip()[:200]}")
    got = run.stdout.rstrip("\n")
    want = (card.get("expect") or "").rstrip("\n")
    if got != want:
        return ("MISMATCH", f"want {want!r}\n         got  {got!r}")
    return ("ok", "")

def main(modules):
    bad = 0
    for name in modules:
        m = load(name)
        print(f"\n=== {name} ({len(m.CARDS)} cards) ===")
        for card in m.CARDS:
            status, detail = check(card)
            if status == "ok":
                print(f"  ok       {card['id']}" + (f"  ({detail})" if detail else ""))
            elif status == "skip":
                print(f"  --       {card['id']}  (prose only)")
            else:
                bad += 1
                print(f"  {status:<10} {card['id']}\n         {detail}")
    print(f"\n{'ALL VERIFIED' if not bad else str(bad) + ' PROBLEM(S)'}")
    return 1 if bad else 0

if __name__ == "__main__":
    sys.exit(main(sys.argv[1:] or ["cards_strings"]))
