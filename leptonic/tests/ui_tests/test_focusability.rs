// Upstream: react-aria/src/utils/isFocusable.ts @ 740c6c5c4a
//! `is_focusable` and `is_tabbable` against react-aria's selectors, for elements with different
//! `tabindex` values.
use browser_test::browser_test;
use rootcause::{Report, prelude::ResultExt};

use crate::pages::{ElementActions, Page};

const PATH: &str = "/hooks/focusability";

/// Any `tabindex` makes an element focusable (also an invalid one), and only exactly
/// `tabindex="-1"` keeps it out of the tab order: `-2` and invalid values count as tabbable, as in
/// react-aria's `isTabbable`. Disabled elements are neither.
#[browser_test]
pub async fn tabindex_values(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    for (case, focusable, tabbable) in [
        ("div", "false", "false"),
        ("div-0", "true", "true"),
        ("div-1", "true", "true"),
        ("div-minus-1", "true", "false"),
        ("div-minus-2", "true", "true"),
        ("div-invalid", "true", "true"),
        ("div-disabled", "false", "false"),
        ("button", "true", "true"),
        ("button-minus-1", "true", "false"),
        ("button-minus-2", "true", "true"),
        ("button-invalid", "true", "true"),
        ("button-disabled", "false", "false"),
    ] {
        let element = page.element(format!("[data-case='{case}']")).await?;
        element
            .wait_for_attr("data-focusable", Some(focusable))
            .await
            .context_with(|| format!("is_focusable of {case}"))?;
        element
            .wait_for_attr("data-tabbable", Some(tabbable))
            .await
            .context_with(|| format!("is_tabbable of {case}"))?;
    }
    Ok(())
}
