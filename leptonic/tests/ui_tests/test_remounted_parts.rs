// No upstream: React passes a part's props through context on every render; leptonic's props bind
// a hook to one element (`documentation/hooks-implementation.md`, "Props are Single-Use").
//! Atom parts mounted again take their props again: the same id, referenced again by the
//! element they belong to. A second part mounted beside the first gets none (no duplicate id, no
//! second reference) and a warning in debug builds (`leptonic::Part`).
use assertr::prelude::*;
use browser_test::browser_test;
use rootcause::Report;

use leptonic::AriaRole;

use crate::pages::{Page, role};

const PATH: &str = "/atoms/remounted-parts";

/// The warning when a second element takes a part's props.
const SECOND_PART: &str = "one element at a time";

/// A part of the fixture's section `#rp-<name>`.
struct Case {
    name: &'static str,
    /// The part's elements, in the section.
    part: &'static str,
    /// The element referencing the part's id, and the attribute.
    reference: Option<(&'static str, &'static str)>,
    /// Whether the reference disappears while the part is unmounted (a slot's), or stays (a
    /// label's `for`).
    reference_follows: bool,
    /// Whether a second part without props renders an element (a text part) or nothing (a part
    /// that only exists to carry its props: an input, a panel, a button).
    second_renders: bool,
}

const DESCRIPTION: Case = Case {
    name: "description",
    part: ".leptonic-Description",
    reference: Some(("input", "aria-describedby")),
    reference_follows: true,
    second_renders: true,
};
const FIELD_ERROR: Case = Case {
    name: "field-error",
    part: ".leptonic-FieldError",
    reference: Some(("input", "aria-describedby")),
    reference_follows: true,
    second_renders: true,
};
const LABEL: Case = Case {
    name: "label",
    part: ".leptonic-Label",
    reference: Some(("input", "aria-labelledby")),
    reference_follows: true,
    second_renders: true,
};
const INPUT: Case = Case {
    name: "input",
    part: "input",
    reference: Some(("label", "for")),
    reference_follows: false,
    second_renders: false,
};
const OPTION_LABEL: Case = Case {
    name: "option-label",
    part: ".leptonic-ListBoxItemLabel",
    reference: Some(("[role=option]", "aria-labelledby")),
    reference_follows: true,
    second_renders: true,
};
const OPTION_DESCRIPTION: Case = Case {
    name: "option-description",
    part: ".leptonic-ListBoxItemDescription",
    reference: Some(("[role=option]", "aria-describedby")),
    reference_follows: true,
    second_renders: true,
};
const SECTION_HEADING: Case = Case {
    name: "section-heading",
    part: ".leptonic-ListBoxSectionHeading",
    reference: Some(("[role=group]", "aria-labelledby")),
    reference_follows: true,
    second_renders: true,
};
const DIALOG_TITLE: Case = Case {
    name: "dialog-title",
    part: ".leptonic-DialogTitle",
    reference: Some(("[role=dialog]", "aria-labelledby")),
    reference_follows: true,
    second_renders: true,
};
const DISCLOSURE_PANEL: Case = Case {
    name: "disclosure-panel",
    part: ".leptonic-DisclosurePanel",
    reference: Some((".leptonic-Button", "aria-controls")),
    reference_follows: true,
    second_renders: false,
};
const MENU_ITEM_LABEL: Case = Case {
    name: "menu-item-label",
    part: ".leptonic-MenuItemLabel",
    reference: Some(("[role=menuitem]", "aria-labelledby")),
    reference_follows: true,
    second_renders: true,
};
const CHECKBOX_BUTTON: Case = Case {
    name: "checkbox-button",
    part: "input[type=checkbox]",
    reference: None,
    reference_follows: false,
    second_renders: false,
};

