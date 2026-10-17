# The atom theme

`leptonic-theme/scss/leptonic-atoms.scss` styles leptonic's atoms: a port of react-aria-components' starter styles
(`~/dev/react-spectrum/starters/docs/src/*.css`, Apache-2.0, Copyright Adobe). It is leptonic's only theme, optional
and maintained (the user). The book styles the atoms itself and never loads it, and the test-app needs none, so
visual checks of the theme need an app of their own (`PLAN.md`).

## Using it

Apps opt in with `@use "leptonic/leptonic-atoms";`. leptonic's build script copies the SCSS to
`<style-dir>/leptonic`, configured in the app's `Cargo.toml`:

```toml
[package.metadata.leptonic]
style-dir = "style"
```

The build script finds that `Cargo.toml` by walking up from `OUT_DIR`: a `CARGO_TARGET_DIR` inside the app works as
is, one elsewhere needs `LEPTONIC_APP_DIR=<app dir>`, or the theme is silently not copied.

## Files

- `atoms/_theme.scss`: the tokens (`theme.css`): tint and gray scales, semantic colors (`--text-color`,
  `--focus-ring-color`, `--highlight-background`, ...), typography and spacing. `--tint` recolors everything.
- `atoms/_mode.scss`: `@include mode.dark { .. }` / `mode.light`, replacing upstream's
  `@media (prefers-color-scheme: ..)` inside rules. The nearest `data-theme` (`ThemeProvider`) decides the tokens;
  without one, the system preference does.
- `atoms/_utilities.scss`: upstream's utilities (`button-base`, `indicator`, `inset`) as mixins, and as classes.
- `atoms/<family>.scss`: one file per upstream stylesheet, named like it in kebab case (`ListBox.css` →
  `list-box.scss`), wired into `leptonic-atoms.scss`.

## Porting a stylesheet

1. `scripts/port-starter-css.py ListBox.css > leptonic-theme/scss/atoms/list-box.scss` does the mechanical part:
   the `// Upstream:` header, `.react-aria-X` → `.leptonic-X`, dark/light media → the mode mixins.
2. Map the selectors to our atoms' markup by hand. Read the upstream component (`<Name>.tsx` next to the CSS: which
   classes and elements the starter renders; add an `// Upstream:` line for it) and our atom, and:
   - Rename classes whose atom has another name: upstream `Group` → `leptonic-NumberFieldGroup`,
     `leptonic-DatePickerGroup`; `Row`/`Cell`/`Column` → `leptonic-TableRow`/`leptonic-TableCell`/
     `leptonic-TableColumnHeader`; `Text` slots → `leptonic-ListBoxItemLabel`/`leptonic-ListBoxItemDescription` (or
     `MenuItem…`); `Heading` → the atom's heading (`leptonic-DialogTitle`, `leptonic-CalendarHeading`). Keep a
     selector list when one upstream class covers several atoms.
   - Utility classes the starter adds to the atom's element (`button-base`, `indicator`, `inset`) become
     `@include utilities.<name>` on the atom selectors inside `@layer utilities { .. }` at the top of the file
     (`button.scss`), so the atoms' own rules still win, as upstream.
   - Classes of parts the starter renders itself (a checkbox's `.indicator` box with its `svg`, a slider's
     `track inset`, a select's chevron) stay: apps render that markup. Document it in the header ("Markup: ...").
   - Variants upstream passes as props become data attributes the app sets (`attr:data-variant="secondary"`),
     documented in the header.
   - Our atoms render the same `data-*` state attributes as react-aria-components (absent when false). Where an atom
     lacks one the stylesheet uses, or an atom is missing, note it in `PLAN.md` instead of working around it.
3. Compile: `~/.cache/cargo-leptos/sass-*/sass-*/dart-sass/sass leptonic-theme/scss/leptonic-atoms.scss out.css`.

## Absorbing upstream changes

`scripts/upstream-drift.sh -v leptonic-theme/` lists upstream commits since each file's sync point (its `.css` and
`.tsx` lines); re-run the converter on the new upstream file, diff against ours, port the changes, `--mark-synced`.
