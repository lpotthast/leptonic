# Leptonic Book

The documentation site of leptonic, deployed at [leptonic.dev](https://leptonic.dev). It is a Leptos app (SSR with
hydration) built with leptonic itself, and the main target for trying out library changes by hand.

## Running

From the repository root:

```bash
just serve       # https://127.0.0.1:4100 (self-signed certificate from ./certs)
```

or, in this directory, `cargo leptos serve`. The `--cfg=web_sys_unstable_apis` flag leptonic needs is set in
`.cargo/config.toml`.

The server prepares gzip and Brotli WASM files before accepting requests in every build profile, including
`just serve`'s `wasm-dev` and `server-dev`. It reuses fresh compressed files; missing or stale files are compressed
once at startup (gzip level 6, Brotli quality 4). Browsers receive their preferred encoding without compression on
each reload. No extra cargo-leptos flag or command-line compressor is needed.

## Structure

| Path                           | Content                                                                         |
|--------------------------------|---------------------------------------------------------------------------------|
| `src/routes.rs`                | All routes (`leptos-routes`).                                                    |
| `src/nav.rs`                   | The navigation: sidebar sections, concept tabs, page kinds.                     |
| `src/kit/`                     | Building blocks of pages: `DocPage`, `Section`, `ApiTable`, `Demo`, ...         |
| `src/pages/documentation/`     | The pages, by layer (`hooks/`, `atoms/`, `components/`) and by concept/domain.   |
| `src/markdown/`                | Markdown export of every page (`/doc/....md`), the LLM index and search.        |
| `style/main.scss`              | Entry stylesheet: leptonic themes, book styles (`style/book/`), demo styles.    |

How to write pages (page types, the kit, prose and demo conventions) is described in
[`documentation/documentation-strategy.md`](../../documentation/documentation-strategy.md). Open work on the book is
tracked in the repository's [`PLAN.md`](../../PLAN.md) (section "Book").

## Quality bar

Zero clippy findings (`all` and `pedantic`, see `[lints.clippy]` in `Cargo.toml`) for both the server and the WASM
build, as checked by `just verify`. Unit tests: `cargo test --features ssr --lib`.

## Deployment

`Dockerfile` builds a release image. Its build context is the repository root, as the book depends on leptonic
through a path: `docker build -f examples/book-ssr/Dockerfile .`. `precompress.sh` pre-compresses the static
assets. TLS certificates are read from `TLS_CERT_PATH` and `TLS_KEY_PATH` (default: `certs/ssl_cert.pem` and
`certs/ssl_key.pem`).
