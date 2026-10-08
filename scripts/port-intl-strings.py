#!/usr/bin/env python3
"""Converts react-aria's localized message bundles into leptonic's Rust tables.

    scripts/port-intl-strings.py

Reads `packages/*/intl/**/<locale>.json` of the react-spectrum checkout (REACT_SPECTRUM, default
../react-spectrum) for the families in FAMILIES and writes
`leptonic/src/utils/intl_strings/bundles/<module>.rs`: the messages of every locale in en-US key order
(the other locales behind the `intl-strings` feature) and a typed struct with one method per message.
Formatting happens at runtime (`utils::intl_strings`); its tests check every message of every locale.
Rerun after upstream changes the bundles; never edit the generated files.
"""
import json
import os
import pathlib
import re
import subprocess
import unicodedata

root = pathlib.Path(__file__).resolve().parent.parent
rs = pathlib.Path(os.environ.get("REACT_SPECTRUM", root.parent / "react-spectrum"))
out_dir = root / "leptonic/src/utils/intl_strings/bundles"

# (upstream directory below `packages/`, module, struct, what it is for). Only the bundles hooks use:
# add autocomplete with `use_autocomplete`; gridlist's announcements are unused upstream too.
FAMILIES = [
    ("react-aria/intl/breadcrumbs", "breadcrumbs", "BreadcrumbsStrings", "breadcrumbs"),
    ("react-aria/intl/calendar", "calendar", "CalendarStrings", "calendars"),
    ("react-aria/intl/color", "color", "ColorStrings", "the color hooks"),
    ("react-stately/intl/color", "color_names", "ColorNameStrings", "color channel and color names"),
    ("react-aria/intl/combobox", "combobox", "ComboBoxStrings", "combo boxes"),
    ("react-aria/intl/datepicker", "datepicker", "DatePickerStrings", "date fields and pickers"),
    ("react-stately/intl/datepicker", "date_validation", "DateValidationStrings", "date validation"),
    ("react-aria/intl/dnd", "dnd", "DndStrings", "drag and drop"),
    ("react-aria/intl/grid", "grid", "GridStrings", "grids"),
    ("react-aria/intl/menu", "menu", "MenuStrings", "menus"),
    ("react-aria/intl/numberfield", "numberfield", "NumberFieldStrings", "number fields"),
    ("react-aria/intl/overlays", "overlays", "OverlayStrings", "overlays"),
    ("react-aria/intl/searchfield", "searchfield", "SearchFieldStrings", "search fields"),
    ("react-aria/intl/spinbutton", "spinbutton", "SpinButtonStrings", "spin buttons"),
    ("react-aria/intl/table", "table", "TableStrings", "tables"),
    ("react-aria/intl/tag", "tag", "TagStrings", "tag groups"),
    ("react-aria/intl/toast", "toast", "ToastStrings", "toasts"),
    ("react-aria/intl/tree", "tree", "TreeStrings", "trees"),
    ("react-aria-components/intl", "atoms", "AtomStrings", "the atoms (react-aria-components)"),
]

# Upstream translation errors, repaired while converting: (directory, locale, key) -> (wrong, right).
FIXES = {
    # A translated argument name (upstream renders "undefined").
    ("react-aria/intl/gridlist", "sr-SP", "hasLinkAnnouncement"): ("{veza}", "{link}"),
}

def snake(name):
    name = re.sub(r"[^A-Za-z0-9]+", "_", name)
    return re.sub(r"(?<=[a-z0-9])([A-Z])", r"_\1", name).lower().strip("_")


def rust_str(s):
    out = []
    for c in s:
        if c in '"\\':
            out.append("\\" + c)
        elif c == "\n":
            out.append("\\n")
        elif unicodedata.category(c) in ("Cc", "Cf", "Zl", "Zp") or (unicodedata.category(c) == "Zs" and c != " "):
            out.append("\\u{%x}" % ord(c))
        else:
            out.append(c)
    return '"' + "".join(out) + '"'


