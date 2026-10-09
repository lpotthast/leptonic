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
  free with APFS clones (`cp -c -R target/a target/b`; btrfs/XFS on Linux: `cp --reflink=always -R`); cargo
  considers the copy fresh. Good for starting an experiment from a built state.
- `scripts/build-bench.sh` edits files with macOS' `sed -i ''`: its `book-edit` and `lib-edit` scenarios need
  macOS (GNU sed reads `''` as the script).
- Experiments with manifests or profiles: use a copy of the app outside the repository whose `src` is a symlink to
  the live sources (copy `Cargo.toml`, `Cargo.lock`, `style/`, `public/`, `icu4x-data/`, the repository's and the
  app's `.cargo/config.toml` merged into one (rustflags, `LEPTOS_OUTPUT_NAME`, `ICU4X_DATA_DIR`), and make the
  `leptonic` path absolute), so the shared working tree is never touched. Profile settings can also be passed as
  `--config 'profile.dev.package.leptonic.opt-level=0'`.
- Where a crate's compile time goes: `RUSTC_BOOTSTRAP=1 cargo rustc -p <crate> --lib ... -- -Ztime-passes`. Note:
  `RUSTC_BOOTSTRAP` is part of cargo's fingerprint, the first such build recompiles every dependency; the next one
  (same environment) shows the incremental case. `cargo build --timings` shows the units and the critical path.
- Which generics a crate instantiates: `cargo llvm-lines -p book-ssr --lib --target wasm32-unknown-unknown
  --no-default-features --features hydrate`, run in the book's directory (its `.cargo/config.toml` sets
  `LEPTOS_OUTPUT_NAME` and the ICU4X data); aggregate the output by function name with the generic arguments
  stripped.
- Wasm sizes: report raw, gzip -9 and brotli -q 11 (what a browser downloads). `twiggy top` on the wasm *before*
  wasm-bindgen (`<target>/front/wasm32-unknown-unknown/<profile>/book_ssr.wasm`, which still has the function
  names) attributes code to functions. Data (strings, tables) can't be attributed in an LTO build (all symbols are
  anonymous): link a non-LTO release build with a map (`cargo rustc ... --config 'profile.wasm-release.lto=false'
  -- -C link-arg=--Map=<file>`) and sum the `.rodata` input sections per object file (one per crate).

