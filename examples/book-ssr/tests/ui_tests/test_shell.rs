//! Checks the book's shell: the documentation search, the theme toggle, the small-screen menu, the page structure,
//! narrow screens and the demo source toggle; code blocks, fonts and key caps.

use std::time::Duration;

use assertr::prelude::*;
use browser_test::thirtyfour::{By, Key, WebElement};
use leptos_browser_test::{Report, ResultExt};

use super::test_pages::IS_DARK;
use crate::pages::BookPage;

/// Whether the search dialog is open.
const SEARCH_OPEN: &str =
    "return !!document.querySelector('[role=dialog][aria-label=\"Search documentation\"]');";

/// Whether the search dialog is closed.
const SEARCH_CLOSED: &str =
    "return !document.querySelector('[role=dialog][aria-label=\"Search documentation\"]');";

/// Whether the search field's input has focus.
const INPUT_FOCUSED: &str =
    "return !!document.activeElement && document.activeElement.matches('.doc-search-field input');";

/// Whether results are listed.
const HAS_RESULTS: &str = "return document.querySelectorAll('.doc-search-result').length > 0;";

/// Opens the search with the app bar's button (desktop layout, where it shows with its text) and checks that focus
/// moves into the field. Returns the field.
async fn open_search(page: &BookPage<'_>) -> Result<WebElement, Report> {
    page.set_viewport(1600, 1000).await?;
    page.goto("/doc/overview").await?;
    page.driver
        .find(By::Css(".doc-search-trigger"))
        .await
        .context("the app bar has a search button")?
        .click()
        .await?;
    page.wait_until("the search opens", SEARCH_OPEN).await?;
    page.wait_until("the search field has focus", INPUT_FOCUSED)
        .await?;
    let input = page
        .driver
        .find(By::Css(".doc-search-field input"))
        .await
        .context("the search has a field")?;
    Ok(input)
}

/// Opens the search and types `query`, until results are listed. Returns the field.
async fn search(page: &BookPage<'_>, query: &str) -> Result<WebElement, Report> {
    let input = open_search(page).await?;
    input.send_keys(query).await?;
    // The first search waits until the server has converted every page to Markdown, which takes a while right after
    // the server started, under the load of the parallel tests.
    page.wait_until_within(
        &format!("results for \u{201c}{query}\u{201d} are listed"),
        HAS_RESULTS,
        Duration::from_secs(60),
    )
    .await?;
    Ok(input)
}

/// Typing lists results: snippets are plain text with the query highlighted, no Markdown, no frontmatter.
pub async fn search_lists_plain_text_results(page: &BookPage<'_>) -> Result<(), Report> {
    search(page, "button").await?;
    let snippets = page
        .strings("return [...document.querySelectorAll('.doc-search-result-snippet')].map(s => s.textContent);")
        .await?;
    for snippet in &snippets {
        for markup in ["**", "## ", "](", "title: ", "kind: "] {
            assert_that!(snippet.as_str())
                .with_detail_message(format!("a snippet contains `{markup}`"))
                .does_not_contain(markup);
        }
    }
    let marks = page
        .number("return document.querySelectorAll('.doc-search-result mark').length;")
        .await?;
    assert_that!(marks)
        .with_detail_message("results highlight the query")
        .is_greater_than(0.0);
    Ok(())
}

/// The first Escape empties the field and keeps the search open, the second closes it and focus returns to the app
/// bar's button.
pub async fn search_escape_empties_the_field_then_closes(
    page: &BookPage<'_>,
) -> Result<(), Report> {
    let input = search(page, "button").await?;

    input.send_keys(Key::Escape).await?;
    page.wait_until(
        "Escape empties the field",
        "return document.querySelector('.doc-search-field input').value === '';",
    )
    .await?;
    let open_dialogs = page
        .number("return document.querySelectorAll('[role=dialog][aria-label=\"Search documentation\"]').length;")
        .await?;
    assert_that!(open_dialogs)
        .with_detail_message("the first Escape only empties the field")
        .is_equal_to(1.0);

    input.send_keys(Key::Escape).await?;
    page.wait_until("a second Escape closes the search", SEARCH_CLOSED)
        .await?;
    page.wait_until(
        "focus returns to the search button",
        "return !!document.activeElement && document.activeElement.matches('.doc-search-trigger');",
    )
    .await?;
    Ok(())
}

