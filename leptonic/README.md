# Leptonic

⚠️⚠️ **The main branch contains unpublished changes for an upcoming v0.6 release.** ⚠️⚠️

If you are relying on this branch, you will need a patched version of leptos. Add the following to YOUR PROJECTS Cargo.toml:

```toml
[patch.crates-io]
leptos = { git = 'https://github.com/lpotthast/leptos', branch = "spread-event-handlers" }
leptos_reactive = { git = 'https://github.com/lpotthast/leptos', branch = "spread-event-handlers" }
leptos_dom = { git = 'https://github.com/lpotthast/leptos', branch = "spread-event-handlers" }
leptos_axum = { git = 'https://github.com/lpotthast/leptos', branch = "spread-event-handlers" }
leptos_meta = { git = 'https://github.com/lpotthast/leptos', branch = "spread-event-handlers" }
leptos_router = { git = 'https://github.com/lpotthast/leptos', branch = "spread-event-handlers" }
```

---

Component library for the [Leptos](https://github.com/leptos-rs/leptos) framework.

Visit [leptonic.dev](https://leptonic.dev) for installation instructions, component-guides, theming instructions and more. leptonic.dev is the deployed `book` example and itself build with leptonic.

## Browser compatibility

We expect pointer events to be supported by devices executing Leptonic code
and currently do not provide alternative code-paths for devices that doesn't.
See [https://caniuse.com/pointer](https://caniuse.com/pointer) for general availability
(currently at >97%).

No guarantees are made to be compatible with any Internet Explorer release.

## Leptos compatibility

| Crate version | Compatible Leptos version |
|-------------|-------------------------|
| 0.1         | 0.4                     |
| 0.2         | 0.4                     |
| 0.3.0-rc1   | 0.5.0-rc1               |
| 0.3, 0.4    | 0.5                     |
| 0.5, 0.6    | 0.6                     |

## MSRV

The minimum supported rust version is `1.70.0`
