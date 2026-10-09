use leptonic::{
    I18nProvider, Locale, Orientation,
    atoms::{
        button::Button,
        checkbox::{CheckboxButton, CheckboxField},
        link::Link,
        toggle_button::ToggleButton,
        toolbar::Toolbar,
    },
};
use leptos::prelude::*;

/// Toolbar atoms (react-aria-components' `Toolbar.test.tsx` "supports keyboard navigation"
/// setup): "Before", a "Tools" toolbar with an "Align text" and a "Zoom" toolbar (groups of it)
/// separated by an `<hr>`, and "After". Further: a vertical toolbar ("Up 1", "Up 2") and a
/// horizontal one in a right-to-left locale ("RTL 1", "RTL 2"), a vertical one in a right-to-left
/// locale ("RV 1", "RV 2"), and react-aria's example toolbar ("Input Before Toolbar", toggle
/// buttons "B", "U", "I", a "Night Mode" checkbox and a "Help" link), and a toolbar with both an
/// `aria_label` ("Labelled twice") and an `aria_labelledby` (`#test-toolbar-label`).
#[component]
pub fn PageAtomToolbar() -> impl IntoView {
    let rtl: Locale = "ar".parse().expect("a valid locale");
    let rtl_vertical: Locale = "he-IL".parse().expect("a valid locale");
    view! {
        <div id="test-page-atom-toolbar">
            <Button>"Before"</Button>
            <Toolbar aria_label="Tools">
                <Toolbar aria_label="Align text">
                    <Button>"Align left"</Button>
                    <Button>"Align center"</Button>
                    <Button>"Align right"</Button>
                </Toolbar>
                <hr />
                <Toolbar aria_label="Zoom">
                    <Button>"Zoom in"</Button>
                    <Button>"Zoom out"</Button>
                </Toolbar>
            </Toolbar>
            <Button>"After"</Button>

            <Toolbar aria_label="Vertical" orientation=Orientation::Vertical>
                <Button>"Up 1"</Button>
                <Button>"Up 2"</Button>
            </Toolbar>

            <I18nProvider locale=rtl>
                <Toolbar aria_label="Right to left">
                    <Button>"RTL 1"</Button>
                    <Button>"RTL 2"</Button>
                </Toolbar>
            </I18nProvider>

            <I18nProvider locale=rtl_vertical>
                <Toolbar aria_label="Right to left vertical" orientation=Orientation::Vertical>
                    <Button>"RV 1"</Button>
                    <Button>"RV 2"</Button>
                </Toolbar>
            </I18nProvider>

            <span id="test-toolbar-label">"Toolbar aria-labelledby"</span>
            <Toolbar aria_label="Labelled twice" aria_labelledby="test-toolbar-label">
                <Button>"Align right"</Button>
            </Toolbar>

            <input aria-label="Input Before Toolbar" id="test-toolbar-input-before" />
            <Toolbar aria_label="Text formatting">
                <ToggleButton>"B"</ToggleButton>
                <ToggleButton>"U"</ToggleButton>
                <ToggleButton>"I"</ToggleButton>
                <CheckboxField><CheckboxButton>"Night Mode"</CheckboxButton></CheckboxField>
                <Link href="/atoms/toolbar#help">"Help"</Link>
            </Toolbar>
        </div>
    }
}