const DIALOG_DESCRIPTION: Case = Case {
    name: "dialog-description",
    part: ".leptonic-DialogDescription",
    reference: Some(("[role=alertdialog]", "aria-describedby")),
    reference_follows: true,
    second_renders: true,
};
const SLIDER_OUTPUT: Case = Case {
    name: "slider-output",
    part: ".leptonic-SliderOutput",
    reference: None,
    reference_follows: false,
    second_renders: true,
};
const SLIDER_TRACK: Case = Case {
    name: "slider-track",
    part: ".leptonic-SliderTrack",
    reference: None,
    reference_follows: false,
    second_renders: false,
};

/// Takes the one expected warning about a second part.
async fn take_second_part_warning(page: &Page<'_>) {
    assert_that!(|| crate::fixtures::take_warnings(page, SECOND_PART, 1))
        .with_subject_name("the warning about the second part")
        .eventually_ok()
        .satisfies(|taken| {
            taken.is_equal_to(());
        })
        .await;
}

impl Case {
    fn section(&self) -> String {
        format!("#rp-{}", self.name)
    }

    fn parts(&self) -> String {
        format!("{} {}", self.section(), self.part)
    }

    async fn toggle(&self, page: &Page<'_>) -> Result<(), Report> {
        page.element(format!("#rp-{}-toggle", self.name))
            .await?
            .click()
            .await?;
        Ok(())
    }

    /// Waits until the reference contains `id` (`true`) or doesn't (`false`).
    async fn wait_for_reference(
        &self,
        page: &Page<'_>,
        id: &str,
        referenced: bool,
    ) -> Result<(), Report> {
        let Some((element, attribute)) = self.reference else {
            return Ok(());
        };
        let element = page
            .element(format!("{} {element}", self.section()))
            .await?;
        assert_that!(|| async {
            let value = element.attr(attribute).await?.unwrap_or_default();
            Ok::<_, Report>(value.split_whitespace().any(|token| token == id))
        })
        .with_subject_name(format!("whether {attribute} references {id:?}"))
        .eventually_ok()
        .satisfies(|references| {
            references.is_equal_to(referenced);
        })
        .await;
        Ok(())
    }

    /// The part keeps its id when it is unmounted and mounted again, and is referenced again.
    async fn mounts_again(&self, page: &Page<'_>) -> Result<(), Report> {
        page.goto_path(PATH).await?;
        let parts = self.parts();
        page.wait_for_count(parts.as_str(), 1).await?;
        let id = page.element(parts.as_str()).await?.attr("id").await?;
        if let Some(id) = &id {
            self.wait_for_reference(page, id, true).await?;
        }
        self.toggle(page).await?;
        page.wait_for_count(parts.as_str(), 0).await?;
        if let Some(id) = &id
            && self.reference_follows
        {
            self.wait_for_reference(page, id, false).await?;
        }
        self.toggle(page).await?;
        page.wait_for_count(parts.as_str(), 1).await?;
        let again = page.element(parts.as_str()).await?.attr("id").await?;
        assert_that!(again)
            .with_subject_name("the id of the part mounted again")
            .is_equal_to(id.clone());
        if let Some(id) = &id {
            self.wait_for_reference(page, id, true).await?;
        }
        Ok(())
    }

    /// A second part gets no props (no id, nothing referencing it) and warns; the first keeps
    /// them.
    async fn second_gets_none(&self, page: &Page<'_>) -> Result<(), Report> {
        page.goto_path(PATH).await?;
        let parts = self.parts();
        page.wait_for_count(parts.as_str(), 1).await?;
        let id = page.element(parts.as_str()).await?.attr("id").await?;
        page.element(format!("#rp-{}-second", self.name))
            .await?
            .click()
            .await?;
        page.wait_for_attr(self.section().as_str(), "data-second", Some("true"))
            .await?;
        take_second_part_warning(page).await;
        let expected = if self.second_renders { 2 } else { 1 };
        page.wait_for_count(parts.as_str(), expected).await?;
        let elements = page.elements(parts.as_str()).await?;
        assert_that!(elements[0].attr("id").await?)
            .with_subject_name("the first part's id")
            .is_equal_to(id.clone());
        if self.second_renders {
            assert_that!(elements[1].attr("id").await?)
                .with_subject_name("the second part's id")
                .is_none();
        }
        if let Some(id) = &id {
            self.wait_for_reference(page, id, true).await?;
        }
        Ok(())
    }
}

