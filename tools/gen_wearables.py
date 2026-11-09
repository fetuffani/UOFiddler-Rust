#!/usr/bin/env python3
"""Builds crates/uo_core/data/wearables.txt: item ids that UO server emulators treat as wearable,
with the layer when the scripts set it.

Usage: gen_wearables.py OUT.txt NAME=DIR [NAME=DIR ...]   (DIR = a checkout of a C# emulator)

It does not read the scripts one by one by hand: it parses every .cs file for
  * the class tree (`class A : B`),
  * constructors `public A(...) : base(0xID, [Layer.X,] ...)` (literal item id, layer argument),
  * `Layer = Layer.X;` inside constructors,
  * pass-through constructors `base(itemID, Layer.X, ...)` that give a whole class family its layer
    (BaseShirt -> Layer.Shirt, Ring -> Layer.Ring, ...),
and keeps the classes that descend from a wearable base class (BaseClothing, BaseArmor, BaseWeapon,
BaseJewel, ...) or that set a layer themselves. Armor and weapons do not name a layer in the
scripts (the server reads it from the tiledata), so they are listed with `-`.

Output line: `0xID LAYER SOURCE CLASS` (LAYER = UO layer number, `-` = the tiledata decides).
"""
import os
import re
import sys

LAYERS = {
    "OneHanded": 1, "TwoHanded": 2, "Shoes": 3, "Pants": 4, "Shirt": 5, "Helm": 6, "Gloves": 7,
    "Ring": 8, "Talisman": 9, "Neck": 10, "Hair": 11, "Waist": 12, "InnerTorso": 13, "Bracelet": 14,
    "FacialHair": 16, "MiddleTorso": 17, "Earrings": 18, "Arms": 19, "Cloak": 20, "Backpack": 21,
    "OuterTorso": 22, "OuterLegs": 23, "InnerLegs": 24, "Mount": 25,
}
# Classes whose descendants are worn (equipment), whatever the layer.
WEARABLE_ROOTS = {
    "BaseClothing", "BaseArmor", "BaseWeapon", "BaseShield", "BaseJewel", "BaseHat", "BaseQuiver",
    "BaseTalisman", "BaseEquipableLight", "BaseRing", "BaseBracelet", "BaseNecklace", "BaseEarrings",
    "BaseOuterTorso", "BaseMiddleTorso", "BaseShirt", "BasePants", "BaseShoes", "BaseWaist",
    "BaseCloak", "BaseInstrument", "BaseBook",
}
# BaseInstrument / BaseBook are held, not worn: they are not equipment for the paperdoll.
WEARABLE_ROOTS -= {"BaseInstrument", "BaseBook"}

COMMENTS = re.compile(r"//[^\n]*|/\*.*?\*/", re.S)
CLASS = re.compile(r"\bclass\s+(\w+)(?:\s*<[^>{]*>)?\s*(?::\s*([^{]+?))?\s*(?:\bwhere\b[^{]*)?\{")
LAYER_ARG = re.compile(r"\bLayer\.(\w+)")
HEX = re.compile(r"^0[xX]([0-9A-Fa-f]+)$")


def split_args(s):
    out, depth, cur = [], 0, ""
    for ch in s:
        if ch in "([{":
            depth += 1
        elif ch in ")]}":
            depth -= 1
        if ch == "," and depth == 0:
            out.append(cur.strip())
            cur = ""
        else:
            cur += ch
    if cur.strip():
        out.append(cur.strip())
    return out


def block_end(text, open_idx):
    """Index just after the brace that closes the one at open_idx."""
    depth = 0
    for i in range(open_idx, len(text)):
        c = text[i]
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                return i + 1
    return len(text)


def first_base(spec):
    if not spec:
        return None
    for part in split_args(spec):
        name = re.sub(r"<.*", "", part.strip()).split(".")[-1]
        if name and not re.match(r"^I[A-Z]", name):
            return name
    return None


