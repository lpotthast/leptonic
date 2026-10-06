use leptonic::{
    components::prelude::*,
    hooks::{TextFieldState, UseTextFieldStateInput, use_text_field_state},
    prelude::*,
    utils::{
        key::KeyboardKey, keyboard_shortcut::Shortcut, live_announcer::announce_polite,
        platform::device,
    },
};
use leptos::{ev::keydown, prelude::*};
use leptos_router::{
    NavigateOptions,
    hooks::{use_location, use_navigate},
};
use leptos_use::{use_event_listener, use_window};

use crate::search::{SearchResult, search_docs};

/// Opens and closes the search anywhere on the page: Cmd+K on Apple devices, Ctrl+K elsewhere.
const TOGGLE_SEARCH: Shortcut = Shortcut::key("k").primary();

/// Queries shorter than this aren't searched.
const MIN_QUERY_LEN: usize = 2;

/// The documentation search: a button in the app bar (and [`TOGGLE_SEARCH`]) opening a dialog with a search field and
/// the matching pages. Enter opens the first match.
#[component]
pub fn DocSearch() -> impl IntoView {
    let show = RwSignal::new(false);
    let query = use_text_field_state(UseTextFieldStateInput::default());
    let location = use_location();

    let _ = use_event_listener(use_window(), keydown, move |e| {
        if TOGGLE_SEARCH.matches(&e) {
            e.prevent_default();
            show.update(|show| *show = !*show);
        }
    });

    let close = Callback::new(move |()| {
        show.set(false);
        query.set_value(String::new());
    });

    // Following a result navigates; the search is done then.
    Effect::watch(
        move || location.pathname.get(),
        move |_, _, _| close.run(()),
        false,
    );

    view! {
        <SearchTrigger on_press=move || show.set(true)/>

        <Modal
            state=ValueBinding::new(
                show.into(),
                Callback::new(move |open| if open { show.set(true) } else { close.run(()) }),
            )
            aria_label="Search documentation" classes="doc-search-modal">
            <SearchPanel query/>
        </Modal>
    }
}

/// The search field and the matching pages. Created whenever the search opens.
#[component]
fn SearchPanel(query: TextFieldState) -> impl IntoView {
    let navigate = use_navigate();

    let results = Resource::new(
        move || query.value.get(),
        |query| async move {
            if query.len() < MIN_QUERY_LEN {
                return Vec::new();
            }
            search_docs(query).await.unwrap_or_default()
        },
    );

    // Enter opens the first result. Escape empties the field, a second Escape closes the search.
    let open_first = Callback::new(move |_| {
        if let Some(first) = results
            .get_untracked()
            .and_then(|results| results.into_iter().next())
        {
            navigate(&first.path, NavigateOptions::default());
        }
    });

    view! {
        <ModalHeader>
            <SearchField
                state=query
                on_submit=open_first
                aria_label="Search documentation"
                placeholder="Search documentation\u{2026}"
                classes="doc-search-field"
            />
        </ModalHeader>
        <ModalBody>
            <Suspense fallback=move || view! { <p class="doc-search-status">"Searching\u{2026}"</p> }>
                {move || Suspend::new(async move {
                    let results: Vec<SearchResult> = results.await;
                    let searched = query.value.get_untracked().len() >= MIN_QUERY_LEN;
                    if searched {
                        announce_polite(match results.len() {
                            0 => "No results".to_owned(),
                            1 => "1 result".to_owned(),
                            n => format!("{n} results"),
                        });
                    }
                    if results.is_empty() {
                        return searched
                            .then(|| view! { <p class="doc-search-status">"No results found."</p> })
                            .into_any();
                    }
                    view! {
                        <ul class="doc-search-results" aria-label="Results">
                            {results
                                .into_iter()
                                .map(|result| view! {
                                    <li>
                                        <Link href=result.path classes="doc-search-result">
                                            <span class="doc-search-result-title">{result.title}</span>
                                            <span class="doc-search-result-snippet">{result.snippet}</span>
                                        </Link>
                                    </li>
                                })
                                .collect_view()}
                        </ul>
                    }
                    .into_any()
                })}
            </Suspense>
        </ModalBody>
    }
}

/// The app bar button opening the search, showing [`TOGGLE_SEARCH`] with the platform's primary modifier.
#[component]
fn SearchTrigger(on_press: impl Fn() + Send + Sync + 'static) -> impl IntoView {
    // Rendered with Ctrl on the server; switched after hydration, so that server and client markup match.
    let primary = RwSignal::new(KeyboardKey::Control);
    Effect::new(move |_| {
        if device::is_apple_device() {
            primary.set(KeyboardKey::Command);
        }
    });

    view! {
        <Button
            on_press=move |_| on_press()
            variant=ButtonVariant::Outlined
            color=ButtonColor::Secondary
            aria_haspopup=Some(AriaHasPopup::Dialog)
            classes="doc-search-trigger"
            attr:aria-label="Search documentation"
        >
            <Icon icon=icondata::BsSearch/>
            <span class="doc-search-trigger-text">"Search docs\u{2026}"</span>
            {move || view! { <KbdShortcut keys=[primary.get(), KeyboardKey::K] classes="doc-search-trigger-kbd"/> }}
        </Button>
    }
}
