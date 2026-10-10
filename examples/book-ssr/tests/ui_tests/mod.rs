pub mod test_contrast;
pub mod test_pages;
pub mod test_shell;
pub mod test_sidebar;

use std::{borrow::Cow, panic::AssertUnwindSafe};

use browser_test::{
    BrowserTest, BrowserTests, Parallelism, SessionSettings, async_trait, thirtyfour::WebDriver,
};
use futures::FutureExt as _;
use leptos_browser_test::{Report, ResultExt};

use crate::{
    cases::{Case, CaseFn},
    pages::BookPage,
};

/// Every browser test of the book: the checks at `parallelism` at once, then the checks of the whole run
/// ([`after_all`]).
pub fn all(parallelism: Parallelism) -> BrowserTests<str> {
    BrowserTests::sequential()
        .with_nested(checks(BrowserTests::parallel(parallelism)))
        .with_nested(after_all(
            BrowserTests::sequential().named("after all").run_always(),
        ))
}

/// The independent checks, each loading its pages itself. Register new tests here.
///
/// With `BROWSER_TEST_FILTER=<text>`, only the tests whose name contains `<text>` run (several texts separated by
/// commas: any of them).
fn checks(group: BrowserTests<str>) -> BrowserTests<str> {
    Selected::new(group)
        // The page tests first: the search and LLM index cases wait until the server has converted every page to
        // Markdown (`markdown::warm_markdown_cache`, ~15-30s after it started), and would only hold slots meanwhile.
        .with_all(test_pages::page_tests())
        .case(test_pages::llm_index_lists_every_page)
        .case(test_pages::pages_are_served_as_markdown)
        .case(test_shell::search_lists_plain_text_results)
        .case(test_shell::search_escape_empties_the_field_then_closes)
        .case(test_shell::search_enter_opens_the_first_result)
        .case(test_shell::search_opens_with_ctrl_k)
        .case(test_shell::theme_toggle_switches_and_is_remembered)
        .case(test_shell::doc_menu_opens_at_phone_width_and_closes_on_escape)
        .case(test_shell::demo_view_source_toggles_the_code)
        .case(test_shell::code_blocks_are_highlighted_after_client_side_navigation)
        .case(test_shell::pages_have_titles_descriptions_and_landmarks)
        .case(test_shell::search_button_names_its_shortcut)
        .case(test_shell::sidebar_controls_have_distinct_names)
        .case(test_shell::skip_link_moves_focus_to_the_content)
        .case(test_shell::copy_button_leaves_the_title_free_on_a_phone)
        .case(test_shell::concept_tabs_fit_a_phone)
        .case(test_shell::main_menu_links_the_docs_on_a_phone)
        .case(test_shell::welcome_cards_fit_a_tablet)
        .case(test_shell::copy_as_markdown_downloads_only_on_press_and_once)
        .case(test_shell::text_controls_and_code_use_the_book_fonts)
        .case(test_shell::code_block_copy_button_copies_the_code)
        .case(test_shell::keys_are_key_caps_and_descriptions_are_text)
        .with_all(test_contrast::contrast_tests())
        .case(test_sidebar::current_group_is_expanded)
        .case(test_sidebar::concepts_show_layers_and_building_blocks_badges)
        .case(test_sidebar::concept_tabs_are_named_after_the_layers)
        .case(test_sidebar::toggles_and_navigation_expand_groups)
        .tests
}

/// Checks of the whole run, after [`checks`]: they use what the checks collected. They run even when a check failed.
fn after_all(group: BrowserTests<str>) -> BrowserTests<str> {
    Selected::new(group).with(test_pages::LinkTests {}).tests
}

/// The tests matching `BROWSER_TEST_FILTER`.
struct Selected {
    tests: BrowserTests<str>,
    filter: Option<String>,
}

impl Selected {
    fn new(tests: BrowserTests<str>) -> Self {
        Self {
            tests,
            filter: std::env::var("BROWSER_TEST_FILTER").ok(),
        }
    }

    fn case(self, case: impl for<'a> CaseFn<'a>) -> Self {
        self.with(Case(case))
    }

    fn with(mut self, test: impl BrowserTest<str> + 'static) -> Self {
        if self
            .filter
            .as_deref()
            .is_none_or(|filter| filter.split(',').any(|part| test.name().contains(part)))
        {
            self.tests = self.tests.with(CheckPageErrors(test));
        }
        self
    }

    fn with_all<T: BrowserTest<str> + 'static>(self, tests: impl IntoIterator<Item = T>) -> Self {
        tests.into_iter().fold(self, Self::with)
    }
}

/// Runs a test, then checks what the page reported (uncaught errors and Rust panics; see
/// `BookPage::expect_no_page_errors`; `goto` checks the page it leaves):
/// - the test passed: page errors fail it;
/// - the test failed: page errors are added to its failure, as they are often the cause (a panic in an event handler
///   shows as a wait that times out);
/// - an assertion panicked: page errors are logged, then the panic continues.
struct CheckPageErrors<T>(T);

#[async_trait]
impl<T: BrowserTest<str>> BrowserTest<str> for CheckPageErrors<T> {
    fn name(&self) -> Cow<'_, str> {
        self.0.name()
    }

    fn description(&self) -> Option<Cow<'_, str>> {
        self.0.description()
    }

    fn session_settings(&self) -> SessionSettings {
        self.0.session_settings()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = BookPage { driver, base_url };
        let outcome = AssertUnwindSafe(self.0.run(driver, base_url))
            .catch_unwind()
            .await;
        let page_errors = page.expect_no_page_errors().await;
        match (outcome, page_errors) {
            (Ok(Ok(())), page_errors) => {
                page_errors.context("the page reported problems after the test passed")?;
                Ok(())
            }
            (Ok(Err(failure)), Ok(())) => Err(failure),
            (Ok(Err(failure)), Err(page_errors)) => Err(failure
                .context(format!(
                    "the page also reported problems, possibly the cause:\n{page_errors}"
                ))
                .into_dynamic()),
            (Err(panic), page_errors) => {
                if let Err(page_errors) = page_errors {
                    tracing::error!(
                        "The page reported problems, possibly the cause of the panic:\n{page_errors}"
                    );
                }
                std::panic::resume_unwind(panic)
            }
        }
    }
}
