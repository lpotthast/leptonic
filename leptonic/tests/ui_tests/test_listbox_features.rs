// Upstream: react-aria-components/test/ListBox.test.js @ 99e6102368
// Upstream: react-aria/test/selection/useSelectableCollection.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// "should support sections" (one group element per section, named by its header or
/// `aria-label`), separators inside a listbox, and arrow keys across sections.
pub struct ListBoxSectionsTests {}

#[async_trait]
impl BrowserTest<str> for ListBoxSectionsTests {
    fn name(&self) -> Cow<'_, str> {
        "listbox_sections_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/listbox-features").await?;

        let groups = page
            .driver
            .find_all(By::Css("#lbf-sections [role=group]"))
            .await?;
        assert_that!(groups.len()).is_equal_to(2);
        for group in &groups {
            assert_that!(group.class_name().await?)
                .is_equal_to(Some("leptonic-ListBoxSection".to_owned()));
            // The section is the group itself, directly in the listbox.
            assert_that!(group.tag_name().await?).is_equal_to("section".to_owned());
        }
        let heading_id = groups[0].attr("aria-labelledby").await?.unwrap_or_default();
        let heading = page.element(&heading_id).await?;
        assert_that!(heading.text().await?).is_equal_to("Veggies".to_owned());
        assert_that!(heading.attr("role").await?).is_equal_to(Some("presentation".to_owned()));
        assert_that!(groups[1].attr("aria-label").await?).is_equal_to(Some("Protein".to_owned()));
        // A separator between the options is no `<hr>` (invalid inside a listbox).
        let separator = page
            .css("#lbf-sections [role=listbox] > [role=separator]")
            .await?;
        assert_that!(separator.tag_name().await?).is_equal_to("div".to_owned());

        option(&page, "#lbf-sections", "Tomato")
            .await?
            .click()
            .await?;
        page.wait_for_active_text("Tomato").await?;
        for (key, expected) in [
            (Key::Down, "Onion"),
            (Key::Down, "Ham"),
            (Key::Up, "Onion"),
            (Key::End, "Tofu"),
            (Key::Home, "Lettuce"),
        ] {
            page.send_keys_to_active(key).await?;
            page.wait_for_active_text(expected).await?;
        }
        page.expect_no_page_errors().await
    }
}

/// `selectionBehavior="replace"`: replacing selection on press and focus, modifier keys toggle
/// or extend, the action on double click and Enter ("should trigger onAction on double click if
/// selectionBehavior="replace"", "should perform toggle selection in highlight mode when using
/// modifier keys").
pub struct ListBoxReplaceSelectionTests {}

#[async_trait]
impl BrowserTest<str> for ListBoxReplaceSelectionTests {
    fn name(&self) -> Cow<'_, str> {
        "listbox_replace_selection_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/listbox-features").await?;
        let selection = "lbf-replace-selection";

        // "should support hover": interactive options show it, others don't.
        let cat = option(&page, "#lbf-replace", "Cat").await?;
        hover(&page, &cat).await?;
        page.wait_for_attr(&cat, "data-hovered", Some("true"))
            .await?;
        let plain = option(&page, "#lbf-horizontal", "Dog").await?;
        hover(&page, &plain).await?;
        page.wait_for_attr(&cat, "data-hovered", None).await?;
        stays!("data-hovered", None, plain.attr("data-hovered").await?);

        option(&page, "#lbf-replace", "Cat").await?.click().await?;
        page.wait_for_text(selection, "Cat").await?;
        option(&page, "#lbf-replace", "Dog").await?.click().await?;
        page.wait_for_text(selection, "Dog").await?;
        // Ctrl+click toggles, Shift+click extends.
        let kangaroo = option(&page, "#lbf-replace", "Kangaroo").await?;
        page.driver
            .action_chain()
            .key_down(Key::Control)
            .click_element(&kangaroo)
            .key_up(Key::Control)
            .perform()
            .await?;
        page.wait_for_text(selection, "Dog,Kangaroo").await?;
        let cat = option(&page, "#lbf-replace", "Cat").await?;
        page.driver
            .action_chain()
            .key_down(Key::Shift)
            .click_element(&cat)
            .key_up(Key::Shift)
            .perform()
            .await?;
        page.wait_for_text(selection, "Cat,Dog,Kangaroo").await?;
        // A double click performs the action (and selects).
        let dog = option(&page, "#lbf-replace", "Dog").await?;
        page.driver
            .action_chain()
            .double_click_element(&dog)
            .perform()
            .await?;
        page.wait_for_text("lbf-replace-actions", "Dog").await?;
        page.wait_for_text(selection, "Dog").await?;

        // Arrow keys move the selection with the focus; Ctrl moves focus only.
        page.wait_for_active_text("Dog").await?;
        page.send_keys_to_active(Key::Down).await?;
        page.wait_for_active_text("Kangaroo").await?;
        page.wait_for_text(selection, "Kangaroo").await?;
        page.send_keys_to_active(Key::Control + Key::Up).await?;
        page.wait_for_active_text("Dog").await?;
        stays!(
            "the selection",
            "Kangaroo".to_owned(),
            page.read_text_of(selection).await?
        );
        // Ctrl+Space toggles the focused option; Enter performs its action.
        page.send_keys_to_active(Key::Control + " ").await?;
        page.wait_for_text(selection, "Dog,Kangaroo").await?;
        page.send_keys_to_active(Key::Enter).await?;
        page.wait_for_text("lbf-replace-actions", "Dog,Dog").await?;
        page.expect_no_page_errors().await
    }
}

