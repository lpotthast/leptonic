// Upstream: react-aria-components/test/RadioGroup.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// Behavior of the radio hooks (through the `RadioGroup` and `Radio` atoms): ARIA structure,
/// selection by press, the group as one tab stop (roving tabindex), arrow keys moving the
/// selection (wrapping, skipping disabled radios, both orientations), disabled and read-only
/// groups, and native required validation. Spec: react-aria-components `RadioGroup.test.js`.
pub struct RadioGroupTests {}

#[async_trait]
impl BrowserTest<str> for RadioGroupTests {
    fn name(&self) -> Cow<'_, str> {
        "radio_group_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/radio-group").await?;

        structure(&page).await?;
        tab_enters_and_leaves_the_group(&page).await?;
        selection_by_press(&page).await?;
        virtual_label_click(&page).await?;
        arrow_keys(&page).await?;
        selected_radio_is_the_tab_stop(&page).await?;
        skips_disabled_radios(&page).await?;
        horizontal(&page).await?;
        disabled_group(&page).await?;
        read_only_group(&page).await?;
        validation(&page).await?;

        Ok(())
    }
}

async fn label(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    Ok(page
        .driver
        .find(By::XPath(format!("//label[normalize-space(.)='{text}']")))
        .await?)
}

async fn radio(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    Ok(label(page, text).await?.find(By::Css("input")).await?)
}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

async fn structure(page: &Page<'_>) -> Result<(), Report> {
    let group = page.css("[role=radiogroup]").await?;
    let group_label = page
        .driver
        .find(By::XPath("//span[normalize-space(.)='Favorite pet']"))
        .await?;
    let label_id = attr(&group_label, "id").await?;
    assert_that!(attr(&group, "aria-labelledby").await?).is_equal_to(label_id);
    assert_that!(attr(&group, "aria-orientation").await?).is_equal_to(Some("vertical".to_owned()));
    assert_that!(attr(&group, "data-orientation").await?).is_equal_to(Some("vertical".to_owned()));

    let dogs = radio(page, "Dogs").await?;
    let cats = radio(page, "Cats").await?;
    assert_that!(attr(&dogs, "type").await?).is_equal_to(Some("radio".to_owned()));
    let name = attr(&dogs, "name").await?;
    assert_that!(name.is_some()).is_true();
    assert_that!(attr(&cats, "name").await?).is_equal_to(name);
    assert_that!(attr(&cats, "value").await?).is_equal_to(Some("cats".to_owned()));
    // The group's description describes each radio.
    let description = page
        .driver
        .find(By::XPath("//*[normalize-space(.)='Pick one.']"))
        .await?;
    let description_id = attr(&description, "id").await?.unwrap_or_default();
    assert_that!(attr(&dogs, "aria-describedby").await?.unwrap_or_default())
        .contains(description_id.as_str());
    Ok(())
}

/// Tab enters the group at a radio and leaves it with the next Tab (react-aria: "should not
/// navigate within the group using Tab").
async fn tab_enters_and_leaves_the_group(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-rg-before").await?;
    page.press_tab().await?;
    let dogs = radio(page, "Dogs").await?;
    page.wait_for_focus_on(&dogs, "Dogs").await?;
    page.wait_for_attr(
        &label(page, "Dogs").await?,
        "data-focus-visible",
        Some("true"),
    )
    .await?;
    page.press_tab().await?;
    let after = page.element("test-rg-after").await?;
    page.wait_for_focus_on(&after, "the button after the group")
        .await?;
    page.press_shift_tab().await?;
    page.wait_for_focus_on(&dogs, "Dogs").await?;
    // Focusing doesn't select.
    assert_that!(page.read_text_of("test-rg-value").await?).is_equal_to(String::new());
    Ok(())
}