Machine of the numbers below unless marked otherwise (e.g. "Linux"): Apple M4 Max (16 cores, 64 GB), rustc 1.99.0,
cargo-leptos 0.3.11, wasm-bindgen 0.2.129, wasm-opt version_123 (cargo-leptos' download), leptos 0.8.19.

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
| + `opt-level = "z"` for the wasm (`wasm-dev`; 2026-10-08)         |                  | 12-14 s wasm (was 17-18 s) | ~28 s wasm (same) | 19.7 MB (was 43.8 MB) | **applied** |
| + `opt-level = 1` for the server (`server-dev`; 2026-10-08)       |                  | 79 s server (was 89 s) | 41 s server (was 56 s) | | **applied** |

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
- `opt-level = 1` for the server in dev (`bin-profile-dev = "server-dev"`, 2026-10-08; server builds only, other
  builds running, so ±10 s): server-side rendering of the book's 124 pages takes 51 ms per page instead of 82 ms, and
  rebuilds are faster, not slower (less code to generate and link). The cold build takes 112 s instead of 82 s, once.
  The test-app gained more (`server-release`: 6x faster rendering, its calendar page 87 ms instead of 550 ms): the
  book's requests also pay for TLS and compressing the HTML.
- The book serves its wasm, JS and CSS with content-hashed names (`hash-files`, `HashedStylesheet`) and
  `cache-control: public, max-age=31536000, immutable` for `/pkg/` (2026-10-08): browsers keep them, and the code
  compiled from the wasm, until a build changes them. Its Docker image copies `hash.txt` next to the server binary
  and sets `LEPTOS_HASH_FILES`.
- `opt-level = "z"` for `wasm-dev` (2026-10-08; wasm build + wasm-bindgen only, timed with the server not built, the
  machine busy with other builds, so ±5 s): a book edit rebuilds the wasm in 12-14 s instead of 17-18 s (wasm-bindgen:
  1.2 s instead of 3.7 s), a leptonic edit in 23-34 s either way (averages 27.5 s and 28.8 s). Pages load faster (the
  browser tests: 500 ms page loads with 44 MB). Needs `[profile.dev.package."*"]` without an `opt-level`, which would
  override the profile's for every dependency.

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
  of the same minor version as the `icu_*` crates (`scripts/icu-datagen.sh` checks it):
  `icu4x-datagen --format baked --markers all --segmenter-models none --locales <...> --out <dir>`. `--markers all`
  is required: `--markers-for-bin` leaves out markers the ICU crates reference at compile time (errors like "cannot
  find macro `impl_normalizer_nfc_v1`"). Without `--segmenter-models none` the directory is 17-18 MB of Rust
  sources, mostly segmenter dictionaries; with it 6.2 MB, same wasm (the linker drops unused markers). The data
  breaks the build when the `icu_*` crates move to another minor version (2.2 data with the 2.3 crates: "cannot find
  `provider` in `locale`", the markers moved to `icu_locale_fallback`): regenerate it with every ICU4X update.
  Data is decided by the app, leptonic needs no change; unsupported locales fall back to their parent or the root
  locale. Combined with route splitting the main module would be about 6.5 MB (wasm-split keeps all data in the main
  module).
- **syntect on the client**: anything that calls leptonic's highlighter (then `utils::syntax_highlight`, today
  `leptonic::highlight_to_classed_html`) in the wasm links syntect, its syntax definitions and regex (1.35 MB raw,
  0.58 MB brotli: the definitions barely compress). Leaving the call out wasn't enough while leptonic's
  `components::typography::Code` existed: it highlighted on the client whenever the feature was on, whatever its
  `language` (the components layer is gone since 2026-10-07; now only an app's own calls link syntect). So the book
  enables `syntax-highlight` only in its `ssr` feature and its code blocks ask the server (`kit::code`'s `highlight`
  server function) for pages the client renders itself.
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
  24 s less CPU time per target in a fresh build, unless the app itself enables the other sets. (Later that day
  leptonic dropped `icondata`, `ammonia` (html5ever) and `leptos-tiptap` altogether, with the components layer;
  the book keeps the two icon sets for itself.)

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

## Applied changes (2026-10-09): test-app

- **ICU4X data of the fixtures' locales** (`testing/test-app/icu4x-data`, `^en ^de ^fr ^ja ^he ^ar ^ar-AE ^ar-EG`,
  regenerated by `just test-app-icu-data`; the test-app's `.cargo/config.toml` sets `--cfg=icu4x_custom_data` and
  `ICU4X_DATA_DIR` for server and wasm alike): release wasm 13.91 MB → 10.00 MB (−28%; brotli 3.04 MB → 2.27 MB).
  Page loads in the browser suite: 110 ms → 96 ms on average (955 and 979 loads, two full runs of the same day).
  A fixture formatting with another locale needs the data regenerated (otherwise it silently falls back to the
  parent's or the root's data).
- **Per-section fixtures** (the test-app's `Section`, `?only=<name>,...`): measured server-side rendering per fixture
  (best of 5, `server-release`): the calendar page took 53 ms for 321 KB of HTML (29 calendars), every other fixture
  at most 6 ms (table-tree 69 KB, date-picker and table-navigation 50 KB). Page loads in the suite (step log): the
  whole calendar page 479 ms, one calendar section 104 ms (median of 45 loads, one per calendar case); date-picker
  152 ms → 79 ms (15 loads). Sections for the calendar, date-picker and date-field fixtures; the others are not worth
  it.
- **Shorter real timers** in the toast, tooltip and long-press fixtures: `toast::timeouts` 10.1 s → 4.0 s,
  `toast::remaining_time_after_pause` 5.9 s → 4.6 s, `tooltip::close_on_press_disabled_and_close_delay` 2.6 s →
  1.6 s.
- Together (with the other agents' changes of the same day): the full suite 44.8 s → 35.8 s (822 → 871 tests, test
  bodies 5m 11s → 4m 25s, page loads 110 ms → 83 ms on average).

## Advice for users

The book's guide "Optimizing Compile Times & Binary Sizes" (`/doc/optimizing-builds`) turns these, and the
2026-10-08 test-app measurements (release profile without LTO, server at `opt-level = 1`, hashed files, linkers), into
copyable configuration. Keep the two in step:

- No `opt-level` override for leptonic (`[profile.dev.package.leptonic]`) when editing it (path dependency): at 3
  every leptonic edit took twice as long. No `opt-level` in `[profile.dev.package."*"]`: profiles inheriting dev
  (`wasm-dev`, `server-dev`) inherit it, and it overrides their own for every dependency. For a faster dev server:
  a `server-dev` profile (`bin-profile-dev`, inherits dev, `opt-level = 1`): test app 2026-10-08, rendering 6x
  faster (calendar page 87 ms instead of 550 ms), rebuild after a leptonic change 24 s (dev: 26 s).
- `[profile.dev.package."*"] debug = false` and `debug = "line-tables-only"` for the app: smaller target
  directories and faster links (the book's debug info exceeded 4 GiB without it).
- `--cfg=erase_components` for every build (`.cargo/config.toml`), with `disable-erase-components = true` in
  cargo-leptos' metadata so that all builds share one set of flags.
- Release wasm (test app, 2026-10-08): `opt-level = "z"`, no LTO, default codegen units, `incremental = true` in a
  `wasm-release` profile (`lib-profile-release`): 13 MB (dev 33 MB), page loads ~180 ms instead of ~450 ms, rebuild
  after a leptonic change ~20 s (as fast as dev). `lto = true` + `codegen-units = 1` saved 1 MB but made that
  rebuild 84 s: production builds only (`CARGO_PROFILE_WASM_RELEASE_LTO=true`, `..._CODEGEN_UNITS=1`). Opt-levels
  1/2/3/`"s"` were close to `"z"` in speed, `"z"` smallest. Serve it compressed (book: brotli 2.94 MB of 13.01 MB);
  cargo-leptos' release-only `--precompress` (brotli 11) costs ~20 s per build on a 13 MB wasm. The book also
  precompresses at server startup in every profile (`src/assets.rs`, gzip 6 / Brotli 4), reusing fresh sidecars:
  `just serve` delivers compressed WASM without paying for compression on each reload. `hash-files = true` + `HashedStylesheet` +
  `cache-control: public, max-age=31536000, immutable` for `/pkg/` lets browsers cache the wasm and its compiled code.
- Linkers: rust-lld is the default on x86_64 Linux since Rust 1.90; mold linked the test server in 0.75 s instead of
  1.2 s (not worth extra setup for leptonic's own builds). On macOS the book links with lld
  (`-C link-arg=-fuse-ld=lld` for `aarch64-apple-darwin` in its `.cargo/config.toml`).
- Dev wasm: a `wasm-dev` profile (`lib-profile-dev`, see the experiments) without symbols (73% smaller) and at
  `opt-level = "z"` (book: 43.8 MB → 19.7 MB, rebuilds no slower).
- Bake only the ICU4X locales the app supports: −20% wasm. `scripts/icu-datagen.sh` works for any app.
- Enable only the leptonic features you use (`default-features = false`; the default is `intl-strings`, the hooks'
  messages in react-aria's 34 locales: an English-only app can leave it out, saving wasm size, not measured yet).
  `syntax-highlight` (syntect, regex: 68 s CPU, 1.35 MB wasm) is heavy. Features can differ per side: enable
  `syntax-highlight` in the app's `ssr` feature only and highlight on the server.
- Server dependencies: `axum-server` with `tls-rustls` builds `aws-lc-sys` (27 s CPU); `tower-http` with `full`
  builds zstd (11 s). Enable only what's needed. The book uses `tls-rustls-no-provider` plus `rustls` with only `ring`,
  installed in `main` (`rustls::crypto::ring::default_provider().install_default()`): rustls alone builds in 6.2 s
  CPU with `ring` against 29.5 s with `aws-lc-rs` (2026-10-07, Linux, debug), and the book's server no longer
  builds `aws-lc-sys`.
- Large apps: split the views into several crates. Every edit re-expands all `view!` macros of the edited crate
  (3.4 s for the book's 62k lines) and type-checks its view types (16 s for a full rebuild).

## Opportunities

In leptonic:

- ICU4X's any-calendar `DateTimeFormatter<CompositeFieldSet>` in `utils/date_time_formatter.rs` (leptonic's
  `DateTimeFormatter`) and `hooks/datepicker/format.rs` pulls in data and code for every calendar system (Buddhist,
  Chinese, Hebrew, Japanese, ...). `utils/date_time_formatter.rs` only ever formats Gregorian dates (it sets the
  Gregorian calendar; its single fields already use `FixedCalendarDateTimeNames<Gregorian, _>`), so
  `FixedCalendarDateTimeFormatter<Gregorian, _>` would do. The date field formats in the locale's calendar like
  react-aria, so it needs the any-calendar formatter, unless that becomes a feature. Not measured yet (`PLAN.md`).
- typed-builder's `PropsBuilder::build` is instantiated per combination of props a call site sets (5.5% of a user
  crate's IR, 2.4% of the wasm code). Atoms with dozens of props cost the most; fewer, grouped props (structs with
  `Default`) reduce it (`PLAN.md`).
- `().into_any()` early returns for a missing context force atoms' views into `AnyView` (55 of them, `PLAN.md`,
  "wasm size"): returning `Option<impl IntoView>` keeps them typed. Not measured yet.
- Measured and rejected for the release wasm: `opt-level = "s"` (+11%), a newer wasm-opt (−12 KB). Debug-only
  content of the release wasm is small: panic locations are 81 KB of file paths (67 KB of them absolute paths of the
  build machine, `--remap-path-prefix` would shorten them), tracing calls a few KB.

In Leptos (upstream): typed-builder props (a non-generic `build`), the size of hydrate + build paths, and view
types whose inference dominates type checking.

## Our workflow

- Agents share one target directory per project: `target/agents` (leptonic, leptonic-theme),
  `examples/book-ssr/target/agents` and `testing/test-app/target/agents` (the browser suite's
  `TEST_APP_TARGET_DIR`), all with the repository's rustflags; the user's builds use their own. A fresh book build
  costs about 8 GB and 9-13 minutes of CPU time; the incremental caches grow by tens of GB a day
  (`just clean-agent-incremental`).
- The test-app is only built with `--release` (`wasm-release`: `opt-level = "z"`, no LTO, incremental, debug
  assertions; server `server-release`: dev at `opt-level = 1`); the book's `just serve` uses `wasm-dev` and
  `server-dev`, its release wasm keeps fat LTO, one codegen unit and `panic = "abort"`.
- cargo-leptos compiles every proc macro twice (server and wasm in separate target directories) and reruns
  wasm-bindgen on every build (3-4 s even when nothing changed).