/// Enter opens the first result and closes the search.
pub async fn search_enter_opens_the_first_result(page: &BookPage<'_>) -> Result<(), Report> {
    let input = search(page, "modal").await?;
    let first = page
        .strings("return [document.querySelector('.doc-search-result').getAttribute('href')];")
        .await?;
    let first = first.first().cloned().unwrap_or_default();
    assert_that!(first.as_str()).starts_with("/doc/");
    input.send_keys(Key::Enter).await?;
    page.wait_until(
        &format!("Enter opens the first result, {first}"),
        &format!("return location.pathname + location.hash === '{first}';"),
    )
    .await?;
    page.wait_until("the search closes after navigating", SEARCH_CLOSED)
        .await?;
    Ok(())
}

/// The search shortcut: Ctrl+K (Cmd+K on Apple devices) opens the search with focus in its field.
pub async fn search_opens_with_ctrl_k(page: &BookPage<'_>) -> Result<(), Report> {
    page.set_viewport(1600, 1000).await?;
    page.goto("/doc/overview").await?;

    let primary = if page.is_apple_device().await? {
        Key::Meta
    } else {
        Key::Control
    };
    page.driver
        .action_chain()
        .key_down(primary.clone())
        .send_keys("k")
        .key_up(primary)
        .perform()
        .await?;
    page.wait_until("Ctrl+K (Cmd+K) opens the search", SEARCH_OPEN)
        .await?;
    page.wait_until("the search field has focus", INPUT_FOCUSED)
        .await?;
    Ok(())
}

/// The app bar's theme toggle switches from the light theme (the default) to the dark one, and the book remembers
/// the choice: the next page arrives in it.
pub async fn theme_toggle_switches_and_is_remembered(page: &BookPage<'_>) -> Result<(), Report> {
    // The desktop layout, where the app bar shows the theme toggle.
    page.set_viewport(1600, 1000).await?;
    page.goto("/doc/overview").await?;
    let theme = "return [document.documentElement.dataset.theme];";
    assert_that!(page.strings(theme).await?).is_equal_to(vec!["light".to_owned()]);

    page.driver
        .find(By::Css("#book-app-bar .book-theme-toggle"))
        .await
        .context("the app bar has a theme toggle")?
        .click()
        .await?;
    page.wait_until("the dark theme applies", IS_DARK).await?;

    page.goto("/doc/button").await?;
    assert_that!(page.strings(theme).await?).is_equal_to(vec!["dark".to_owned()]);
    page.wait_until("the next page shows the dark theme", IS_DARK)
        .await?;
    Ok(())
}

/// Whether the small-screen documentation menu is open.
const DOC_MENU_OPEN: &str =
    "return !!document.querySelector('[role=dialog][aria-label=\"Documentation\"]');";

/// The documentation menu at phone width: the app bar's menu button opens it as a dialog, Escape closes it again
/// (focus returns to the button).
pub async fn doc_menu_opens_at_phone_width_and_closes_on_escape(
    page: &BookPage<'_>,
) -> Result<(), Report> {
    page.set_viewport(390, 844).await?;
    page.goto("/doc/overview").await?;

    let button = page
        .driver
        .find(By::Css("[aria-label=\"Documentation menu\"]"))
        .await
        .context("the app bar has a documentation menu button")?;
    assert_that!(button.attr("aria-expanded").await?).is_equal_to(Some("false".to_owned()));
    button.click().await?;
    page.wait_until("the documentation menu opens", DOC_MENU_OPEN)
        .await?;
    page.wait_until(
        "focus moves into the menu",
        "return !!document.activeElement && !!document.activeElement.closest('[role=dialog][aria-label=\"Documentation\"]');",
    )
    .await?;

    page.driver
        .active_element()
        .await?
        .send_keys(Key::Escape)
        .await?;
    page.wait_until(
        "Escape closes the documentation menu",
        "return !document.querySelector('[role=dialog][aria-label=\"Documentation\"]');",
    )
    .await?;
    page.wait_until(
        "focus returns to the menu button",
        "return !!document.activeElement && document.activeElement.matches('[aria-label=\"Documentation menu\"]');",
    )
    .await?;
    Ok(())
}