async fn selection_by_press(page: &Page<'_>) -> Result<(), Report> {
    let cats = label(page, "Cats").await?;
    cats.click().await?;
    page.wait_for_text("test-rg-value", "cats").await?;
    page.wait_for_attr(&cats, "data-selected", Some("true"))
        .await?;
    assert_that!(radio(page, "Cats").await?.prop("checked").await?)
        .is_equal_to(Some("true".to_owned()));
    let dogs = label(page, "Dogs").await?;
    dogs.click().await?;
    page.wait_for_text("test-rg-value", "dogs").await?;
    page.wait_for_attr(&cats, "data-selected", None).await
}

/// Arrow keys select the next/previous radio, wrapping around.
async fn arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    let dogs = radio(page, "Dogs").await?;
    let cats = radio(page, "Cats").await?;
    let dragons = radio(page, "Dragons").await?;
    page.click_element_with_id("test-rg-before").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&dogs, "Dogs").await?;

    page.send_keys_to_active(Key::Down).await?;
    page.wait_for_focus_on(&cats, "Cats").await?;
    page.wait_for_text("test-rg-value", "cats").await?;
    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_focus_on(&dragons, "Dragons").await?;
    page.wait_for_text("test-rg-value", "dragons").await?;
    page.send_keys_to_active(Key::Down).await?;
    page.wait_for_focus_on(&dogs, "Dogs (wrapped)").await?;
    page.wait_for_text("test-rg-value", "dogs").await?;
    page.send_keys_to_active(Key::Up).await?;
    page.wait_for_focus_on(&dragons, "Dragons (wrapped back)")
        .await?;
    page.wait_for_text("test-rg-value", "dragons").await?;
    page.send_keys_to_active(Key::Left).await?;
    page.wait_for_focus_on(&cats, "Cats").await?;
    page.wait_for_text("test-rg-value", "cats").await
}

/// With a selection, only the selected radio is a tab stop.
async fn selected_radio_is_the_tab_stop(page: &Page<'_>) -> Result<(), Report> {
    // Cats is selected (previous step).
    let cats = radio(page, "Cats").await?;
    assert_that!(attr(&cats, "tabindex").await?).is_equal_to(Some("0".to_owned()));
    assert_that!(attr(&radio(page, "Dogs").await?, "tabindex").await?)
        .is_equal_to(Some("-1".to_owned()));
    page.click_element_with_id("test-rg-before").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&cats, "Cats").await
}

async fn skips_disabled_radios(page: &Page<'_>) -> Result<(), Report> {
    let a = label(page, "Skip A").await?;
    a.click().await?;
    let a_input = radio(page, "Skip A").await?;
    page.wait_for_focus_on(&a_input, "Skip A").await?;
    assert_that!(attr(&radio(page, "Skip B").await?, "disabled").await?).is_some();
    page.send_keys_to_active(Key::Down).await?;
    let c = radio(page, "Skip C").await?;
    page.wait_for_focus_on(&c, "Skip C").await?;
    page.wait_for_attr(&label(page, "Skip C").await?, "data-selected", Some("true"))
        .await
}

async fn horizontal(page: &Page<'_>) -> Result<(), Report> {
    let group = page
        .css("[role=radiogroup][aria-label='Horizontal']")
        .await?;
    assert_that!(attr(&group, "aria-orientation").await?)
        .is_equal_to(Some("horizontal".to_owned()));
    let b = label(page, "Horizontal B").await?;
    assert_that!(attr(&b, "data-selected").await?).is_equal_to(Some("true".to_owned()));
    b.click().await?;
    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_attr(
        &label(page, "Horizontal C").await?,
        "data-selected",
        Some("true"),
    )
    .await?;
    page.send_keys_to_active(Key::Left).await?;
    page.wait_for_attr(&b, "data-selected", Some("true")).await
}

