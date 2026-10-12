// No upstream: react-aria-components throws for a part without its parent's context; leptonic's
// atoms warn and render nothing (documentation/atoms-implementation.md, "Context Pattern").
//! Atom parts outside their parent: each warns once (debug builds) and renders nothing, instead of
//! panicking. So does a `CheckboxField` without the `value` its `CheckboxGroup` needs.
use assertr::prelude::*;
use browser_test::browser_test;
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/atoms/orphan-parts";

/// Mounts the misplaced parts of the family `name`, checks that each of `warnings` was logged
/// once, and that nothing rendered.
async fn mount_orphans(page: &Page<'_>, name: &str, warnings: &[&str]) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(format!("#orphans-{name}-mount"))
        .await?
        .click()
        .await?;
    let orphans = page.element(format!("#orphans-{name}")).await?;
    orphans.wait_for_attr("data-mounted", Some("true")).await?;
    for warning in warnings {
        crate::fixtures::take_warnings(page, warning, 1).await?;
    }
    assert_that!(orphans.count("*").await?).is_equal_to(0);
    assert_that!(orphans).inner_text().await.is_empty();
    Ok(())
}

/// A `SearchFieldClearButton` outside a `SearchField` warns and renders nothing.
#[browser_test]
pub async fn search_field_parts(page: &Page<'_>) -> Result<(), Report> {
    mount_orphans(
        page,
        "search-field",
        &["A <SearchFieldClearButton> must be inside a <SearchField>."],
    )
    .await
}

/// The group and stepper buttons of a number field outside a `NumberField` warn and render
/// nothing.
#[browser_test]
pub async fn number_field_parts(page: &Page<'_>) -> Result<(), Report> {
    mount_orphans(
        page,
        "number-field",
        &[
            "A <NumberFieldGroup> must be inside a <NumberField>.",
            "A <NumberFieldIncrementButton> must be inside a <NumberField>.",
            "A <NumberFieldDecrementButton> must be inside a <NumberField>.",
        ],
    )
    .await
}

/// `ListBoxItems` outside a `ListBox` warns and renders nothing.
#[browser_test]
pub async fn list_box_parts(page: &Page<'_>) -> Result<(), Report> {
    mount_orphans(
        page,
        "list-box",
        &["<ListBoxItems> must be inside a <ListBox>."],
    )
    .await
}

/// A `ComboBoxPopover` outside a `ComboBox` warns and renders nothing.
#[browser_test]
pub async fn combo_box_parts(page: &Page<'_>) -> Result<(), Report> {
    mount_orphans(
        page,
        "combo-box",
        &["A <ComboBoxPopover> must be inside a <ComboBox>."],
    )
    .await
}

/// A `RadioField` outside a `RadioGroup` warns and renders nothing.
#[browser_test]
pub async fn radio_parts(page: &Page<'_>) -> Result<(), Report> {
    mount_orphans(
        page,
        "radio",
        &["A <RadioField> must be inside a <RadioGroup>."],
    )
    .await
}

/// Menu items, sections, submenu triggers and item slots outside their `Menu` or `MenuItem` warn
/// and render nothing.
#[browser_test]
pub async fn menu_parts(page: &Page<'_>) -> Result<(), Report> {
    mount_orphans(
        page,
        "menu",
        &[
            "A <MenuItem> must be inside a <Menu>.",
            "<MenuItems> must be inside a <Menu>.",
            "A <MenuSection> must be inside a <Menu>.",
            "A <SubmenuTrigger> must be inside a <Menu>.",
            "A <MenuItemLabel> must be inside a <MenuItem>.",
            "A <MenuItemDescription> must be inside a <MenuItem>.",
            "A <MenuItemShortcut> must be inside a <MenuItem>.",
        ],
    )
    .await
}

/// A `CheckboxField` in a `CheckboxGroup` without a `value` warns and renders nothing, while the
/// group renders.
#[browser_test]
pub async fn checkbox_field_without_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#orphans-checkbox-mount")
        .await?
        .click()
        .await?;
    let orphans = page.element("#orphans-checkbox").await?;
    orphans.wait_for_attr("data-mounted", Some("true")).await?;
    crate::fixtures::take_warnings(
        page,
        "A <CheckboxField> in a <CheckboxGroup> needs a `value`.",
        1,
    )
    .await?;
    assert_that!(orphans.count(".leptonic-CheckboxGroup").await?).is_equal_to(1);
    assert_that!(orphans.count(".leptonic-CheckboxField").await?).is_equal_to(0);
    assert_that!(orphans.count("input").await?).is_equal_to(0);
    Ok(())
}