/// "should support onAction" (mouse and keyboard) without selection, and links: "should support
/// links with selectionMode="none"" and "single" (the link opens, nothing is selected); arrow
/// keys onto link items are handled (no native scrolling).
pub struct ListBoxActionsAndLinksTests {}

#[async_trait]
impl BrowserTest<str> for ListBoxActionsAndLinksTests {
    fn name(&self) -> Cow<'_, str> {
        "listbox_actions_and_links_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/listbox-features").await?;

        option(&page, "#lbf-action", "Cat").await?.click().await?;
        page.wait_for_text("lbf-action-actions", "Cat").await?;
        page.wait_for_active_text("Cat").await?;
        page.send_keys_to_active(Key::Down).await?;
        page.wait_for_active_text("Dog").await?;
        page.send_keys_to_active(Key::Enter).await?;
        page.wait_for_text("lbf-action-actions", "Cat,Dog").await?;
        assert_that!(
            option(&page, "#lbf-action", "Dog")
                .await?
                .attr("aria-selected")
                .await?
        )
        .is_none();

        // Links open on press, also with single selection, and aren't selected.
        option(&page, "#lbf-links", "One").await?.click().await?;
        wait_for!(
            "the location hash",
            "#lbf-one".to_owned(),
            hash(&page).await?
        );
        page.wait_for_active_text("One").await?;
        // ArrowDown onto a link: handled (the default, scrolling, is prevented).
        let prevented = page
            .driver
            .execute(
                "let event = new KeyboardEvent('keydown', \
                     {key: 'ArrowDown', bubbles: true, cancelable: true}); \
                 document.activeElement.dispatchEvent(event); \
                 return event.defaultPrevented;",
                vec![],
            )
            .await?
            .convert::<bool>()?;
        assert_that!(prevented).is_true();
        page.wait_for_active_text("Two").await?;
        page.send_keys_to_active(Key::Enter).await?;
        wait_for!(
            "the location hash",
            "#lbf-two".to_owned(),
            hash(&page).await?
        );

        let one = option(&page, "#lbf-links-single", "One").await?;
        one.click().await?;
        wait_for!(
            "the location hash",
            "#lbf-one".to_owned(),
            hash(&page).await?
        );
        stays!(
            "aria-selected",
            Some("false".to_owned()),
            one.attr("aria-selected").await?
        );
        page.expect_no_page_errors().await
    }
}

/// "should support horizontal orientation" (also right-to-left), "should support grid layout",
/// PageDown/PageUp, `shouldFocusWrap`.
pub struct ListBoxLayoutTests {}