/// A text field's `Description` mounted again describes the field again.
#[browser_test]
pub async fn description_mounts_again(page: &Page<'_>) -> Result<(), Report> {
    DESCRIPTION.mounts_again(page).await
}

/// A second `Description` in a field gets no id and warns.
#[browser_test]
pub async fn second_description(page: &Page<'_>) -> Result<(), Report> {
    DESCRIPTION.second_gets_none(page).await
}

/// An invalid field's `FieldError` mounted again describes the field again.
#[browser_test]
pub async fn field_error_mounts_again(page: &Page<'_>) -> Result<(), Report> {
    FIELD_ERROR.mounts_again(page).await
}

/// A second `FieldError` in a field gets no id and warns.
#[browser_test]
pub async fn second_field_error(page: &Page<'_>) -> Result<(), Report> {
    FIELD_ERROR.second_gets_none(page).await
}

/// A text field's `Label` mounted again labels the field again.
#[browser_test]
pub async fn label_mounts_again(page: &Page<'_>) -> Result<(), Report> {
    LABEL.mounts_again(page).await
}

/// A second `Label` in a field gets no id and warns.
#[browser_test]
pub async fn second_label(page: &Page<'_>) -> Result<(), Report> {
    LABEL.second_gets_none(page).await
}

/// A text field's `Input` mounted again keeps the id its label refers to.
#[browser_test]
pub async fn input_mounts_again(page: &Page<'_>) -> Result<(), Report> {
    INPUT.mounts_again(page).await
}

/// A second `Input` in a field renders nothing and warns.
#[browser_test]
pub async fn second_input(page: &Page<'_>) -> Result<(), Report> {
    INPUT.second_gets_none(page).await
}

/// A `ListBoxItemLabel` mounted again labels its option again.
#[browser_test]
pub async fn option_label_mounts_again(page: &Page<'_>) -> Result<(), Report> {
    OPTION_LABEL.mounts_again(page).await
}

/// A second `ListBoxItemLabel` in an option gets no id and warns.
#[browser_test]
pub async fn second_option_label(page: &Page<'_>) -> Result<(), Report> {
    OPTION_LABEL.second_gets_none(page).await
}

/// A `ListBoxItemDescription` mounted again describes its option again.
#[browser_test]
pub async fn option_description_mounts_again(page: &Page<'_>) -> Result<(), Report> {
    OPTION_DESCRIPTION.mounts_again(page).await
}

/// A second `ListBoxItemDescription` in an option gets no id and warns.
#[browser_test]
pub async fn second_option_description(page: &Page<'_>) -> Result<(), Report> {
    OPTION_DESCRIPTION.second_gets_none(page).await
}

/// A second `ListBoxSectionHeading` in a section gets no id and warns.
#[browser_test]
pub async fn second_section_heading(page: &Page<'_>) -> Result<(), Report> {
    SECTION_HEADING.second_gets_none(page).await
}

/// A `DialogTitle` mounted again names its dialog again.
#[browser_test]
pub async fn dialog_title_mounts_again(page: &Page<'_>) -> Result<(), Report> {
    DIALOG_TITLE.mounts_again(page).await
}

/// A second `DialogTitle` in a dialog gets no id and warns.
#[browser_test]
pub async fn second_dialog_title(page: &Page<'_>) -> Result<(), Report> {
    DIALOG_TITLE.second_gets_none(page).await
}

/// A second `DisclosurePanel` in a disclosure renders nothing and warns.
#[browser_test]
pub async fn second_disclosure_panel(page: &Page<'_>) -> Result<(), Report> {
    DISCLOSURE_PANEL.second_gets_none(page).await
}

/// A `MenuItemLabel` mounted again labels its item again.
#[browser_test]
pub async fn menu_item_label_mounts_again(page: &Page<'_>) -> Result<(), Report> {
    MENU_ITEM_LABEL.mounts_again(page).await
}

