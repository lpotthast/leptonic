use leptonic::{
    atoms::prelude::{
        Button, Dialog, Input, Link, ModalBackdrop, ModalContent, SearchField,
        SearchFieldClearButton, ShortcutKeys,
    },
    hooks::{UseGlobalShortcutsInput, use_global_shortcuts},
    utils::{
        aria::AriaHasPopup,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut},
        live_announcer::announce_polite,
        platform::device,
    },
};
use leptos::prelude::*;
use leptos_router::{
    NavigateOptions,
    hooks::{use_location, use_navigate},
};
use leptos_use::signal_debounced;

use crate::{
    kit::Icon,
    search::{SearchResult, search_docs},
};

/// Opens and closes the search anywhere on the page: Cmd+K on Apple devices, Ctrl+K elsewhere.
const TOGGLE_SEARCH: Shortcut = Shortcut::key("k").primary();

/// Queries shorter than this aren't searched.
const MIN_QUERY_LEN: usize = 2;

/// How long typing has to pause before the query is searched, in milliseconds.
const DEBOUNCE_MS: f64 = 150.0;

/// The documentation search: a button in the app bar (and [`TOGGLE_SEARCH`]) opening a dialog with a search field and
/// the matching pages. Enter opens the first match.
#[component]
pub fn DocSearch() -> impl IntoView {
    let show = RwSignal::new(false);
    let query = RwSignal::new(String::new());
    let location = use_location();

    // Also while typing, e.g. in the search field itself, which closes it again.
    use_global_shortcuts(UseGlobalShortcutsInput {
        anywhere: KeyboardShortcuts::new()
            .on(TOGGLE_SEARCH, move |_| show.update(|show| *show = !*show)),
        outside_text_fields: KeyboardShortcuts::new(),
    });

    let close = Callback::new(move |()| {
        show.set(false);
        query.set(String::new());
    });

    // Following a result navigates; the search is done then.
    Effect::watch(
        move || location.pathname.get(),
        move |_, _, _| close.run(()),
        false,
    );

    view! {
        <SearchTrigger on_press=move || show.set(true)/>

        <ModalBackdrop
            is_open=show
            set_open=move |open: bool| if open { show.set(true) } else { close.run(()) }
            is_dismissable=true
            classes="doc-search-backdrop"
        >
            <ModalContent classes="doc-search-modal">
                <Dialog aria_label="Search documentation" classes="doc-search-dialog">
                    <SearchPanel query/>
                </Dialog>
            </ModalContent>
        </ModalBackdrop>
    }
}

/// The search field and the matching pages. Created whenever the search opens.
#[component]
fn SearchPanel(query: RwSignal<String>) -> impl IntoView {
    let navigate = use_navigate();

    // Searched once typing pauses. `None`: the query is too short to search.
    let searched_query: Signal<String> = signal_debounced(query, DEBOUNCE_MS);
    let results = Resource::new(
        move || searched_query.get(),
        |query| async move {
            if query.trim().len() < MIN_QUERY_LEN {
                return None;
            }
            Some(search_docs(query).await)
        },
    );

    // Enter opens the first result. Escape empties the field, a second Escape closes the search.
    let open_first = Callback::new(move |_| {
        if let Some(first) = results
            .get_untracked()
            .flatten()
            .and_then(Result::ok)
            .and_then(|results| results.into_iter().next())
        {
            navigate(&first.path, NavigateOptions::default());
        }
    });

    view! {
        <div class="doc-search-header">
            <SearchField
                value=query set_value=query
                on_submit=open_first
                aria_label="Search documentation"
                placeholder="Search documentation\u{2026}"
                classes="doc-search-field"
            >
                <Icon icon=icondata::BsSearch classes="doc-search-field-icon"/>
                <Input classes="doc-search-input"/>
                <SearchFieldClearButton classes="doc-search-clear">
                    <Icon icon=icondata::BsXLg/>
                </SearchFieldClearButton>
            </SearchField>
        </div>
        <div class="doc-search-body">
            <Suspense fallback=move || view! { <p class="doc-search-status">"Searching\u{2026}"</p> }>
                {move || Suspend::new(async move {
                    let Some(results) = results.await else {
                        return ().into_any();
                    };
                    let Ok(results) = results else {
                        announce_polite("Search failed");
                        return view! {
                            <p class="doc-search-status" data-error="">
                                "The search failed. Check your connection and try again."
                            </p>
                        }
                        .into_any();
                    };
                    announce_polite(match results.len() {
                        0 => "No results".to_owned(),
                        1 => "1 result".to_owned(),
                        n => format!("{n} results"),
                    });
                    if results.is_empty() {
                        return view! { <p class="doc-search-status">"No results found."</p> }.into_any();
                    }
                    let terms = searched_query.get_untracked();
                    view! {
                        <ul class="doc-search-results" aria-label="Results">
                            {results
                                .into_iter()
                                .map(|result| view! { <li><SearchResultLink result terms=terms.clone()/></li> })
                                .collect_view()}
                        </ul>
                    }
                    .into_any()
                })}
            </Suspense>
        </div>
    }
}