/// A demo's "View source" disclosure: collapsed by default (`aria-expanded="false"`, the code hidden), pressing it
/// expands it and shows the code.
pub async fn demo_view_source_toggles_the_code(page: &BookPage<'_>) -> Result<(), Report> {
    page.set_viewport(1600, 1000).await?;
    // A page whose demo starts with its source collapsed.
    page.goto("/doc/tooltip/atom").await?;

    let trigger = page
        .driver
        .find(By::XPath(
            "//*[contains(@class,'doc-demo')]//button[contains(@class,'doc-disclosure-trigger')][normalize-space(.)='View source']",
        ))
        .await
        .context("the demo has a \u{201c}View source\u{201d} button")?;
    assert_that!(trigger.attr("aria-expanded").await?).is_equal_to(Some("false".to_owned()));
    let panel_id = trigger.attr("aria-controls").await?.unwrap_or_default();
    assert_that!(panel_id.as_str()).is_not_empty();
    let panel = page.driver.find(By::Id(&panel_id)).await?;
    assert_that!(panel.is_displayed().await?)
        .with_detail_message("the code is hidden while collapsed")
        .is_false();

    // Near the top of the viewport, below the sticky app bar (instantly: the book scrolls smoothly).
    page.driver
        .execute(
            "arguments[0].scrollIntoView({block: 'start', behavior: 'instant'}); window.scrollBy({top: -200, behavior: 'instant'});",
            vec![trigger.to_json()?],
        )
        .await?;
    trigger.click().await?;
    page.wait_until(
        "pressing \u{201c}View source\u{201d} expands it",
        "return document.querySelector('.doc-demo .doc-disclosure-trigger').getAttribute('aria-expanded') === 'true';",
    )
    .await?;
    assert_that!(panel.is_displayed().await?)
        .with_detail_message("the code is shown while expanded")
        .is_true();
    assert_that!(panel.text().await?).contains("TooltipTrigger");
    Ok(())
}

/// Whether the current page has highlighted Rust blocks (`syn-` spans from the highlighter).
const RUST_HIGHLIGHTED: &str = "return document.querySelectorAll('main .doc-code[data-language=rust] .doc-code-text [class*=\"syn-\"]').length > 0;";

/// Code blocks are highlighted on pages the client renders itself: the wasm has no highlighter, the server highlights
/// them (`kit::code`'s `highlight`).
pub async fn code_blocks_are_highlighted_after_client_side_navigation(
    page: &BookPage<'_>,
) -> Result<(), Report> {
    page.set_viewport(1600, 1000).await?;

    // The server highlights the blocks of the page it renders.
    page.goto("/doc/installation").await?;
    page.wait_until("the server rendered highlighted blocks", RUST_HIGHLIGHTED)
        .await?;

    // A link in the sidebar navigates on the client: the page isn't loaded again (the marker survives).
    page.driver
        .execute("window.__noReload = true;", vec![])
        .await?;
    page.driver
        .find(By::Css("nav a[href='/doc/classes-and-styles']"))
        .await
        .context("the sidebar links the classes and styles guide")?
        .click()
        .await?;
    page.wait_until(
        "the client rendered the classes and styles guide",
        "return location.pathname === '/doc/classes-and-styles' && document.querySelectorAll('main .doc-code[data-language=rust]').length > 0;",
    )
    .await?;
    page.wait_until(
        "the server highlighted the new page's blocks",
        RUST_HIGHLIGHTED,
    )
    .await?;
    let reloaded = page
        .number("return window.__noReload === true ? 0 : 1;")
        .await?;
    assert_that!(reloaded)
        .with_detail_message("the page was navigated to on the client, not loaded")
        .is_equal_to(0.0);
    Ok(())
}

/// Loads the overview of Button at desktop width.
async fn open_button(page: &BookPage<'_>) -> Result<(), Report> {
    page.set_viewport(1600, 1000).await?;
    page.goto("/doc/button").await
}