/// A second `MenuItemLabel` in an item gets no id and warns.
#[browser_test]
pub async fn second_menu_item_label(page: &Page<'_>) -> Result<(), Report> {
    MENU_ITEM_LABEL.second_gets_none(page).await
}

/// A second `CheckboxButton` in a field renders nothing and warns.
#[browser_test]
pub async fn second_checkbox_button(page: &Page<'_>) -> Result<(), Report> {
    CHECKBOX_BUTTON.second_gets_none(page).await
}

/// An alert dialog's `DialogDescription` mounted again describes the dialog again.
#[browser_test]
pub async fn dialog_description_mounts_again(page: &Page<'_>) -> Result<(), Report> {
    DIALOG_DESCRIPTION.mounts_again(page).await
}

/// A second `DialogDescription` in a dialog gets no id and warns.
#[browser_test]
pub async fn second_dialog_description(page: &Page<'_>) -> Result<(), Report> {
    DIALOG_DESCRIPTION.second_gets_none(page).await
}

/// A `SliderOutput` mounted again shows the slider's value again.
#[browser_test]
pub async fn slider_output_mounts_again(page: &Page<'_>) -> Result<(), Report> {
    SLIDER_OUTPUT.mounts_again(page).await?;
    assert_that!(
        page.element("#rp-slider-output .leptonic-SliderOutput")
            .await?
    )
    .has_attribute("for")
    .await
    .is_not_empty();
    Ok(())
}

/// A second `SliderOutput` in a slider shows the value without the output's attributes, and
/// warns.
#[browser_test]
pub async fn second_slider_output(page: &Page<'_>) -> Result<(), Report> {
    SLIDER_OUTPUT.second_gets_none(page).await?;
    let outputs = page
        .elements("#rp-slider-output .leptonic-SliderOutput")
        .await?;
    assert_that!(outputs[1].attr("for").await?)
        .with_subject_name("the second output's for")
        .is_none();
    Ok(())
}

/// A `SliderTrack` mounted again (with its thumb) is the slider's track again.
#[browser_test]
pub async fn slider_track_mounts_again(page: &Page<'_>) -> Result<(), Report> {
    SLIDER_TRACK.mounts_again(page).await?;
    page.wait_for_count(
        "#rp-slider-track [role=slider], #rp-slider-track input[type=range]",
        1,
    )
    .await
}

/// A second `SliderTrack` in a slider renders nothing and warns.
#[browser_test]
pub async fn second_slider_track(page: &Page<'_>) -> Result<(), Report> {
    SLIDER_TRACK.second_gets_none(page).await
}

/// A second `ModalContent` in a backdrop renders nothing and warns; the first is the modal.
#[browser_test]
pub async fn second_modal_content(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#rp-modal-open").await?.click().await?;
    page.wait_for_count(role(AriaRole::Dialog).text("Inside"), 1)
        .await?;
    take_second_part_warning(page).await;
    page.wait_for_count(".leptonic-ModalContent", 1).await?;
    assert_that!(page.element(".leptonic-ModalContent").await?)
        .has_attribute("id")
        .await
        .is_not_empty();
    Ok(())
}

/// A second `Menu` in a submenu's popover renders nothing and warns; the first is the submenu
/// its trigger controls.
#[browser_test]
pub async fn second_submenu_menu(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let share = page.element(role(AriaRole::Menuitem).text("Share")).await?;
    share.click().await?;
    page.wait_for_count(role(AriaRole::Menuitem).text("Email"), 1)
        .await?;
    take_second_part_warning(page).await;
    page.wait_for_count(role(AriaRole::Menuitem).text("Second email"), 0)
        .await?;
    let submenu = page
        .element(role(AriaRole::Menu).has(role(AriaRole::Menuitem).text("Email")))
        .await?;
    let submenu_id = submenu.attr("id").await?;
    assert_that!(share.attr("aria-controls").await?)
        .with_subject_name("the trigger's aria-controls")
        .is_equal_to(submenu_id);
    Ok(())
}