/// A result: the page title (and the matching section), and a plain-text snippet. Occurrences of the query are
/// highlighted.
#[component]
fn SearchResultLink(result: SearchResult, terms: String) -> impl IntoView {
    let SearchResult {
        path,
        title,
        section,
        snippet,
    } = result;
    // `Link` may render its content more than once: the parts are cloned for each.
    view! {
        <Link href=path classes="doc-search-result">
            <span class="doc-search-result-title">
                <Highlighted text=title.clone() terms=terms.clone()/>
                {section.clone().map(|section| view! {
                    <span class="doc-search-result-section">
                        " \u{203a} "<Highlighted text=section terms=terms.clone()/>
                    </span>
                })}
            </span>
            <span class="doc-search-result-snippet">
                <Highlighted text=snippet.clone() terms=terms.clone()/>
            </span>
        </Link>
    }
}

/// `text` with the occurrences of `terms` (case-insensitive) in `<mark>`s.
#[component]
fn Highlighted(text: String, terms: String) -> impl IntoView {
    highlight(&text, terms.trim())
        .into_iter()
        .map(|(part, matches)| {
            if matches {
                view! { <mark class="doc-search-mark">{part}</mark> }.into_any()
            } else {
                part.into_any()
            }
        })
        .collect_view()
}

/// Splits `text` into parts that are (`true`) or aren't occurrences of `terms`, ignoring case.
fn highlight(text: &str, terms: &str) -> Vec<(String, bool)> {
    let fold = |c: char| c.to_lowercase().next().unwrap_or(c);
    let terms: Vec<char> = terms.chars().map(fold).collect();
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let mut parts = Vec::new();
    if terms.is_empty() {
        parts.push((text.to_owned(), false));
        return parts;
    }
    let (mut part_start, mut i) = (0, 0);
    while i + terms.len() <= chars.len() {
        let matches = chars[i..i + terms.len()]
            .iter()
            .zip(&terms)
            .all(|((_, c), term)| fold(*c) == *term);
        if matches {
            let start = chars[i].0;
            let end = chars.get(i + terms.len()).map_or(text.len(), |(at, _)| *at);
            if start > part_start {
                parts.push((text[part_start..start].to_owned(), false));
            }
            parts.push((text[start..end].to_owned(), true));
            part_start = end;
            i += terms.len();
        } else {
            i += 1;
        }
    }
    if part_start < text.len() {
        parts.push((text[part_start..].to_owned(), false));
    }
    parts
}

/// The app bar button opening the search, showing [`TOGGLE_SEARCH`] in the keys of the reader's platform.
#[component]
fn SearchTrigger(on_press: impl Fn() + Send + Sync + 'static) -> impl IntoView {
    // `aria-keyshortcuts` names the generic form (Control) on the server and in the first client render; switched
    // after hydration (as `ShortcutKeys` switches its keys), so that server and client markup match.
    let apple = RwSignal::new(false);
    Effect::new(move |_| apple.set(device::is_apple_device()));

    view! {
        // The name contains the visible text (which small screens hide); the shortcut is announced through
        // `aria-keyshortcuts` instead of the key caps.
        <Button
            on_press=move |_| on_press()
            aria_haspopup=Some(AriaHasPopup::Dialog)
            aria_label="Search docs"
            classes="doc-search-trigger"
            attr:aria-keyshortcuts=move || TOGGLE_SEARCH.to_aria_keyshortcuts(apple.get())
        >
            <Icon icon=icondata::BsSearch/>
            <span class="doc-search-trigger-text">"Search docs\u{2026}"</span>
            <span aria-hidden="true" class="doc-search-trigger-keys">
                <ShortcutKeys shortcut=TOGGLE_SEARCH/>
            </span>
        </Button>
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::highlight;

    #[test]
    fn highlights_occurrences_ignoring_case() {
        assert_that!(highlight("Press a button, press it", "press")).is_equal_to(vec![
            ("Press".to_owned(), true),
            (" a button, ".to_owned(), false),
            ("press".to_owned(), true),
            (" it".to_owned(), false),
        ]);
        assert_that!(highlight("Größe", "ö")).is_equal_to(vec![
            ("Gr".to_owned(), false),
            ("ö".to_owned(), true),
            ("ße".to_owned(), false),
        ]);
        assert_that!(highlight("abc", "")).is_equal_to(vec![("abc".to_owned(), false)]);
    }
}