/// The page structure: a title per page, a description, landmarks (the app bar in a `<header>`, the navigation and the
/// table of contents outside `<main>`), and headings for the sidebar parts.
pub async fn pages_have_titles_descriptions_and_landmarks(
    page: &BookPage<'_>,
) -> Result<(), Report> {
    open_button(page).await?;
    page.wait_until(
        "the page has its own title",
        "return document.title === 'Button \u{2013} Leptonic';",
    )
    .await?;
    let description = page
        .strings(
            "return [...document.querySelectorAll('meta[name=description]')].map(m => m.content);",
        )
        .await?;
    assert_that!(description.len())
        .with_detail_message("one description")
        .is_equal_to(1);
    assert_that!(description[0].as_str()).starts_with("Triggers an action");

    let structure = page
        .strings(
            "return [
                String(document.querySelectorAll('main').length),
                document.querySelector('main').id,
                String(!!document.querySelector('header #book-app-bar, header#book-app-bar')),
                String(!!document.querySelector('main .book-doc-nav, main #book-toc')),
                String(!!document.querySelector('main .doc-concept-tabs')),
                [...document.querySelectorAll('#book-doc-sidebar nav h2')].map(h => h.textContent).join(', '),
                document.querySelector('.doc-search-trigger').getAttribute('aria-label'),
            ];",
        )
        .await?;
    assert_that!(structure).is_equal_to(
        [
            "1",
            "book-main",
            "true",
            "false",
            "true",
            "Concepts, Building blocks",
            "Search docs",
        ]
        .map(str::to_owned)
        .to_vec(),
    );
    Ok(())
}

/// The search button names its shortcut with the platform's primary modifier (once hydrated, on Apple devices).
pub async fn search_button_names_its_shortcut(page: &BookPage<'_>) -> Result<(), Report> {
    open_button(page).await?;
    let shortcut = if page.is_apple_device().await? {
        "Meta+K"
    } else {
        "Control+K"
    };
    page.wait_until(
        &format!("the search button's shortcut is {shortcut}"),
        &format!(
            "return document.querySelector('.doc-search-trigger').getAttribute('aria-keyshortcuts') === '{shortcut}';"
        ),
    )
    .await?;
    Ok(())
}

/// No two controls of the sidebar share a name: a group's toggle is not named like its overview link.
pub async fn sidebar_controls_have_distinct_names(page: &BookPage<'_>) -> Result<(), Report> {
    open_button(page).await?;
    let duplicates = page
        .strings(
            "const names = [...document.querySelectorAll('#book-doc-sidebar :is(a, button)')]
                .map(e => (e.getAttribute('aria-label') || e.textContent).trim());
             return names.filter((name, i) => names.indexOf(name) !== i);",
        )
        .await?;
    assert_that!(duplicates).is_empty();
    Ok(())
}

/// The skip link is the first stop of Tab, and moves focus into the page content.
pub async fn skip_link_moves_focus_to_the_content(page: &BookPage<'_>) -> Result<(), Report> {
    open_button(page).await?;
    page.driver
        .action_chain()
        .send_keys(Key::Tab)
        .perform()
        .await?;
    page.wait_until(
        "Tab focuses the skip link first, which shows",
        "const a = document.activeElement; return !!a && a.textContent.trim() === 'Skip to content' \
         && a.getBoundingClientRect().width > 20;",
    )
    .await?;
    page.driver
        .active_element()
        .await?
        .send_keys(Key::Enter)
        .await?;
    page.wait_until(
        "the skip link moves focus to the page content",
        "return document.activeElement && document.activeElement.id === 'book-main';",
    )
    .await?;
    Ok(())
}

/// On a phone, the "Copy as Markdown" button doesn't cover the page title.
pub async fn copy_button_leaves_the_title_free_on_a_phone(
    page: &BookPage<'_>,
) -> Result<(), Report> {
    page.set_viewport(390, 844).await?;
    page.goto("/doc/focus/use-has-tabbable-child").await?;
    let overlap = page
        .number(
            "const a = document.querySelector('.doc-article h1').getBoundingClientRect();
             const b = document.querySelector('.doc-copy-markdown').getBoundingClientRect();
             return Math.max(0, Math.min(a.right, b.right) - Math.max(a.left, b.left))
                  * Math.max(0, Math.min(a.bottom, b.bottom) - Math.max(a.top, b.top));",
        )
        .await?;
    assert_that!(overlap)
        .with_detail_message("the copy button doesn't cover the title")
        .is_equal_to(0.0);
    Ok(())
}

/// On a phone, the concept tabs stay on one row, and anchored headings stay below them.
pub async fn concept_tabs_fit_a_phone(page: &BookPage<'_>) -> Result<(), Report> {
    page.set_viewport(390, 844).await?;
    page.goto("/doc/button/atom").await?;
    let rows = page
        .number(
            "return new Set([...document.querySelectorAll('.doc-concept-tab')].map(t => t.offsetTop)).size;",
        )
        .await?;
    assert_that!(rows)
        .with_detail_message("the concept tabs are one row")
        .is_equal_to(1.0);
    page.driver
        .execute(
            "const h = [...document.querySelectorAll('.doc-article h2[id]')].pop();
             h.style.scrollBehavior = 'auto';
             h.scrollIntoView({block: 'start', behavior: 'instant'});",
            vec![],
        )
        .await?;
    page.wait_until(
        "an anchored heading stays below the concept tabs",
        "const h = [...document.querySelectorAll('.doc-article h2[id]')].pop().getBoundingClientRect();
         const tabs = document.querySelector('.doc-concept-tabs').getBoundingClientRect();
         return h.top >= tabs.bottom - 1;",
    )
    .await?;
    Ok(())
}

