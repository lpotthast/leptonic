# Build performance: compile times and binary sizes

What we know about how long apps using leptonic take to build and how big their output is: how to measure, where
time and bytes go, what was decided (with the numbers behind it) and what is left. Goal: leptonic's size and
complexity must not weigh down its users' builds. Wasm size is the first performance priority (`conventions.md`);
runtime costs: `performance.md`. Open work: `PLAN.md`, "Build performance".

## How to measure

Measure the book (`examples/book-ssr`), the biggest consumer (~62k lines of views on leptonic's ~79k).
`scripts/build-bench.sh` (see its header) runs `cargo leptos build` in a target dir of its own and prints wall time,
CPU time and the machine's load; scenarios `fresh`, `noop`, `book-edit`/`book-revert` (a string in a book page),
`lib-edit`/`lib-revert` (a line in `use_button`). It runs on Linux and macOS, restores an edited file however it ends, and fails if an edit no longer applies.

- An edit is measured twice (applying, reverting): both are real single-edit rebuilds; the same edit twice in a row
  is not.
- Other builds share the machine (load averages of 4 to 26 seen): note the load, compare CPU time (user + sys) too,
  and repeat a measurement before trusting a difference under 15%. Say which machine a number comes from.
- Measure in a target dir of its own and delete it afterwards; a built one can be copied for free (APFS `cp -c -R`,
  btrfs/XFS `cp --reflink=always -R`) and cargo considers the copy fresh.
- Experiments with manifests or profiles run on a copy of the app outside the repository whose `src` symlinks to the
  live sources (copy `Cargo.toml`, `Cargo.lock`, `style/`, `public/`, `icu4x-data/`, merge the repository's and the
  app's `.cargo/config.toml`, make the `leptonic` path absolute), so the shared tree is never touched. Profile
  settings also go through `--config 'profile.dev.package.leptonic.opt-level=0'`.
- Compile time per pass: `RUSTC_BOOTSTRAP=1 cargo rustc -p <crate> --lib ... -- -Ztime-passes` (`RUSTC_BOOTSTRAP` is
  part of the fingerprint: the first such build recompiles everything). Units and critical path: `cargo build
  --timings`.
- Generic instantiations: `cargo llvm-lines -p book-ssr --lib --target wasm32-unknown-unknown --no-default-features
  --features hydrate` in the book's directory; aggregate by function name with the generic arguments stripped.
- Wasm sizes: report raw, gzip -9 and brotli -q 11. `twiggy top` on the wasm *before* wasm-bindgen
  (`<target>/front/wasm32-unknown-unknown/<profile>/<app>.wasm`, which has the names) attributes code to functions.
  Data can't be attributed in an LTO build: link a non-LTO build with a map (`-C link-arg=--Map=<file>`) and sum the
  `.rodata` input sections per object file (one per crate).

## Where the time goes (book, 2026-10-07, Apple M4 Max)

- **Fresh build** (critical path): `syn` → `wasm-bindgen-macro` → `js-sys` → `web-sys` → `tachys` → `leptos` →
  `leptonic` → `book-ssr` → link. `book-ssr` can't start before `leptonic` has *finished* (a `cdylib` + `rlib` crate
  links, so cargo's pipelining is off), and cargo-leptos compiles every proc macro and build script twice (the wasm
  builds in `<target>/front`). Heavy dependency groups (CPU): syntect + regex 68 s, ICU4X 62 s, `aws-lc-sys` 27 s
  (rustls' default provider), zstd 11 s (`tower-http`'s `full`).
- **leptonic itself** (wasm, `full`, `opt-level = 0`): 23.6 s, no single hot spot.
- **The book crate**: type checking dominates a full rebuild (16 of 35 s, inferring the deeply nested view types of
  functions returning `impl IntoView`; `-Ztime-passes` books it under "coherence_checking"). An incremental edit
  (9.4 s on the wasm side) re-expands every `view!` of the crate (3.4 s), then wasm-bindgen runs (3-4 s, also on a
  no-op build).
- **IR of a user crate** (`llvm-lines`): component bodies 33%, `AnyView` plumbing 13%, hydrate paths 12.5%,
  reactive_graph generics 7%, typed-builder's `PropsBuilder::build` 5.5% (instantiated per combination of props a
  call site sets: ~1,000 lines per copy for leptonic's big atoms).

## Where the bytes go (book release wasm, 2026-10-07)

Code 52%, data 47%. Data by crate: `icu_datetime` 3.64 MB (every locale, calendar system, time zone name; before
the book baked its own data), the book's texts and samples 2.07 MB, `icu_collator` 1.09 MB, syntect + regex 0.73 MB,
`jiff_tzdb` 0.22 MB, leptonic 0.20 MB. Code: 23% `to_children` closures (the volume of the app's views), 8% hydrate
paths, 2.4% `PropsBuilder::build`; a hydrated app contains both the hydrate and the build path of every view. The dev
wasm is mostly its `name` section (117 of 167 MB).

## Decisions and their numbers

**Type-erased views** (`--cfg=erase_components`, 2026-10-16, Linux, 32 threads, load 6–30; test-app `wasm-release`
without LTO, its server without debug info in both modes):

| | erased | not erased |
|---|---|---|
| wasm after wasm-opt, raw / gzip / brotli | 15.7 / 4.83 / 3.09 MB | 25.3 / 7.62 / 4.86 MB (+61 / +58 / +57%) |
| fresh build, wall / CPU / peak memory | 263 s / 1531 s / 6.0 GB | 890 s / 3439 s / 20.7 GB |
| no-op / leptonic edit (median), wall | 17 s / 79 s | 49 s / 147 s |
| allocations of the perf suite (5,010 measurements) | 131.7 M | 121.9 M (−7.5%, mounts −8.5%; reallocations −33%) |

Without erasure the test-app needs `#![recursion_limit = "256"]`, its server with debug info exceeds 32 GB, and the
book's dev build hadn't finished after 11 minutes (erased: 4.5). Erased views cost allocations (`performance.md`,
"View layer") and DOM writes: an element's attributes become a `Vec<AnyAttribute>` whose rebuild removes and sets every
attribute again (−39% attribute writes without erasure). Verdict: erased in every build, release included. Erasure
also hides bugs: tachys rebuilds a typed view in place (`Attribute::rebuild`) but builds an erased one anew;
leptos-element-capture 0.1's `rebuild` didn't capture (fixed in 0.2), and 23 browser tests failed only without
erasure (also hydration panics around `<Show>` + `<Portal use_shadow=true>`). cargo-leptos' release default is not
erased, so the suite also runs without it before releases. Not yet measured: the book's LTO production build in both
modes.

Dev profile (book; wall / CPU of single-edit rebuilds):

| Change                                                         | Effect                                                            | Verdict |
|----------------------------------------------------------------|-------------------------------------------------------------------|---------|
| leptonic at `opt-level = 0` (no override) instead of 3         | leptonic edit 36-46 s / 187-225 s → 18-20 s / 45-51 s             | applied |
| leptonic at `opt-level = 1`                                    | leptonic edit 28-38 s                                             | rejected |
| optimized proc macros (`build-override opt-level = 3`)          | fresh CPU 560 → 1133 s, no faster expansion                       | rejected |
| library as `rlib` + a tiny `cdylib` crate (pipelining)          | fresh 129 → 107 s, edits unchanged                                | not applied |
| `wasm-dev` profile: `strip = "symbols"`, `debug = false` for the app | dev wasm 190 → 51 MB, no-op 4.2 → 3.5 s; stack traces lose names | applied |
| `wasm-dev` at `opt-level = "z"`                                | dev wasm 43.8 → 19.7 MB, book edit 17-18 → 12-14 s, lib edit same | applied |
| `server-dev` at `opt-level = 1`                                | SSR 82 → 51 ms per book page, rebuilds faster; cold build +30 s once | applied |

Together (2026-10-07, book): fresh 127 s / 765 s → 102.5 s / 489 s, book edit −15%, leptonic edit −50% wall / −78%
CPU. Pitfalls: a profile-wide `strip = "symbols"` is silently not applied to a package with its own profile override
(check `-C strip=symbols` with `cargo rustc ... -v`); an `opt-level` in `[profile.dev.package."*"]` overrides the
`opt-level` of every profile inheriting dev for all dependencies.

Release wasm (book, raw / brotli; baseline 18.18 / 4.28 MB):

| Change                                                                 | Raw / brotli      | Verdict |
|------------------------------------------------------------------------|-------------------|---------|
| ICU4X data of the book's locales only                                  | 14.42 / 3.53 MB   | applied |
| syntect only on the server (`syntax-highlight` only in the `ssr` feature) | 16.83 / 3.71 MB | applied |
| both, only the ICU markers in use                                      | **13.01 / 2.94 MB** | applied |
| every page a lazy route, `--split` (main module only)                  | 11.33 / 2.85 MB   | open (`PLAN.md`) |
| `opt-level = "s"` instead of `"z"`                                     | 20.25 / 4.55 MB   | rejected |
| a newer wasm-opt                                                       | −12 KB            | rejected |

Test-app wasm (`wasm-release`, 2026-10-16, raw / gzip / brotli): `opt-level = "z"` 15.1 / 4.72 / 3.03 MB;
`opt-level = 3` 25.7 / 7.38 / 4.07 MB (+70% / +56% / +34%); `"z"` without debug assertions 13.2 / 4.23 / 2.72 MB. So
the test-app stays at `"z"` (its counts are identical at both: `testing.md`, "Performance Measurements"). Earlier: LTO
with one codegen unit saved 1 MB of 13 but made a rebuild after a leptonic change take 84 s instead of 20 s; opt-levels
1/2/3/`"s"` ran about as fast as `"z"`.

- **ICU4X data**: ICU4X reads baked data at compile time with `--cfg icu4x_custom_data` and `ICU4X_DATA_DIR=<dir>`.
  Generate it with `icu4x-datagen` of the same minor version as the `icu_*` crates (`scripts/icu-datagen.sh` checks
  it): `--format baked --markers all --segmenter-models none --locales <...>`. `--markers all` is required
  (`--markers-for-bin` leaves out markers the crates reference at compile time); without `--segmenter-models none` the
  sources are 17-18 MB instead of 6.2 MB, same wasm. Data of another ICU4X minor version breaks the build:
  regenerate after every update (`just book-icu-data`, `just test-app-icu-data`). Unsupported locales fall back to
  their parent or the root.
- **syntect** links its syntax definitions and regex (1.35 MB raw, 0.58 MB brotli) into any wasm that calls the
  highlighter. The book highlights on the server only (`kit::code`'s `highlight` server function for pages the client
  renders; browser test `code_blocks_are_highlighted_after_client_side_navigation`).
- **Route splitting**: the main module's code shrinks from 9.5 to 2.6 MB (data stays), but wasm-split's chunks are
  fine-grained (3,198 files; a first page visit loads a median of 89 files, up to 511) and the feature is unstable
  (`__wasm_split_unstable`). Worth trying with a coarser grouping (a lazy route per page group).
- **`None`/`Some(view)` instead of `().into_any()`** early returns in atoms: +0.18% wasm (with `erase_components` the
  view is erased at the component boundary anyway, while `Option`/`Either` add monomorphized code). Kept as the atom
  pattern (no boxing per render), not for size.

## Advice for users

The book's guide "Optimizing Compile Times & Binary Sizes" (`/doc/optimizing-builds`) turns these into copyable
configuration; keep the two in step.

- No `opt-level` override for leptonic in dev, and none in `[profile.dev.package."*"]`. A `server-dev` profile
  (`bin-profile-dev`, `opt-level = 1`) renders several times faster without slower rebuilds.
- `[profile.dev.package."*"] debug = false`, `debug = "line-tables-only"` for the app: smaller target dirs, faster
  links.
- `--cfg=erase_components` for every build, dev and release (`.cargo/config.toml`), with cargo-leptos'
  `disable-erase-components = true` (cargo-leptos adds it to dev builds only), so that all builds share one set of
  flags: about 3× faster fresh builds, 2× faster rebuilds, a third of the memory and a 38% smaller wasm, for about
  7.5% more allocations.
- Release wasm: `opt-level = "z"`, no LTO, default codegen units, `incremental = true` in a `wasm-release` profile
  (`lib-profile-release`) for fast rebuilds; LTO + one codegen unit for production builds only
  (`CARGO_PROFILE_WASM_RELEASE_LTO=true`, `..._CODEGEN_UNITS=1`). Serve it compressed (precompressed sidecars; the book
  makes them at server startup, `src/assets.rs`), with `hash-files = true` and `cache-control: public,
  max-age=31536000, immutable` for `/pkg/`.
- Dev wasm: a `wasm-dev` profile (`lib-profile-dev`) without symbols and at `opt-level = "z"`.
- Bake only the ICU4X locales the app supports (−20% wasm).
- Enable only the leptonic features in use (`default-features = false`; `intl-strings` adds react-aria's messages in
  34 locales); `syntax-highlight` is heavy: enable it in the app's `ssr` feature only and highlight on the server.
- Server: `axum-server`'s `tls-rustls` builds `aws-lc-sys`; use `tls-rustls-no-provider` with `rustls` on `ring`
  (6.2 s instead of 29.5 s CPU), and only the `tower-http` features in use.
- Large apps: split views into several crates (every edit re-expands and type-checks all views of the edited crate).
- Linkers: rust-lld is the default on x86_64 Linux; mold saved 0.5 s per link (not worth the setup here); on macOS
  the book links with lld.

## Opportunities

- ICU4X's any-calendar `DateTimeFormatter` in `utils/date_time_formatter.rs` pulls in every calendar system's data
  and code, though it formats only Gregorian dates: `FixedCalendarDateTimeFormatter<Gregorian, _>` would do (the date
  field needs the locale's calendar). Not measured (`PLAN.md`).
- `PropsBuilder::build` copies per prop combination: fewer, grouped props (structs with `Default`) on the biggest
  atoms (`PLAN.md`).
- Panic locations are 81 KB of the release wasm (67 KB absolute build paths; `--remap-path-prefix` would shorten
  them).
- In Leptos (upstream): a non-generic props `build`, the size of hydrate + build paths, view types whose inference
  dominates type checking.

## Our workflow

- Agents share one target dir per project (`AGENTS.md`); a fresh book build costs ~8 GB and 9-13 minutes of CPU, the
  incremental caches grow by tens of GB a day (`just clean-agent-incremental`).
- The test-app is built only with `--release` (`testing.md`, "Speed and Memory"); the book's `just serve` uses
  `wasm-dev` and `server-dev`, its release wasm keeps fat LTO, one codegen unit and `panic = "abort"` (against the
  advice above: `PLAN.md`).