def parse_repo(root):
    """-> (classes, ctors): classes[name] = base name; ctors = [(class, item_id|None, layer|None)]."""
    bases, ctors, default_layer = {}, [], {}
    for dirpath, _, files in os.walk(root):
        if "/.git" in dirpath:
            continue
        for fn in files:
            if not fn.endswith(".cs"):
                continue
            try:
                text = COMMENTS.sub("", open(os.path.join(dirpath, fn), encoding="utf-8-sig", errors="replace").read())
            except OSError:
                continue
            for m in CLASS.finditer(text):
                name = m.group(1)
                bases.setdefault(name, first_base(m.group(2)))
                body = text[m.end() - 1 : block_end(text, m.end() - 1)]
                ctor = re.compile(
                    r"\b(?:public|protected|internal)\s+" + re.escape(name)
                    + r"\s*\(([^)]*)\)\s*(?::\s*(base|this)\s*\(((?:[^()]|\((?:[^()]|\([^()]*\))*\))*)\))?\s*\{"
                )
                for c in ctor.finditer(body):
                    call_args = split_args(c.group(3) or "")
                    own_params = {p.split()[-1] for p in split_args(c.group(1)) if p.split()}
                    item_id = None
                    layer = None
                    if c.group(2) == "base" and call_args:
                        h = HEX.match(call_args[0])
                        if h:
                            item_id = int(h.group(1), 16)
                        for a in call_args[1:]:
                            lm = LAYER_ARG.fullmatch(a)
                            if lm:
                                layer = lm.group(1)
                    cbody = body[c.end() - 1 : block_end(body, c.end() - 1)]
                    am = re.search(r"\bLayer\s*=\s*Layer\.(\w+)\s*;", cbody)
                    if am:
                        layer = am.group(1)
                    if item_id is None and layer and call_args and call_args[0] in own_params:
                        default_layer.setdefault(name, layer)  # pass-through: the family's layer
                    ctors.append((name, item_id, layer))
    return bases, ctors, default_layer


def ancestors(name, bases):
    seen = []
    while name and name not in seen:
        seen.append(name)
        name = bases.get(name)
    return seen


def main():
    out_path, repos = sys.argv[1], [a.split("=", 1) for a in sys.argv[2:]]
    found = {}  # (item_id, layer or None) -> (source, class)
    for src, root in repos:
        bases, ctors, default_layer = parse_repo(root)
        for name, item_id, layer in ctors:
            if item_id is None or item_id <= 0 or item_id > 0xFFFF:
                continue
            chain = ancestors(name, bases)
            wearable = any(c in WEARABLE_ROOTS for c in chain)
            if layer is None:
                layer = next((default_layer[c] for c in chain if c in default_layer), None)
            if layer is not None and layer not in LAYERS:
                continue  # Invalid / shop / bank layers
            if not wearable and layer is None:
                continue
            key = (item_id, LAYERS.get(layer))
            found.setdefault(key, (src, name))
    # keep a layer-less entry only when the id has no entry with a layer
    with_layer = {i for (i, l) in found if l is not None}
    rows = sorted((i, l, s, c) for (i, l), (s, c) in found.items() if l is not None or i not in with_layer)
    with open(out_path, "w") as f:
        f.write("# Item ids that emulator scripts treat as wearable: `0xID LAYER SOURCE CLASS`.\n")
        f.write("# LAYER = UO layer number; `-` = the script does not name one (the tiledata decides).\n")
        f.write("# Generated by tools/gen_wearables.py from: " + ", ".join(f"{s}" for s, _ in repos) + ".\n")
        for i, l, s, c in rows:
            f.write(f"0x{i:04X} {l if l is not None else '-'} {s} {c}\n")
    print(f"{len(rows)} entries, {len({r[0] for r in rows})} ids", file=sys.stderr)


if __name__ == "__main__":
    main()