/// On a phone, the app bar's main menu leads to the docs.
pub async fn main_menu_links_the_docs_on_a_phone(page: &BookPage<'_>) -> Result<(), Report> {
    page.set_viewport(390, 844).await?;
    page.goto("/").await?;
    page.driver
        .find(By::Css("[aria-label=\"Menu\"]"))
        .await
        .context("the app bar has a menu button")?
        .click()
        .await?;
    page.wait_until(
        "the main menu links the docs",
        "return !!document.querySelector('[role=dialog] .book-main-menu a[href=\"/doc\"]');",
    )
    .await?;
    Ok(())
}

/// On a tablet, the welcome page's cards don't leave one alone in a row.
pub async fn welcome_cards_fit_a_tablet(page: &BookPage<'_>) -> Result<(), Report> {
    page.set_viewport(700, 900).await?;
    page.goto("/").await?;
    let columns = page
        .strings(
            "return [...document.querySelectorAll('.book-welcome-cards')].map(list =>
                String(new Set([...list.children].map(card => card.offsetLeft)).size));",
        )
        .await?;
    for count in columns {
        assert_that!(count == "1" || count == "3")
            .with_detail_message(format!(
                "welcome cards: one per row or all in one row, not {count} columns"
            ))
            .is_true();
    }
    Ok(())
}

/// How many Markdown exports (`*.md`) the page has requested so far.
const MARKDOWN_REQUESTS: &str = "return performance.getEntriesByType('resource')\
    .filter(e => new URL(e.name).pathname.endsWith('.md')).length;";

/// "Copy as Markdown": opening a page or navigating to another one downloads no Markdown; the first press downloads
/// the page's export and copies it, a second press copies the cached one without downloading it again.
pub async fn copy_as_markdown_downloads_only_on_press_and_once(
    page: &BookPage<'_>,
) -> Result<(), Report> {
    page.goto("/doc/overview").await?;
    assert_that!(page.number(MARKDOWN_REQUESTS).await?).is_equal_to(0.0);
    page.allow_clipboard_read().await?;

    // A client-side navigation to another page doesn't download its export either. The book scrolls smoothly: bring
    // the link into view instantly before clicking it.
    let link = page
        .driver
        .find(By::Css("article a[href=\"/doc/installation\"]"))
        .await
        .context("the overview links the installation guide")?;
    page.driver
        .execute(
            "arguments[0].scrollIntoView({ behavior: 'instant', block: 'center' });",
            vec![link.to_json()?],
        )
        .await?;
    link.click().await?;
    // The navigation scrolls the new page to the top, smoothly: wait for it, so that the button stays in place.
    page.wait_until(
        "the installation guide is shown, scrolled to the top",
        "return location.pathname === '/doc/installation' && !!document.querySelector('.doc-copy-markdown') \
            && window.scrollY === 0;",
    )
    .await?;
    assert_that!(page.number(MARKDOWN_REQUESTS).await?).is_equal_to(0.0);

    // The first press downloads the export and copies it.
    let press_and_wait = async || -> Result<(), Report> {
        page.driver
            .execute("return navigator.clipboard.writeText('');", vec![])
            .await?;
        page.driver
            .find(By::Css(".doc-copy-markdown"))
            .await
            .context("the page has a Copy as Markdown button")?
            .click()
            .await?;
        page.wait_until(
            "the button reports the copy",
            "return document.querySelector('.doc-copy-markdown').textContent.includes('Copied');",
        )
        .await?;
        let copied = page
            .driver
            .execute("return navigator.clipboard.readText();", vec![])
            .await?
            .convert::<String>()?;
        assert_that!(copied.as_str()).starts_with("---\ntitle: \"Installation\"\n");
        page.wait_until(
            "the button is ready again",
            "return document.querySelector('.doc-copy-markdown').textContent.includes('Copy as Markdown');",
        )
        .await
    };
    press_and_wait().await?;
    assert_that!(page.number(MARKDOWN_REQUESTS).await?).is_equal_to(1.0);

    // The second press copies the cached export.
    press_and_wait().await?;
    assert_that!(page.number(MARKDOWN_REQUESTS).await?).is_equal_to(1.0);
    Ok(())
}

