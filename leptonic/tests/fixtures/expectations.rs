//! Expected fixture defects, separate from diagnostic collection. Each entry is scoped to an
//! exact route and category. Full fixture loads validate that the expectation still occurs.
use rootcause::{Report, bail};
use serde::{Deserialize, Serialize};

use crate::pages::{
    Page,
    health::{self, Health, PagePanicked},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum Kind {
    Warning,
    Reference,
}
struct Expected {
    path: &'static str,
    kind: Kind,
    text: &'static str,
    occurrences: Occurrences,
}
enum Occurrences {
    Always(usize),
    Sections(&'static [(&'static str, usize)]),
}
impl Occurrences {
    fn count(&self, only: Option<&str>) -> usize {
        match self {
            Self::Always(count) => *count,
            Self::Sections(sections) => sections
                .iter()
                .filter(|(section, _)| {
                    only.is_none_or(|only| only.split(',').any(|selected| selected == *section))
                })
                .map(|(_, count)| count)
                .sum(),
        }
    }
}
const MISSING_LABEL: &str = "If you do not provide a visible label, you must specify an aria-label or aria-labelledby attribute for accessibility";

const EXPECTED: &[Expected] = &[
    Expected {
        path: "/atoms/label-slots",
        kind: Kind::Warning,
        text: MISSING_LABEL,
        occurrences: Occurrences::Always(4),
    },
    Expected {
        path: "/atoms/slider-interactions",
        kind: Kind::Warning,
        text: "Slider thumb 1 has no value: the slider has 1 values.",
        occurrences: Occurrences::Sections(&[("missing", 1)]),
    },
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Accepted {
    kind: Kind,
    text: String,
}

/// Navigation resets the page's diagnostic baseline. Capture only declared fixture issues,
/// with their exact messages and multiplicities. Later new occurrences are not suppressed.
pub(super) async fn check_initial(page: &Page<'_>) -> Result<(), Report> {
    let health = health::read(page.driver()).await?;
    let url = page.driver().current_url().await?;
    let only = url
        .query_pairs()
        .find(|(key, _)| key == "only")
        .map(|(_, value)| value.into_owned());
    let accepted = accept_initial(&health, only.as_deref())?;
    page.low_level()
        .eval::<()>(
            "window.__acceptedFixtureProblems = arguments[0];",
            vec![serde_json::to_value(&accepted)?],
        )
        .await?;
    validate(health, accepted)
}

/// Validate declared initial counts before recording exact browser messages.
fn accept_initial(health: &Health, only: Option<&str>) -> Result<Vec<Accepted>, Report> {
    let mut accepted = Vec::new();
    for expected in EXPECTED
        .iter()
        .filter(|expected| expected.path == health.path)
    {
        let items = match expected.kind {
            Kind::Warning => &health.diagnostics.console_warnings,
            Kind::Reference => &health.dangling_references,
        };
        let matches: Vec<_> = items
            .iter()
            .filter(|item| match expected.kind {
                Kind::Warning => *item == expected.text,
                Kind::Reference => item.ends_with(expected.text),
            })
            .collect();
        let count = expected.occurrences.count(only);
        if matches.len() != count {
            bail!(
                "fixture expectation for {}: expected {count} {:?} occurrences of {:?}, found {}",
                health.path,
                expected.kind,
                expected.text,
                matches.len()
            );
        }
        accepted.extend(matches.into_iter().map(|text| Accepted {
            kind: expected.kind,
            text: text.clone(),
        }));
    }
    Ok(accepted)
}

pub async fn check(page: &Page<'_>) -> Result<(), Report> {
    let url = page.driver().current_url().await?;
    // Runner reset pages and the server's plain-text panic report have no app instrumentation.
    if url.scheme() == "about" || url.scheme() == "data" || url.path() == "/__test/server-panics" {
        return Ok(());
    }
    let health = health::read(page.driver()).await?;
    let accepted = page
        .low_level()
        .eval("return window.__acceptedFixtureProblems ?? [];", vec![])
        .await?;
    validate(health, accepted)
}

/// Consume only warnings that the case explicitly expects, with an exact occurrence count.
/// Other categories and unmatched warnings remain for the fixture's health check.
pub async fn take_warnings(page: &Page<'_>, containing: &str, count: usize) -> Result<(), Report> {
    let found: usize = page.low_level().eval(
        "const [text, count] = arguments; const warnings = window.__consoleWarnings;
         if (!Array.isArray(warnings)) throw new Error('test diagnostics are not installed');
         const found = warnings.filter(value => value.includes(text)).length;
         if (found === count) {
             window.__consoleWarnings = warnings.filter(value => !value.includes(text));
             window.__acceptedFixtureProblems = (window.__acceptedFixtureProblems ?? []).filter(entry => entry.kind !== 'Warning' || !entry.text.includes(text));
         }
         return found;", vec![containing.into(), count.into()]).await?;
    if found != count {
        rootcause::bail!("expected {count} warnings containing {containing:?}, found {found}");
    }
    Ok(())
}

fn validate(health: Health, mut accepted: Vec<Accepted>) -> Result<(), Report> {
    let mut problems = Vec::new();
    for (kind, category, items) in [
        (None, "uncaught errors", health.diagnostics.uncaught_errors),
        (None, "console errors", health.diagnostics.console_errors),
        (
            Some(Kind::Warning),
            "warnings",
            health.diagnostics.console_warnings,
        ),
        (None, "duplicate ids", health.duplicate_ids),
        (
            Some(Kind::Reference),
            "dangling references",
            health.dangling_references,
        ),
        (None, "literal attributes", health.literal_attributes),
    ] {
        for item in items {
            if let Some(index) = accepted
                .iter()
                .position(|entry| Some(entry.kind) == kind && entry.text == item)
            {
                accepted.remove(index);
            } else {
                problems.push(format!("{category}: {item}"));
            }
        }
    }
    if !health.diagnostics.panics.is_empty() {
        return Err(Report::new_sendsync(PagePanicked {
            messages: health.diagnostics.panics,
        })
        .context(problems.join("\n"))
        .into_dynamic());
    }
    if !problems.is_empty() {
        bail!("{}: {}", health.path, problems.join("\n"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::pages::health::PagePanic;

    fn health() -> Health {
        Health {
            path: "/fixture".to_owned(),
            diagnostics: health::Diagnostics::default(),
            duplicate_ids: Vec::new(),
            dangling_references: Vec::new(),
            literal_attributes: Vec::new(),
        }
    }

    fn warning() -> Accepted {
        Accepted {
            kind: Kind::Warning,
            text: "intentional warning".to_owned(),
        }
    }

    #[test]
    fn initial_expectations_reject_missing_extra_and_changed_warnings() {
        let mut state = health();
        state.path = "/atoms/label-slots".to_owned();
        assert_that!(accept_initial(&state, None)).is_err();
        state.diagnostics.console_warnings = vec![MISSING_LABEL.to_owned(); 4];
        assert_that!(accept_initial(&state, None))
            .ok()
            .has_length(4);
        state
            .diagnostics
            .console_warnings
            .push(MISSING_LABEL.to_owned());
        assert_that!(accept_initial(&state, None)).is_err();
        state.diagnostics.console_warnings = vec![format!("{MISSING_LABEL}: new problem"); 4];
        assert_that!(accept_initial(&state, None)).is_err();
    }

    #[test]
    fn section_expectations_apply_only_to_the_selected_sections() {
        const SLIDER_WARNING: &str = "Slider thumb 1 has no value: the slider has 1 values.";
        let mut state = health();
        state.path = "/atoms/slider-interactions".to_owned();
        assert_that!(accept_initial(&state, Some("basic")))
            .ok()
            .is_empty();
        assert_that!(accept_initial(&state, Some("missing"))).is_err();
        state.diagnostics.console_warnings = vec![SLIDER_WARNING.to_owned(); 1];
        assert_that!(accept_initial(&state, Some("basic,missing")))
            .ok()
            .has_length(1);
        assert_that!(accept_initial(&state, Some("basic"))).is_err();
    }

    #[test]
    fn baseline_allows_only_the_recorded_number_of_occurrences() {
        let mut state = health();
        state.diagnostics.console_warnings.push(warning().text);
        assert_that!(validate(state, vec![warning()])).is_ok();
        let mut state = health();
        state.diagnostics.console_warnings = vec![warning().text, warning().text];
        assert_that!(validate(state, vec![warning()])).is_err();
    }

    #[test]
    fn baseline_cannot_hide_another_category_or_a_different_message() {
        let mut state = health();
        state.diagnostics.console_errors.push(warning().text);
        assert_that!(validate(state, vec![warning()])).is_err();
        let mut state = health();
        state
            .diagnostics
            .console_warnings
            .push("intentional warning plus new problem".to_owned());
        assert_that!(validate(state, vec![warning()])).is_err();
    }

    #[test]
    fn baseline_never_hides_panics_or_dom_errors() {
        let mut state = health();
        state.diagnostics.panics.push(warning().text);
        assert_that!(validate(state, vec![warning()]))
            .err()
            .derive_owned(PagePanic::page_panicked)
            .is_true();
        let mut state = health();
        state.duplicate_ids.push(warning().text);
        assert_that!(validate(state, vec![warning()])).is_err();
    }
}