#[async_trait]
impl BrowserTest<str> for ListBoxLayoutTests {
    fn name(&self) -> Cow<'_, str> {
        "listbox_layout_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/listbox-features").await?;

        for (container, keys) in [
            (
                "#lbf-horizontal",
                [
                    (Key::Right, "Dog"),
                    (Key::Right, "Kangaroo"),
                    (Key::Left, "Dog"),
                ],
            ),
            (
                "#lbf-rtl",
                [
                    (Key::Left, "Dog"),
                    (Key::Left, "Kangaroo"),
                    (Key::Right, "Dog"),
                ],
            ),
            (
                "#lbf-grid",
                [
                    (Key::Down, "Kangaroo"),
                    (Key::Left, "Dog"),
                    (Key::Left, "Cat"),
                ],
            ),
            (
                "#lbf-wrap",
                [
                    (Key::Up, "Kangaroo"),
                    (Key::Down, "Cat"),
                    (Key::Down, "Dog"),
                ],
            ),
        ] {
            option(&page, container, "Cat").await?.click().await?;
            page.wait_for_active_text("Cat").await?;
            for (key, expected) in keys {
                page.send_keys_to_active(key.clone()).await?;
                page.wait_for_active_text(expected).await.map_err(|e| {
                    e.context(format!("{container}: after pressing {key:?}"))
                        .into_dynamic()
                })?;
            }
        }

        // PageDown moves a page of options down, PageUp back.
        option(&page, "#lbf-page", "Option 1")
            .await?
            .click()
            .await?;
        page.wait_for_active_text("Option 1").await?;
        page.send_keys_to_active(Key::PageDown).await?;
        wait_for!("focus to move down a page", true, {
            let text = page.active_element_text().await?;
            let number: u32 = text
                .strip_prefix("Option ")
                .and_then(|n| n.parse().ok())
                .unwrap_or_default();
            number >= 4
        });
        page.send_keys_to_active(Key::PageUp).await?;
        page.wait_for_active_text("Option 1").await?;
        page.expect_no_page_errors().await
    }
}

/// `disabledBehavior="selection"` (on the listbox and on one item: focusable but not
/// selectable), "should support empty state", and focus moving on when the focused option is
/// removed.
pub struct ListBoxDisabledAndEmptyTests {}

#[async_trait]
impl BrowserTest<str> for ListBoxDisabledAndEmptyTests {
    fn name(&self) -> Cow<'_, str> {
        "listbox_disabled_and_empty_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/listbox-features").await?;

        for container in ["#lbf-disabled-selection", "#lbf-item-disabled-behavior"] {
            option(&page, container, "Cat").await?.click().await?;
            page.wait_for_active_text("Cat").await?;
            page.send_keys_to_active(Key::Down).await?;
            page.wait_for_active_text("Dog").await?;
            let dog = option(&page, container, "Dog").await?;
            assert_that!(dog.attr("aria-disabled").await?).is_none();
            page.send_keys_to_active(" ").await?;
            dog.click().await?;
            stays!(
                "Dog's aria-selected",
                Some("false".to_owned()),
                dog.attr("aria-selected").await?
            );
        }
        // An item disabled without its own behavior is skipped.
        page.send_keys_to_active(Key::Down).await?;
        stays!(
            "the focused option",
            "Dog".to_owned(),
            page.active_element_text().await?
        );

        let empty = page.css("#lbf-empty [role=listbox]").await?;
        assert_that!(empty.attr("data-empty").await?).is_equal_to(Some("true".to_owned()));
        assert_that!(page.css("#lbf-empty [role=option]").await?.text().await?)
            .is_equal_to("No results".to_owned());

        // Removing the focused option moves focus to the next one.
        option(&page, "#lbf-removal", "Dog").await?.click().await?;
        page.wait_for_active_text("Dog").await?;
        page.driver
            .execute("document.getElementById('lbf-remove-dog').click();", vec![])
            .await?;
        page.wait_for_count("#lbf-removal [role=option]", 2).await?;
        page.wait_for_active_text("Kangaroo").await?;
        page.expect_no_page_errors().await
    }
}

/// The option with the text `text` in the listbox inside `container`.
async fn option(page: &Page<'_>, container: &str, text: &str) -> Result<WebElement, Report> {
    let container = page.css(container).await?;
    Ok(container
        .find(By::XPath(format!(
            ".//*[@role='option'][normalize-space(.)='{text}']"
        )))
        .await?)
}

async fn hash(page: &Page<'_>) -> Result<String, Report> {
    Ok(page
        .driver
        .execute("return window.location.hash;", vec![])
        .await?
        .convert::<String>()?)
}

/// Moves the pointer over `element`.
async fn hover(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    page.driver
        .action_chain()
        .move_to_element_center(element)
        .perform()
        .await?;
    Ok(())
}