/// The book's fonts: Roboto for text and controls, `JetBrains Mono` for code. Leptonic's atoms bring no styles, so the
/// book's own base stylesheet has to apply them.
pub async fn text_controls_and_code_use_the_book_fonts(page: &BookPage<'_>) -> Result<(), Report> {
    page.goto("/doc/installation").await?;
    let fonts = page
        .strings(
            "return [document.body, document.querySelector('.doc-search-trigger'), document.querySelector('pre code, code.doc-code')]\
             .map(e => e ? getComputedStyle(e).fontFamily : 'missing');",
        )
        .await?;
    assert_that!(fonts[0].as_str()).contains("Roboto");
    assert_that!(fonts[1].as_str()).contains("Roboto");
    assert_that!(fonts[2].as_str()).contains("JetBrains Mono");
    Ok(())
}

/// A code block's copy button puts exactly the block's code on the clipboard and confirms it with a check mark.
pub async fn code_block_copy_button_copies_the_code(page: &BookPage<'_>) -> Result<(), Report> {
    page.goto("/doc/interactions/use-press").await?;
    page.allow_clipboard_read().await?;
    page.driver
        .execute("return navigator.clipboard.writeText('');", vec![])
        .await?;

    let button = page
        .driver
        .find(By::Css(
            "article code.doc-code[data-language] .doc-code-copy",
        ))
        .await
        .context("the page has a code block with a copy button")?;
    page.driver
        .execute(
            "arguments[0].scrollIntoView({ behavior: 'instant', block: 'center' });",
            vec![button.to_json()?],
        )
        .await?;
    let icon_before = page
        .strings("return [document.querySelector('article .doc-code-copy svg').innerHTML];")
        .await?;
    button.click().await?;
    page.wait_until(
        "the button shows a check mark",
        &format!(
            "return document.querySelector('article .doc-code-copy svg').innerHTML !== {};",
            serde_json::to_string(&icon_before[0])?
        ),
    )
    .await?;

    let code = page
        .strings("return [document.querySelector('article code.doc-code[data-language] .doc-code-text').textContent];")
        .await?;
    let copied = page
        .driver
        .execute("return navigator.clipboard.readText();", vec![])
        .await?
        .convert::<String>()?;
    assert_that!(copied.trim()).is_equal_to(code[0].trim());
    assert_that!(copied.as_str()).contains("use_press(");
    Ok(())
}

/// Keys in keyboard tables are key caps: one `<kbd>` per key inside the combination's `<kbd>`, drawn with a border in
/// the code font. A description of a key group in a combination ("Shift + Arrow keys") is text, not a key cap.
pub async fn keys_are_key_caps_and_descriptions_are_text(
    page: &BookPage<'_>,
) -> Result<(), Report> {
    page.goto("/doc/color-slider").await?;
    let row = page
        .strings(
            "const cell = [...document.querySelectorAll('article td.doc-table-name')]
                .find(td => td.textContent.includes('Arrow keys') && td.textContent.includes('Shift'));
             if (!cell) return ['missing'];
             // The text a screen reader reads: glyphs are hidden, the keys' names are visually hidden.
             const spoken = node => node.nodeType === Node.TEXT_NODE ? node.textContent
                : node.nodeType !== Node.ELEMENT_NODE || node.getAttribute('aria-hidden') === 'true' ? ''
                : [...node.childNodes].map(spoken).join('');
             const caps = [...cell.querySelectorAll('kbd.doc-keys > kbd')];
             const style = getComputedStyle(caps[0]);
             return [
                caps.map(spoken).join(' '),
                spoken(cell).replace(/\\s+/g, ' ').trim(),
                String(parseFloat(style.borderTopWidth) >= 1 && style.borderTopStyle !== 'none'),
                String(style.fontFamily.includes('JetBrains Mono')),
             ];",
        )
        .await?;
    assert_that!(row).is_equal_to(
        ["Shift", "Shift + Arrow keys", "true", "true"]
            .map(str::to_owned)
            .to_vec(),
    );
    Ok(())
}
