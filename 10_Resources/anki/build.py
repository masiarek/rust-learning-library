#!/usr/bin/env python3
"""Render verified cards into Anki plain-text import files (tab separated).

Fields: Front, Back, Tags.  No literal newline ever reaches a field -- code is
rendered as <pre> with <br>, so every note is exactly one line of TSV.
"""
import html, importlib.util, pathlib, sys

HERE = pathlib.Path(__file__).parent
OUT  = HERE / "out"; OUT.mkdir(exist_ok=True)

PRE = ('<pre style="text-align:left; white-space:pre-wrap; font-size:0.92em; '
       'line-height:1.45; border-left:3px solid #7a7a7a; padding:2px 0 2px 10px; '
       'margin:10px 0; font-family:ui-monospace,SFMono-Regular,Menlo,Consolas,monospace;">')

def block(text, label=None):
    body = html.escape(text).replace("\n", "<br>")
    head = f'<div style="opacity:.6; font-size:.8em; margin-top:8px;">{label}</div>' if label else ""
    return head + PRE + body + "</pre>"

def load(name):
    spec = importlib.util.spec_from_file_location(name, HERE / f"{name}.py")
    m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m); return m

def render(card):
    front = card["front"]
    back_parts = []

    code, on = card.get("code"), card.get("code_on")
    if code and on == "front":
        front += block(code)
        if card.get("fails"):
            back_parts.append(block(f"error[{card['fails']}]  -- it does not compile", "rustc 1.98 says"))
        else:
            back_parts.append(block(card["expect"], "it prints"))

    back_parts.append(card["back"])

    if code and on == "back":
        rendered = code + "\n"
        if card.get("expect"):
            rendered += "\n" + "\n".join("// " + l for l in card["expect"].split("\n"))
        back_parts.append(block(rendered, "verified — rustc 1.98.0, edition 2024"))

    if card.get("bridge"):
        back_parts.append('<div style="opacity:.85; font-size:.95em; border-top:1px solid #9995; '
                          'margin-top:12px; padding-top:8px;">' + card["bridge"] + "</div>")

    label, url = card["link"]
    back_parts.append(f'<div style="margin-top:10px; font-size:.9em;">&rarr; '
                      f'<a href="{url}">{html.escape(label)}</a></div>')

    back = "<br>".join(back_parts)
    for f in (front, back):
        assert "\t" not in f and "\n" not in f, card["id"]
    return front, back, card["tags"]

def build(name, filename):
    m = load(name)
    lines = ["#separator:tab", "#html:true", "#notetype:Basic",
             f"#deck:{m.DECK}", "#tags column:3"]
    for card in m.CARDS:
        lines.append("\t".join(render(card)))
    path = OUT / filename
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"{path.name}: {len(m.CARDS)} cards -> {m.DECK}")
    return path

if __name__ == "__main__":
    build("cards_strings",   "Rust_Strings.txt")
    build("cards_vec",       "Rust_Vec.txt")
    build("cards_iterators", "Rust_Iterators.txt")
    build("cards_ownership", "Rust_Ownership.txt")
