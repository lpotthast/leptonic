#!/usr/bin/env python3
"""Mechanical first pass of porting a react-aria-components starter stylesheet to the atom theme.

    scripts/port-starter-css.py Button.css > leptonic-theme/scss/atoms/button.scss

Reads `starters/docs/src/<file>` of the react-spectrum checkout (REACT_SPECTRUM, default
../react-spectrum) and prints SCSS: the `// Upstream:` header, `.react-aria-X` as `.leptonic-X`,
`@media (prefers-color-scheme: dark|light)` as `@include mode.dark|light` (leptonic follows
`data-theme`), the starter's own `@import`s dropped. Review the result by hand against the atoms'
markup: class names of parts the starter components render themselves stay as they are.
"""
import os
import pathlib
import re
import subprocess
import sys

root = pathlib.Path(__file__).resolve().parent.parent
rs = pathlib.Path(os.environ.get("REACT_SPECTRUM", root.parent / "react-spectrum"))
name = sys.argv[1]
src = rs / "starters/docs/src" / name
commit = subprocess.run(
    ["git", "-C", str(rs), "rev-parse", "--short=9", "HEAD"], capture_output=True, text=True, check=True
).stdout.strip()
css = src.read_text()
css = re.sub(r"^@import [^;]+;\n", "", css, flags=re.M)
css = re.sub(r"\.react-aria-([A-Za-z]+)", r".leptonic-\1", css)
css = re.sub(r"@media \(prefers-color-scheme: (dark|light)\)", r"@include mode.\1", css)
uses = '@use "mode";\n' if "@include mode." in css else ""
print(f"// Upstream: ../starters/docs/src/{name} @ {commit}\n{uses}\n{css.lstrip()}", end="")