class MessageArgs:
    """The arguments of an ICU message, from a parse that mirrors `utils/intl_strings/mod.rs`
    (`{name}`, `{name, plural, ..}`, `{name, select, ..}`, `#`, apostrophe quoting): name -> "str" |
    "count" (a plural) | "bool" (a select on true/other), in the order of first use."""

    NAME = re.compile(r"\s*([A-Za-z_]\w*)\s*")
    KIND = re.compile(r",\s*(\w+)\s*,")
    SELECTOR = re.compile(r"\s*(?:offset:\s*\d+\s*)?(=?[\w-]+)\s*\{")
    RANK = {"str": 0, "bool": 1, "count": 2}

    def __init__(self, message):
        self.message, self.args = message, {}
        end = self.text(0, in_plural=False)
        assert end == len(message), f"unbalanced message {message!r}"

    def add(self, name, kind):
        if kind is None:
            self.args.setdefault(name, "str")
        elif self.RANK[kind] >= self.RANK[self.args.get(name, "str")]:
            self.args[name] = kind

    def text(self, i, in_plural):
        m = self.message
        while i < len(m) and m[i] != "}":
            if m[i] == "{":
                i = self.argument(i + 1)
            elif m[i] == "'":
                i = self.apostrophe(i, in_plural)
            else:
                i += 1
        return i

    def apostrophe(self, i, in_plural):
        m = self.message
        following = m[i + 1 : i + 2]
        if following == "'":
            return i + 2
        if following in ("{", "}") or (in_plural and following == "#"):
            j = i + 1
            while j < len(m):
                if m[j] == "'":
                    if m[j + 1 : j + 2] == "'":
                        j += 2
                        continue
                    return j + 1
                j += 1
            return j
        return i + 1

    def argument(self, i):
        m = self.message
        name = self.NAME.match(m, i)
        assert name, f"no argument name at {i} in {m!r}"
        self.add(name.group(1), None)
        i = name.end()
        if m[i] == "}":
            return i + 1
        kind = self.KIND.match(m, i)
        assert kind and kind.group(1) in ("plural", "select"), f"unsupported argument at {i} in {m!r}"
        i = kind.end()
        selectors = []
        while True:
            while m[i].isspace():
                i += 1
            if m[i] == "}":
                break
            selector = self.SELECTOR.match(m, i)
            assert selector, f"no option at {i} in {m!r}"
            selectors.append(selector.group(1))
            # `#` is the count in a plural's own options only.
            i = self.text(selector.end(), in_plural=kind.group(1) == "plural")
            assert m[i] == "}", f"unclosed option at {i} in {m!r}"
            i += 1
        if kind.group(1) == "plural":
            self.add(name.group(1), "count")
        else:
            self.add(name.group(1), "bool" if "true" in selectors else "str")
        return i + 1


def args_of(message):
    return MessageArgs(message).args


def pascal(name):
    return "".join(part[:1].upper() + part[1:] for part in snake(name).split("_"))


RUST_TYPE = {"str": "&str", "count": "usize", "bool": "bool"}
# The args structs generated, as (module, name), for the re-exports.
ARGS_STRUCTS = []
ARG_CTOR = {"str": "Arg::Str", "count": "Arg::Count", "bool": "Arg::Bool"}


