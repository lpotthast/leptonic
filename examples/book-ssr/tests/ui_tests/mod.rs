pub mod test_contrast;
pub mod test_pages;
pub mod test_shell;
pub mod test_sidebar;

use browser_test::{BrowserTest, BrowserTests, Parallelism};

/// Every browser test of the book: the checks at `parallelism` at once, then the checks of the whole run
/// ([`after_all`]). With `BROWSER_TEST_FILTER=<text>`, only the tests whose name contains `<text>` run.
pub fn all(parallelism: Parallelism) -> BrowserTests<str> {
    BrowserTests::sequential()
        .with_group(checks(BrowserTests::parallel(parallelism)))
        .with_group(after_all(BrowserTests::sequential().named("after all")))
}

/// The independent checks, each visiting its pages itself. Register new tests here.
fn checks(group: BrowserTests<str>) -> BrowserTests<str> {
    Selected::new(group)
        .with_all(
            test_pages::Shard::all(PAGE_SHARDS).map(|shard| test_pages::PageContentTests { shard }),
        )
        .with(test_pages::MarkdownExportTests {})
        .with(test_shell::SearchTests {})
        .with(test_shell::SearchShortcutTests {})
        .with(test_shell::DocMenuTests {})
        .with(test_shell::DemoSourceTests {})
        .with(test_shell::CopyMarkdownTests {})
        .with(test_shell::FontTests {})
        .with(test_shell::CodeCopyTests {})
        .with(test_shell::KeyCapTests {})
        .with(test_shell::CodeHighlightTests {})
        .with(test_shell::ShellStructureTests {})
        .with(test_shell::NarrowShellTests {})
        .with(test_contrast::ContrastTests {})
        .with(test_sidebar::SidebarTests {})
        .tests
}

/// Checks of the whole run, after [`checks`]: they use what the checks collected.
fn after_all(group: BrowserTests<str>) -> BrowserTests<str> {
    Selected::new(group).with(test_pages::LinkTests {}).tests
}

/// Into how many parallel shards the walks over all pages are split.
const PAGE_SHARDS: usize = 8;

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

    fn with(mut self, test: impl BrowserTest<str> + 'static) -> Self {
        if self
            .filter
            .as_deref()
            .is_none_or(|filter| test.name().contains(filter))
        {
            self.tests = self.tests.with(test);
        }
        self
    }

    fn with_all<T: BrowserTest<str> + 'static>(self, tests: impl IntoIterator<Item = T>) -> Self {
        tests.into_iter().fold(self, Self::with)
    }
}