async fn disabled_group(page: &Page<'_>) -> Result<(), Report> {
    let group = page
        .css("[role=radiogroup][aria-label='Disabled group']")
        .await?;
    assert_that!(attr(&group, "aria-disabled").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&group, "data-disabled").await?).is_equal_to(Some("true".to_owned()));
    let a = label(page, "Disabled group A").await?;
    assert_that!(attr(&a, "data-disabled").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&radio(page, "Disabled group A").await?, "disabled").await?).is_some();
    a.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert_that!(attr(&a, "data-selected").await?).is_none();
    Ok(())
}

async fn read_only_group(page: &Page<'_>) -> Result<(), Report> {
    let group = page
        .css("[role=radiogroup][aria-label='Read-only group']")
        .await?;
    assert_that!(attr(&group, "aria-readonly").await?).is_equal_to(Some("true".to_owned()));
    let a = label(page, "Read-only A").await?;
    let b = label(page, "Read-only B").await?;
    b.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert_that!(attr(&a, "data-selected").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&b, "data-selected").await?).is_none();
    assert_that!(radio(page, "Read-only A").await?.prop("checked").await?)
        .is_equal_to(Some("true".to_owned()));
    assert_that!(radio(page, "Read-only B").await?.prop("checked").await?)
        .is_equal_to(Some("false".to_owned()));

    // Space on another radio: the browser unchecks A natively, the group restores it.
    radio(page, "Read-only A").await?.focus().await?;
    page.send_keys_to_active(Key::Down).await?;
    let b_input = radio(page, "Read-only B").await?;
    page.wait_for_focus_on(&b_input, "Read-only B").await?;
    page.send_keys_to_active(Key::Space).await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert_that!(radio(page, "Read-only A").await?.prop("checked").await?)
        .is_equal_to(Some("true".to_owned()));
    assert_that!(b_input.prop("checked").await?).is_equal_to(Some("false".to_owned()));
    assert_that!(attr(&a, "data-selected").await?).is_equal_to(Some("true".to_owned()));
    Ok(())
}

/// Native required validation, also with the last radio disabled (react-aria: "supports
/// validation errors when last radio is disabled").
async fn validation(page: &Page<'_>) -> Result<(), Report> {
    let form = page.element("test-rg-form").await?;
    let group = form.find(By::Css("[role=radiogroup]")).await?;
    let a = radio(page, "Required A").await?;
    let b = radio(page, "Required B").await?;
    for input in [&a, &b] {
        assert_that!(attr(input, "required").await?).is_some();
        assert_that!(attr(input, "aria-required").await?).is_none();
    }
    assert_that!(attr(&group, "data-invalid").await?).is_none();
    let described = attr(&group, "aria-describedby").await?;

    page.driver
        .execute(
            "document.getElementById('test-rg-form').checkValidity()",
            Vec::new(),
        )
        .await?;
    page.wait_for_attr(&group, "data-invalid", Some("true"))
        .await?;
    let ids = attr(&group, "aria-describedby").await?.unwrap_or_default();
    assert_that!(Some(ids.clone())).is_not_equal_to(described.clone());
    let error = page
        .element(ids.split(' ').next_back().unwrap_or_default())
        .await?;
    assert_that!(error.text().await?.is_empty()).is_false();

    label(page, "Required A").await?.click().await?;
    page.wait_for_attr(&group, "data-invalid", None).await?;
    page.wait_for_attr(&group, "aria-describedby", described.as_deref())
        .await
}

/// A virtual click on a radio's label selects it.
async fn virtual_label_click(page: &Page<'_>) -> Result<(), Report> {
    let dragons = label(page, "Dragons").await?;
    virtual_click(page, &dragons).await?;
    page.wait_for_text("test-rg-value", "dragons").await?;
    // Back to Dogs, which the next steps expect selected.
    virtual_click(page, &label(page, "Dogs").await?).await?;
    page.wait_for_text("test-rg-value", "dogs").await
}

/// `element.click()` from script: a virtual click, as assistive technology sends.
async fn virtual_click(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    page.driver
        .execute("arguments[0].click()", vec![element.to_json()?])
        .await?;
    Ok(())
}
