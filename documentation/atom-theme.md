# The atom theme

`leptonic-theme/scss/leptonic-atoms.scss` styles leptonic's atoms: a port of react-aria-components' starter styles
(`~/dev/react-spectrum/starters/docs/src/*.css`, Apache-2.0, Copyright Adobe). Apps opt in with
`@use "leptonic/leptonic-atoms";` (the build script copies the SCSS into the app's `style-dir`). It replaces the
component themes (`leptonic-themes.scss`) once the components are gone; until then, use one or the other, never both
(the components render atoms, so the atom rules would reach into them).

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
   the `// Upstream:` header (`scripts/upstream-drift.sh` reports upstream changes), `.react-aria-X` → `.leptonic-X`,
   dark/light media → the mode mixins.
2. Map the selectors to our atoms' markup by hand. Read the upstream component (`<Name>.tsx` next to the CSS: it
   shows which classes and elements the starter renders) and our atom (`leptonic/src/atoms/`), and:
   - Rename classes whose atom has another name: e.g. upstream `Group` (a field's group) →
     `leptonic-NumberFieldGroup`, `leptonic-DatePickerGroup`; `Row`/`Cell`/`Column` → `leptonic-TableRow`/
     `leptonic-TableCell`/`leptonic-TableColumnHeader`; `Text` slots → `leptonic-ListBoxItemLabel`/
     `leptonic-ListBoxItemDescription` (or `MenuItem…`); `Heading` → the atom's heading (`leptonic-DialogTitle`,
     `leptonic-CalendarHeading`). Keep a selector list when one upstream class covers several atoms.
   - Utility classes the starter component adds (`button-base`, `indicator`, `inset`, `track`) become
     `@include utilities.<name>` on the atom selectors inside `@layer utilities { .. }` at the top of the file
     (see `button.scss`), so the atoms' own rules still win, as upstream.
   - Classes of parts the starter component renders itself (e.g. a checkbox's `.indicator` box with its `svg`, a
     select's chevron) stay as they are: apps render that markup. Document it in the file's header comment ("Markup:
     ...").
   - Variants upstream passes as props become data attributes the app sets (`attr:data-variant="secondary"`),
     documented in the header.
   - Data attributes: our atoms render the same `data-*` state attributes as react-aria-components (absent when
     false). Where an atom lacks one the stylesheet uses, note it in `PLAN.md` instead of working around it.
   - Drop rules for atoms we don't have (record them in `PLAN.md`).
3. Compile: `~/.cache/cargo-leptos/sass-*/sass-*/dart-sass/sass leptonic-theme/scss/leptonic-atoms.scss out.css`.

## Absorbing upstream changes

`scripts/upstream-drift.sh atoms/` lists upstream commits since each file's sync point; re-run the converter on the
new upstream file, diff against ours, port the changes, and `--mark-synced` the file.