def generate(upstream, module, struct, purpose, commit):
    src = rs / "packages" / upstream
    locales = sorted(p.stem for p in src.glob("*.json"))
    bundles = {loc: json.loads((src / f"{loc}.json").read_text()) for loc in locales}
    for (directory, loc, key), (wrong, right) in FIXES.items():
        if directory == upstream:
            assert wrong in bundles[loc][key], f"fix no longer needed: {directory} {loc} {key}"
            bundles[loc][key] = bundles[loc][key].replace(wrong, right)
    en = bundles["en-US"]
    keys = list(en)
    lines = [
        f"// Generated by scripts/port-intl-strings.py from {upstream}/*.json @ {commit}. Do not edit.",
        f"//! Localized messages of {purpose}.",
        "",
        "use crate::utils::intl_strings::{"
        + ("Arg, " if any(args_of(en[k]) for k in keys) else "")
        + "Bundle, LocalizedStrings, Strings};",
        "",
        "const EN_US: &[&str] = &[",
    ]
    lines += [f"    {rust_str(en[k])}," for k in keys]
    lines += ["];", "", '#[cfg(feature = "intl-strings")]', "const OTHER: &[(&str, &[&str])] = &["]
    for loc in locales:
        if loc == "en-US":
            continue
        lines.append(f'    ("{loc}", &[')
        lines += [f"        {rust_str(bundles[loc].get(k, en[k]))}," for k in keys]
        lines.append("    ]),")
    lines += [
        "];",
        '#[cfg(not(feature = "intl-strings"))]',
        "const OTHER: &[(&str, &[&str])] = &[];",
        "",
        "static BUNDLE: Bundle = Bundle {",
        "    keys: &[" + ", ".join(rust_str(k) for k in keys) + "],",
        "    en_us: EN_US,",
        "    other: OTHER,",
        "};",
        "",
        f"/// Localized messages of {purpose}, in the locale they were created for",
        "/// ([`use_localized_strings`](crate::utils::intl_strings::use_localized_strings)).",
        "#[derive(Debug, Clone, PartialEq)]",
        f"pub struct {struct}(Strings);",
        "",
        f"impl LocalizedStrings for {struct} {{",
        "    fn bundle() -> &'static Bundle {",
        "        &BUNDLE",
        "    }",
        "",
        "    fn new(strings: Strings) -> Self {",
        "        Self(strings)",
        "    }",
        "",
        "    fn strings(&self) -> &Strings {",
        "        &self.0",
        "    }",
        "}",
        "",
        f"impl {struct} {{",
    ]
    structs = []
    for index, key in enumerate(keys):
        args = args_of(en[key])
        doc = en[key].replace("\n", " ")
        types = list(args.values())
        # Arguments that could be swapped and still compile (two of one type) are named fields
        # of an args struct, so that an upstream reordering can't swap them silently.
        if len(set(types)) < len(types):
            name = f"{pascal(key)}Args"
            lifetime = "<'a>" if "str" in types else ""
            structs += [
                f"/// The arguments of [`{struct}::{snake(key)}`].",
                "#[derive(Debug, Clone, Copy)]",
                f"pub struct {name}{lifetime} {{",
                *(f"    pub {snake(n)}: {RUST_TYPE[t].replace('&', chr(38) + chr(39) + 'a ')}," for n, t in args.items()),
                "}",
                "",
            ]
            params = f", args: {name}{'<' + chr(39) + '_>' if lifetime else ''}"
            values = ", ".join(f'("{n}", {ARG_CTOR[t]}(args.{snake(n)}))' for n, t in args.items())
            ARGS_STRUCTS.append((module, name))
        else:
            params = "".join(f", {snake(n)}: {RUST_TYPE[t]}" for n, t in args.items())
            values = ", ".join(f'("{n}", {ARG_CTOR[t]}({snake(n)}))' for n, t in args.items())
        lines += [
            f"    /// en-US: `{doc}`",
            "    #[must_use]",
            f"    pub fn {snake(key)}(&self{params}) -> String {{",
            f"        self.0.format({index}, &[{values}])",
            "    }",
        ]
    lines += ["}", ""]
    lines += structs
    (out_dir / f"{module}.rs").write_text("\n".join(lines))


def main():
    commit = subprocess.run(
        ["git", "-C", str(rs), "rev-parse", "--short=10", "HEAD"], capture_output=True, text=True, check=True
    ).stdout.strip()
    out_dir.mkdir(parents=True, exist_ok=True)
    for family in FAMILIES:
        generate(*family, commit)
    mods = [f"// Generated by scripts/port-intl-strings.py @ {commit}. Do not edit.", ""]
    mods += [f"mod {m};" for _, m, _, _ in FAMILIES]
    mods.append("")
    names = [s for _, _, s, _ in FAMILIES] + [name for _, name in ARGS_STRUCTS]
    assert len(names) == len(set(names)), "two generated types share a name"
    for _, m, s, _ in FAMILIES:
        exported = [s] + sorted(name for module, name in ARGS_STRUCTS if module == m)
        mods.append(f"pub use {m}::{{{', '.join(exported)}}};" if len(exported) > 1 else f"pub use {m}::{s};")
    mods.append("")
    mods.append("/// Every bundle, for the tests.")
    mods.append("#[cfg(test)]")
    mods.append("pub(super) fn all() -> Vec<&'static super::Bundle> {")
    mods.append("    use super::LocalizedStrings;")
    mods.append("    vec![" + ", ".join(f"{s}::bundle()" for _, _, s, _ in FAMILIES) + "]")
    mods.append("}")
    mods.append("")
    (out_dir / "mod.rs").write_text("\n".join(mods))
    subprocess.run(["rustfmt", "--edition", "2024", *map(str, out_dir.glob("*.rs"))], check=True)


main()
