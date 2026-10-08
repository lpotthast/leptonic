use indoc::indoc;
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageSsr() -> impl IntoView {
    view! {
        <DocPage title="Server-Side Rendering">
            <p>
                "Leptonic works in apps rendered on the server and hydrated in the browser (SSR) as well as in client-side "
                "rendered apps. With SSR, the server sends complete HTML, including roles, ARIA attributes, labels and ids, "
                "so the page is readable before its WebAssembly has loaded; once it hydrates, leptonic attaches its behavior. "
                "This book is rendered that way. This guide explains what leptonic does on each side and what your own code "
                "has to watch out for."
            </p>

            <Section title="Features">
                <p>
                    "Forward your app\u{2019}s "<Code inline=true>"ssr"</Code>" and "<Code inline=true>"hydrate"</Code>
                    " features to leptonic\u{2019}s features of the same name (see "
                    <Link href=routes::doc::Installation.materialize()>"Installation"</Link>"). A client-side rendered app "
                    "enables neither."
                </p>
                <Code language=Language::Toml>
                    {indoc!(r#"
                        [features]
                        hydrate = ["leptos/hydrate", "leptonic/hydrate"]
                        ssr = ["leptos/ssr", "leptonic/ssr"]
                    "#)}
                </Code>
                <DocTable headers=&["Feature", "Enables"]>
                    <TableRow>
                        <TableCell><Code inline=true>"ssr"</Code></TableCell>
                        <TableCell>
                            "The server side of Leptos, leptos-use and leptos-element-capture. Leptonic leaves out code that only "
                            "makes sense in a browser, such as its document-wide focus tracking."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"hydrate"</Code></TableCell>
                        <TableCell>"The hydrating client of Leptos and of leptonic\u{2019}s element references."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="What Runs Where">
                <p>
                    "The bodies of hooks, atoms and your own Leptos components run on the server and again in the browser, where "
                    "hydration creates every component a second time and attaches it to the server\u{2019}s HTML. Effects, event "
                    "listeners, timers and animation frames only run in the browser. Leptonic follows that split:"
                </p>
                <ul>
                    <li>
                        "Everything a hook computes from its input is rendered on the server: roles, "
                        <Code inline=true>"aria-*"</Code>" attributes, ids and the references between them, "
                        <Code inline=true>"tabindex"</Code>", initial state such as "<Code inline=true>"aria-expanded"</Code>
                        " or "<Code inline=true>"aria-selected"</Code>"."
                    </li>
                    <li>
                        "Everything that reacts to the user starts after hydration: press, hover and focus handling, "
                        "keyboard navigation, and the "<Code inline=true>"data-pressed"</Code>", "
                        <Code inline=true>"data-hovered"</Code>" and "<Code inline=true>"data-focus-visible"</Code>
                        " attributes that reflect it."
                    </li>
                    <li>
                        "Page-wide state, such as the stack of open overlays, the focus scope tree or the prevent-scroll "
                        "count, is only touched from browser-only code. On the server, many requests share a thread, and a "
                        "request\u{2019}s render can move between threads, so such state would mix requests."
                    </li>
                </ul>
            </Section>

            <Section title="Stable Ids">
                <p>
                    "Hooks link elements through ids: a label through "<Code inline=true>"aria-labelledby"</Code>", a "
                    "popover through "<Code inline=true>"aria-controls"</Code>", a field through "
                    <Code inline=true>"for"</Code>". The ids must be the same in the server\u{2019}s HTML and in the hydrated "
                    "page, or the references point nowhere. Leptonic therefore creates them with "
                    <Code inline=true>"leptonic::utils::id::use_id"</Code>", which counts with Leptos\u{2019} hydration "
                    "counter: because the server and the client create components in the same order, they produce the same "
                    "ids. Ids created after hydration (and in client-side rendered apps) carry a "<Code inline=true>"c"</Code>
                    " marker (\u{201c}listbox-c3\u{201d}), so they never collide with server ids."
                </p>
                <p>
                    "This only holds while the server and the hydrating client render the same tree. Render the same "
                    "components on both sides during hydration, and branch on browser-only information (the window size, "
                    "local storage) only in effects or after hydration. Use "<Code inline=true>"use_id"</Code>
                    " for ids in your own hooks and components as well, called unconditionally while the component is "
                    "created, never in an effect, an event handler or code that only exists on one side:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::utils::id::use_id;

                        #[component]
                        fn Hint(children: Children) -> impl IntoView {
                            let id = use_id("hint"); // "hint-12" on the server and while hydrating
                            view! {
                                <input aria-describedby=id.clone()/>
                                <p id=id>{children()}</p>
                            }
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Browser Access">
                <p>
                    "There is no "<Code inline=true>"window"</Code>" or "<Code inline=true>"document"</Code>" on the server, "
                    "and "<Code inline=true>"web_sys::window()"</Code>" panics there. Leptonic accesses both through "
                    <Code inline=true>"use_window()"</Code>" and "<Code inline=true>"use_document()"</Code>" from leptos-use, "
                    "which return "<Code inline=true>"None"</Code>" in a build with the "<Code inline=true>"ssr"</Code>
                    " feature. Do the same in your code:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptos_use::use_window;

                        let width = use_window()
                            .as_ref()
                            .and_then(|window| window.inner_width().ok())
                            .and_then(|width| width.as_f64());
                    ")}
                </Code>
                <p>
                    "Values that depend on the browser fall back on the server. For example, the platform checks of "
                    <Code inline=true>"leptonic::utils::platform"</Code>" ("<Code inline=true>"device::is_mac"</Code>", "
                    <Code inline=true>"browser::is_safari"</Code>", \u{2026}) are "<Code inline=true>"false"</Code>
                    " there, as they read the browser\u{2019}s navigator."
                </p>
            </Section>

            <Section title="Internationalization">
                <p>
                    "Leptonic formats and compares text with "
                    <Link href="https://github.com/unicode-org/icu4x" target=LinkTarget::Blank>"ICU4X"</Link>
                    ", the Unicode Consortium\u{2019}s internationalization library, compiled into your app with its locale "
                    "data. It needs no JavaScript "<Code inline=true>"Intl"</Code>" API, which doesn\u{2019}t exist on the server: "
                    "number fields and sliders render their formatted values on the server exactly as in the browser, and "
                    "combobox filtering and type-to-select in collections compare text the same way on both sides. The "
                    "modules are public, for your own formatting:"
                </p>
                <DocTable headers=&["Module", "Provides"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::utilities::NumberFormatter.materialize()><Code inline=true>"utils::number_formatter"</Code></Link></TableCell>
                        <TableCell>"Locale-aware number formatting"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::utilities::DateTimeFormatter.materialize()><Code inline=true>"utils::date_time_formatter"</Code></Link></TableCell>
                        <TableCell>"Locale-aware date and time formatting"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::utilities::ListFormatter.materialize()><Code inline=true>"utils::list_formatter"</Code></Link></TableCell>
                        <TableCell>"Lists such as \u{201c}A, B, and C\u{201d}"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"utils::plurals"</Code></TableCell>
                        <TableCell>"Plural categories for labels"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::utilities::Collator.materialize()><Code inline=true>"utils::filter"</Code></Link></TableCell>
                        <TableCell>"Locale-aware string comparison and filtering, e.g. in comboboxes"</TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "The locale comes from the nearest "
                    <Link href=routes::doc::utilities::I18nProvider.materialize()><Code inline=true>"I18nProvider"</Code></Link>
                    " ("<Code inline=true>"leptonic::utils::i18n"</Code>"), \u{201c}en-US\u{201d} without one, never from the "
                    "browser. If you choose the locale per request, e.g. from a cookie, make sure the hydrating client starts "
                    "with the same locale as the server, or formatted text differs between the two."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::utils::i18n::{I18nProvider, Locale, locale};

                        view! {
                            <I18nProvider locale=Locale::from(locale!("de-DE"))>
                                <App/>
                            </I18nProvider>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="What Appears After Hydration">
                <p>
                    "Some parts of the page can\u{2019}t be rendered on the server and are added once the page runs in the "
                    "browser:"
                </p>
                <DocTable headers=&["What", "Why"]>
                    <TableRow>
                        <TableCell>"Overlays: popovers, modals, tooltips"</TableCell>
                        <TableCell>
                            "They render through Leptos\u{2019} "<Code inline=true>"Portal"</Code>", which mounts its content "
                            "into "<Code inline=true>"<body>"</Code>" in the browser only. An overlay that is open on the "
                            "first render appears after hydration."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Hidden description elements"</TableCell>
                        <TableCell>
                            "Some hooks describe an element with text that isn\u{2019}t on the page, e.g. how to long-press a "
                            "button. The hidden element holding that text, and the "<Code inline=true>"aria-describedby"</Code>
                            " pointing at it, are added after mount."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"References to elements leptonic doesn\u{2019}t render"</TableCell>
                        <TableCell>
                            "A "<Link href=routes::doc::disclosure::Atom.materialize()>"disclosure"</Link>"\u{2019}s panel is "
                            "named by its trigger, a dialog without a title by the button that opened it. Those elements may "
                            "carry an id of your own, so leptonic reads (or sets) the id once the element exists and only then "
                            "adds the "<Code inline=true>"aria-labelledby"</Code>"."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Live regions"</TableCell>
                        <TableCell>
                            "The "<Link href=routes::doc::screen_readers::LiveAnnouncer.materialize()>"live announcer"</Link>
                            " creates its regions on the first announcement; announcing does nothing on the server."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"The stored theme"</TableCell>
                        <TableCell>
                            "A theme remembered with "<Code inline=true>"signal_ls"</Code>" lives in local storage, which "
                            "the server can\u{2019}t read: the server renders the default theme, and the page switches to the "
                            "stored theme when it hydrates. Keep the theme in a cookie to render it on the server (see "
                            <Link href=format!("{}#remembering-the-theme-on-the-server", routes::doc::Themes.materialize())>"Themes"</Link>")."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Writing Your Own Hooks">
                <p>"Code built on leptonic\u{2019}s hooks follows the same rules as leptonic itself:"</p>
                <ul>
                    <li>"Access the browser through "<Code inline=true>"use_window()"</Code>" and "<Code inline=true>"use_document()"</Code>", or from effects and event handlers."</li>
                    <li>"Create ids with "<Code inline=true>"use_id"</Code>", in the same order on both sides."</li>
                    <li>
                        "Keep state of one request in the reactive owner ("<Code inline=true>"provide_context"</Code>
                        "), not in "<Code inline=true>"thread_local!"</Code>"s or statics. Touch page-wide state only from "
                        "effects, event handlers or code behind "<Code inline=true>"#[cfg(not(feature = \"ssr\"))]"</Code>"."
                    </li>
                    <li>
                        "Prefer thread-safe storage ("<Code inline=true>"StoredValue::new"</Code>", "
                        <Code inline=true>"RwSignal::new"</Code>") for values created while rendering: a value in local "
                        "storage ("<Code inline=true>"new_local"</Code>") can be dropped on another thread on the server, "
                        "which panics."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Installation.materialize()>"Installation"</Link></li>
                <li><Link href=routes::doc::Architecture.materialize()>"Hooks & Atoms"</Link></li>
                <li><Link href=routes::doc::Themes.materialize()>"Themes"</Link></li>
                <li><Link href=routes::doc::Accessibility.materialize()>"Accessibility"</Link></li>
                <li><Link href=routes::doc::screen_readers::LiveAnnouncer.materialize()>"Live Announcer"</Link></li>
                <li><Link href=routes::doc::utilities::I18nProvider.materialize()>"I18nProvider"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
