# Build performance: compile times and binary sizes

The living document for everything we know about how long apps using leptonic take to build and how big their
output is: how to measure, the baseline, where time and bytes go, what was tried (with numbers) and what is left.
Goal: leptonic's size and complexity must not weigh down the builds of its users. The findings feed three things:
changes to leptonic itself (every user profits), advice for users (setup and profiles, for the book's guides), and
our own workflow. Update this document whenever you measure something; keep raw numbers, not impressions. Open work
goes to `PLAN.md` as usual.

## How to measure

Measure the book (`examples/book-ssr`), the biggest consumer (about 62k lines of views on top of leptonic's 79k).
`scripts/build-bench.sh` (see its header) runs `cargo leptos build` in a target directory of its own and prints wall
time, CPU time (user, sys) and the machine's load. Scenarios:

- `fresh`: a build in an empty target directory (server and wasm, in parallel, as `cargo leptos` does).
- `noop`: nothing changed.
- `book-edit` / `book-revert`: one string literal changed in a book page (`atoms/button.rs`), then changed back.
- `lib-edit` / `lib-revert`: a line inserted into a leptonic hook (`use_button`), then removed again.

An edit is measured twice (applying it, reverting it): both are real single-edit rebuilds. Measuring the same edit
twice in a row is not: the second build sees the content of the first.

Rules and tools:

- Other agents and projects build on the same machine. Wall times are noisy (load averages of 4 to 26 were seen):
  note the load, compare CPU time (user + sys) too, and repeat a measurement before trusting a difference under 15%.
- Measure in a target directory of its own and delete it afterwards. A built target directory can be copied for
  free with APFS clones (`cp -c -R target/a target/b`); cargo considers the copy fresh. Good for starting an
  experiment from a built state.
- Experiments with manifests or profiles: use a copy of the app outside the repository whose `src` is a symlink to
  the live sources (copy `Cargo.toml`, `Cargo.lock`, `style/`, `public/`, the repository's `.cargo/config.toml`, and
  make the `leptonic` path absolute), so the shared working tree is never touched. Profile settings can also be
  passed as `--config 'profile.dev.package.leptonic.opt-level=0'`.
- Where a crate's compile time goes: `RUSTC_BOOTSTRAP=1 cargo rustc -p <crate> --lib ... -- -Ztime-passes`. Note:
  `RUSTC_BOOTSTRAP` is part of cargo's fingerprint, the first such build recompiles every dependency; the next one
  (same environment) shows the incremental case. `cargo build --timings` shows the units and the critical path.
- Which generics a crate instantiates: `cargo llvm-lines -p book-ssr --lib --target wasm32-unknown-unknown
  --no-default-features --features hydrate` (with `LEPTOS_OUTPUT_NAME=book-ssr`); aggregate the output by function
  name with the generic arguments stripped.
- Wasm sizes: report raw, gzip -9 and brotli -q 11 (what a browser downloads). `twiggy top` on the wasm *before*
  wasm-bindgen (`<target>/front/wasm32-unknown-unknown/<profile>/book_ssr.wasm`, which still has the function
  names) attributes code to functions. Data (strings, tables) can't be attributed in an LTO build (all symbols are
  anonymous): link a non-LTO release build with a map (`cargo rustc ... --config 'profile.wasm-release.lto=false'
  -- -C link-arg=--Map=<file>`) and sum the `.rodata` input sections per object file (one per crate).

Machine of all numbers below: Apple M4 Max (16 cores, 64 GB), rustc 1.99.0, cargo-leptos 0.3.11, wasm-bindgen
0.2.129, wasm-opt version_123 (cargo-leptos' download), leptos 0.8.19.

## Baseline (2026-10-07)

Configuration as it was: leptonic at `opt-level = 3` in the book's dev profile, `erase_components`, release wasm with
`opt-level = "z"`, fat LTO, one codegen unit, `panic = "abort"`, wasm-opt `-Oz`.

| Build (book)                               | Wall    | CPU (user + sys) |
|--------------------------------------------|---------|------------------|
| fresh, dev (`cargo leptos build`)          | 127 s   | 765 s            |
| fresh, release (`--release`)               | 215 s   | 1197 s           |
| no-op, dev                                 | 4 s     | 8 s              |
| book page edit, dev                        | 15-18 s | 40-47 s          |
| leptonic hook edit (one line), dev         | 36-46 s | 187-225 s        |

| Artifact (book)               | Raw      | gzip    | brotli  |
|-------------------------------|----------|---------|---------|
| wasm, release                 | 18.18 MB | 6.20 MB | 4.28 MB |
| wasm, dev                     | 167.2 MB | 18.4 MB |         |
| server binary, release        | 94.6 MB  |         |         |
| server binary, dev            | 250.8 MB |         |         |

A fresh dev build fills 8 GB of target directory (server 4.4 GB, wasm 3.6 GB).

## Where the time goes

**Fresh build.** Critical path (`cargo build --timings`, both builds in parallel): `syn` → `wasm-bindgen-macro` →
`js-sys` (10-16 s) → `web-sys` (12-14 s) → `tachys` → `leptos` → `leptonic` (50-60 s at `opt-level = 3`) →
`book-ssr` (45-61 s) → link.

- `book-ssr` can't start before `leptonic` has *finished*: its library is `crate-type = ["cdylib", "rlib"]`, and a
  crate that links needs its dependencies' full artifacts, so cargo's pipelining (start as soon as a dependency's
  metadata exists) is off. The server build compiles the cdylib too, for nothing.
- Every proc macro and build script is compiled twice: cargo-leptos builds the wasm in a target directory of its own
  (`<target>/front`).
- CPU time of dependency groups (server + wasm build): syntect, regex, fancy-regex 68 s; ICU4X (incl. zerovec & co.)
  62 s; icondata (all 19 icon sets) 47 s; `aws-lc-sys` 27 s (build script; `axum-server`'s `tls-rustls` pulls
  `rustls` with its default `aws-lc-rs` provider); ammonia + html5ever 17 s; `zstd-sys` 11 s (`tower-http`'s `full`
  feature); `leptos_reactive` 0.6 4 s (was unused).

**leptonic itself** (wasm, `full`, `opt-level = 0`, as in a user's dev build): 23.6 s. Codegen 6.3 s, type checking
5.6 s, metadata 4.4 s, borrow checking 3.5 s, monomorphization 3.3 s, macro expansion 1.8 s: no single hot spot.

**The book crate** (wasm, full rebuild, 35 s): type checking 16 s, metadata 6.9 s, monomorphization 5.7 s, codegen
5 s, LLVM 4.4 s, macro expansion 3.4 s. `-Ztime-passes` books nearly all type checking under "coherence_checking":
rustc type-checks a function returning `impl IntoView` during the well-formedness check (it needs the hidden type), so
this is inferring the book's deeply nested view types, not coherence.

**An incremental book edit** (wasm side, 9.4 s): macro expansion 3.4 s (not incremental: every `view!` of the 62k
lines is expanded again on each edit), codegen 1.8 s, metadata 1.2 s, the rest small. Then wasm-bindgen (3.3-3.6 s),
which runs after the wasm build, so it adds to the wall time. A no-op build still takes 4 s: cargo-leptos always
reruns wasm-bindgen.

**Where the user's crate's IR comes from** (`cargo llvm-lines`, book, wasm: 2.48M lines in 189k function copies):
component bodies (the book's views) 33%, `AnyView` plumbing 13%, hydrate paths 12.5%, reactive_graph generics 7%,
typed-builder `PropsBuilder::build` 5.5% (leptonic's big atoms: about 1,000 lines per combination of props a call
site sets, e.g. `TextFieldPropsBuilder::build` 10 copies, 9.9k lines), drop glue 1.6%.

## Where the bytes go (release wasm)

Sections: code 9.5 MB (52%), data 8.5 MB (47%).

**Data** (non-LTO release build, `.rodata` per crate, 8.9 MB):

| Crate          | Bytes   | What                                                                        |
|----------------|---------|-----------------------------------------------------------------------------|
| icu_datetime   | 3.64 MB | baked CLDR data: every locale, every calendar system, time zone names       |
| book_ssr       | 2.07 MB | the book's texts, demo sources (`include_str!`), code samples               |
| icu_collator   | 1.09 MB | baked collation data, every locale                                          |
| syntect        | 0.37 MB | syntax definitions (client-side highlighting of code blocks)                |
| regex_syntax   | 0.36 MB | Unicode tables (syntect's `fancy-regex`)                                    |
| web_atoms      | 0.26 MB | html5ever's atoms (`ammonia`, `SanitizedHtml`)                              |
| jiff_tzdb      | 0.22 MB | the bundled time zone database                                              |
| leptonic       | 0.20 MB |                                                                             |
| icu_normalizer, icu_locale, icu_list | 0.24 MB |                                                       |

**Code** (twiggy, LTO build before wasm-opt, 15.5 MB; wasm-opt shrinks it to 9.5 MB):

- 23%: `to_children` closures (7,901): constructing the views passed as `children`. The book's views themselves:
  their volume is the cost, not any one function.
- 8% hydrate paths of view types, 2% `rebuild`, 2% `into_any`. A hydrated app contains both the hydrate and the
  build (client-side navigation) path of every view.
- 2.4%: `PropsBuilder::build` (355 copies; typed-builder's builder is generic over which props were set).
- By crate (non-LTO): book_ssr 8.8 MB (it instantiates the generics), leptonic 4.4 MB, regex-automata 0.36 MB,
  ammonia + html5ever 0.3 MB, web-sys 0.15 MB, leptos-tiptap 0.15 MB.

The dev wasm is 167 MB: 117 MB of it is the `name` section (270k function names), 39 MB code, 10 MB data.

## Experiments

Dev experiments ran on copies of the book (sources symlinked to the live ones). "lib edit" and "book edit" are
wall / CPU (user + sys) of single-edit rebuilds.

| Change (dev)                                                     | Fresh wall / CPU | Book edit       | Lib edit        | Dev wasm | Verdict |
|------------------------------------------------------------------|------------------|-----------------|-----------------|----------|---------|
| baseline: leptonic `opt-level = 3`                               | 127 s / 765 s    | 15-18 s / 40-47 s | 36-46 s / 187-225 s | 167 MB | |
| leptonic `opt-level = 0` (the dev default)                       | 129 s / 560 s    | 13-15 s / 31-39 s | 18-20 s / 45-51 s | 190 MB | **applied** |
| leptonic `opt-level = 1`                                         | 111 s / 674 s    | 13 s / 34 s     | 28-38 s / 154-171 s | 142 MB | rejected |
| + library as `rlib` and a separate tiny `cdylib` frontend crate (pipelining; cargo-leptos workspace mode) | 107 s / 552 s | 13-15 s / 33-44 s | 17-19.5 s / 44-56 s | 190 MB | small gain: fresh only |
| + `[profile.dev.build-override] opt-level = 3` (optimized proc macros) | 142 s / 1133 s | 12-15 s / 30-41 s | 17-18.5 s / 42-52 s | 190 MB | rejected: no gain |
| + `strip = "symbols"` for the wasm (no `name` section)           |                  | 12.8-15.6 s / 36-48 s |           | 51 MB    | **applied** |

- `opt-level = 3` for leptonic made each leptonic edit 2x slower in wall time and 4x in CPU time, and its claimed
  benefit was smaller wasm: with `opt-level = 1` the dev wasm is even smaller (142 MB) than with 3 (167 MB). The
  dev wasm's size is dominated by the name section anyway.
- Optimized proc macros double the fresh build's CPU time (syn, serde_derive, leptos_macro, ... at `opt-level = 3`)
  without making expansion measurably faster.
- Stripping symbols removes function names from browser stack traces (`wasm-function[1234]`; panic messages keep
  file and line). The 190 MB dev wasm took the browser noticeably longer to load; 51 MB is about the size of the
  release build before wasm-opt. A no-op build goes from 4.2 to 3.5 s. The book's edit times stay (its server build
  is the slower half now: 15 s against 8.8 s wasm + 4 s wasm-bindgen).
- How: cargo-leptos' `lib-profile-dev = "wasm-dev"` builds the wasm with a profile of its own (the server keeps its
  symbols): `[profile.wasm-dev] inherits = "dev"` and `[profile.wasm-dev.package.<app>] strip = "symbols"`,
  `debug = false`. Pitfall: a profile-wide `strip = "symbols"` is silently not applied to a package that has its
  own profile override (the book's `[profile.dev.package.book-ssr]`): check the `-C strip=symbols` flag with
  `cargo rustc ... -v`.

| Change (release wasm)                                                         | Raw      | gzip    | brotli  |
|-------------------------------------------------------------------------------|----------|---------|---------|
| baseline                                                                      | 18.18 MB | 6.20 MB | 4.28 MB |
| ICU4X data for the book's 13 languages incl. regional variants (`--locales en en-GB de ja hi ar sv pt fr es fi nl da`) | 14.57 MB | 5.11 MB | 3.55 MB |
| same, the exact locales only (`^en ^en-GB ^de ...`)                            | 14.42 MB | 5.09 MB | 3.53 MB |
| every page a lazy route, `cargo leptos build --release --split` (main module)  | 11.33 MB | 3.89 MB | 2.85 MB |
| wasm-opt version_133 instead of version_123 (`LEPTOS_WASM_OPT_VERSION`)          | 18.17 MB | 6.20 MB | 4.28 MB |
| `opt-level = "s"` instead of `"z"`                                               | 20.25 MB | 6.83 MB | 4.55 MB |
| syntect only on the server (leptonic's `syntax-highlight` feature only in the book's `ssr` feature) | 16.83 MB | 5.50 MB | 3.71 MB |
| **applied: syntect only on the server + ICU4X data of the book's locales, only the markers of the ICU crates in use** | **13.01 MB** | **4.36 MB** | **2.94 MB** |

- **ICU4X data subset** (−20% raw, −17% brotli): ICU4X reads custom baked data at compile time when the crates are
  built with `--cfg icu4x_custom_data` and `ICU4X_DATA_DIR=<dir>`. Generate the directory with `icu4x-datagen`
  (same minor version as the `icu_*` crates; build it with `cargo install icu4x-datagen@2.2.0 --locked`):
  `icu4x-datagen --format baked --markers all --locales <...> --out <dir>`. `--markers all` is required:
  `--markers-for-bin` leaves out markers the ICU crates reference at compile time (errors like "cannot find macro
  `impl_normalizer_nfc_v1`"). That directory is 17-18 MB of Rust sources, mostly segmenter dictionaries no crate of
  the book uses: `scripts/icu-datagen.sh` passes the markers of the `icu_*_data` crates in the app's dependency tree
  instead (the file names of their baked data), leaving out the few that need an `unstable` datagen: 5.8 MB of
  sources (0.9 MB gzipped), same wasm. Data is decided by the app, leptonic needs no change; unsupported locales
  fall back to their parent or the root locale. Combined with route splitting the main module would be about
  6.5 MB (wasm-split keeps all data in the main module).
- **syntect on the client**: anything that calls `utils::syntax_highlight` in the wasm links syntect, its syntax
  definitions and regex (1.35 MB raw, 0.58 MB brotli: the definitions barely compress). Leaving the call out isn't
  enough while leptonic's `components::typography::Code` exists in the wasm: it highlights on the client whenever the
  feature is on, whatever its `language`. So the book enables `syntax-highlight` only in its `ssr` feature and its
  code blocks ask the server (`kit::code`'s `highlight` server function) for pages the client renders itself.
  Side effect: leptonic's styled `Code` component (components layer, on its way out) doesn't highlight in the book.
- **Route splitting**: 217 pages wrapped as `#[lazy_route]` structs, routes as
  `page!({ ::leptos_router::Lazy::<Lz0>::new() })` (works with leptos-routes as is). The main module's code
  shrinks from 9.5 MB to 2.6 MB, its data stays (8.5 MB). But wasm-split's chunks are fine-grained: 3,198 files, a
  page needs a median of 89 files (109 KB) and up to 511 files (1.46 MB) when first visited, plus a 2 MB loader
  script; Leptos marks the feature unstable (`__wasm_split_unstable`). Not applied; worth a follow-up with a
  coarser grouping (one lazy route per page group) and a check of the request count over HTTP/2.

## Applied changes (2026-10-07)

Library (every user profits):

- Removed `leptos_reactive` (Leptos 0.6's reactive system, only used by the dead `utils::signals` module, whose
  traits wrapped 0.6 types) and the unused dependencies `educe`, `indoc`, `leptos_meta`.
- `icondata` with only the icon sets leptonic uses (`bootstrap-icons`, `vs-code-icons`) instead of all 19: about
  24 s less CPU time per target in a fresh build, unless the app itself enables the other sets.

Book: dropped `opt-level = 3` for leptonic in the dev profile; `icondata` with the two icon sets it uses.

Wasm size (book and test app, same day):

- Dev wasm without the `name` section (`wasm-dev` profile, see the experiments): 190 MB → 51 MB.
- Book release wasm 18.18 MB → 13.01 MB (brotli 4.28 MB → 2.94 MB, −31%): ICU4X data of the book's locales
  (`examples/book-ssr/icu4x-data`, regenerated by `just book-icu-data`; the book's `.cargo/config.toml` sets
  `--cfg=icu4x_custom_data` and `ICU4X_DATA_DIR`), and syntect only on the server (`syntax-highlight` only in the
  book's `ssr` feature, `kit::code` asks the server to highlight; browser test
  `code_blocks_are_highlighted_after_client_side_navigation`). The book's clippy runs from its directory now
  (`just verify`), so that its config applies.

Result (book, dev, measured on the changed tree with `scripts/build-bench.sh`, load 6-11):

| Build (book)                       | Before                 | After                    | Change (wall / CPU) |
|------------------------------------|------------------------|--------------------------|---------------------|
| fresh                              | 127 s / 765 s          | 102.5 s / 489 s          | −19% / −36%         |
| book page edit                     | 15-18 s / 40-47 s      | 13-14.4 s / 31-36 s      | −15% / −22%         |
| leptonic hook edit                 | 36-46 s / 187-225 s    | 19-20 s / 41-48 s        | −50% / −78%         |
| no-op                              | 4 s                    | 4.3 s                    |                     |

The release build is unaffected by the profile change (the dev wasm grows from 167 MB to 190 MB: unoptimized
leptonic code, mostly a larger name section).

## Advice for users

To be turned into a guide in the book ("Build times and bundle size"):

- Keep the dev profile at `opt-level = 0` for leptonic (no per-package override); optimize only dependencies that
  rarely change if runtime speed in dev matters (`[profile.dev.package."*"] opt-level = 1`, which needs an explicit
  `[profile.dev.package.leptonic] opt-level = 0` because `"*"` matches path dependencies outside the workspace).
- `[profile.dev.package."*"] debug = false` and `debug = "line-tables-only"` for the app: smaller target
  directories and faster links (the book's debug info exceeded 4 GiB without it).
- `--cfg=erase_components` for every build (`.cargo/config.toml`), with `disable-erase-components = true` in
  cargo-leptos' metadata so that all builds share one set of flags.
- Release wasm: `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `panic = "abort"` in a `wasm-release` profile
  (`lib-profile-release`), wasm-opt via cargo-leptos. Serve it compressed (brotli: 4.3 MB instead of 18.2 MB).
- Dev wasm: a `wasm-dev` profile without symbols (`lib-profile-dev`, see the experiments): 73% smaller.
- Bake only the ICU4X locales the app supports: −20% wasm. `scripts/icu-datagen.sh` works for any app.
- Enable only the leptonic features you use: `syntax-highlight` (syntect, regex: 68 s CPU, 1.35 MB wasm) and
  `sanitize` (ammonia, html5ever: 17 s CPU, 0.5 MB wasm) are heavy. Features can differ per side: enable
  `syntax-highlight` in the app's `ssr` feature only and highlight on the server.
- Server dependencies: `axum-server` with `tls-rustls` builds `aws-lc-sys` (27 s CPU); `tower-http` with `full`
  builds zstd (11 s). Enable only what's needed.
- Large apps: split the views into several crates. Every edit re-expands all `view!` macros of the edited crate
  (3.4 s for the book's 62k lines) and type-checks its view types (16 s for a full rebuild).

## Opportunities

In leptonic:

- ICU4X `DateTimeFormatter` (any calendar) in `utils/date_time_formatter.rs` (weekday and month names) and
  `hooks/datepicker/format.rs` (`DateTimeFormatter<CompositeFieldSet>`) pulls in data and code for every calendar
  system (Buddhist, Chinese, Hebrew, Japanese, ...). The weekday/month names only ever format Gregorian dates
  (`FixedCalendarDateTimeFormatter<Gregorian, _>` would do). The date field formats in the locale's calendar like
  react-aria, so it needs the any-calendar formatter, unless that becomes a feature. Not measured yet.
- typed-builder's `PropsBuilder::build` is instantiated per combination of props a call site sets (5.5% of a user
  crate's IR, 2.4% of the wasm code). Atoms with dozens of props cost the most; fewer, grouped props (structs with
  `Default`) reduce it.
- The components layer (being removed) and its heavy optional features (`syntax-highlight`, `sanitize`, `tiptap`).
  `components::typography::Code` links syntect into every wasm built with `syntax-highlight` that uses it.
- Measured and rejected for the release wasm: `opt-level = "s"` (+11%), a newer wasm-opt (−12 KB). Debug-only
  content of the release wasm is small: panic locations are 81 KB of file paths (67 KB of them absolute paths of the
  build machine, `--remap-path-prefix` would shorten them), tracing calls a few KB.

In Leptos (upstream): typed-builder props (a non-generic `build`), the size of hydrate + build paths, and view
types whose inference dominates type checking.

## Our workflow

- The book and the test app build in separate target directories per agent group (`target/agents`); a fresh book
  build costs about 8 GB and 9-13 minutes of CPU time.
- cargo-leptos compiles every proc macro twice (server and wasm in separate target directories) and reruns
  wasm-bindgen on every build (3-4 s even when nothing changed).
