//! Atom parts mounted again, and a second part mounted beside the first (`leptonic::Part`: one
//! element holds a part's props at a time). In each section `#rp-<name>`, `#rp-<name>-toggle`
//! unmounts and mounts the part (inside a `<Show>`, mounted at first), and `#rp-<name>-second`
//! mounts a second one (unmounted at first; then the section has `data-second="true"`):
//! - `description`, `field-error`, `label`, `input`: a text field's `Description`, `FieldError`
//!   (the field is invalid), `Label` and `Input`;
//! - `option-label`, `option-description`: a `ListBoxItemLabel` and a `ListBoxItemDescription`
//!   of the option "Apple";
//! - `section-heading`: the `ListBoxSectionHeading` of the section "Fruit";
//! - `dialog-title`: a `Dialog`'s `DialogTitle`;
//! - `disclosure-panel`: a `Disclosure`'s `DisclosurePanel` (expanded);
//! - `menu-item-label`: the `MenuItemLabel` of the menu item "Copy";
//! - `checkbox-button`: a `CheckboxField`'s `CheckboxButton`;
//! - `dialog-description`: an alert `Dialog`'s `DialogDescription`;
//! - `slider-output`, `slider-track`: a `Slider`'s `SliderOutput` and `SliderTrack` (with the
//!   thumb "Volume");
//! - `modal` (no toggles): `#rp-modal-open` opens a `ModalBackdrop` with two `ModalContent`s;
//! - `submenu` (no toggles): the menu "Files" with the submenu trigger "Share", whose popover
//!   holds two `Menu`s.

use leptonic::{
    atoms::{
        button::Button,
        checkbox::{CheckboxButton, CheckboxField},
        dialog::{Dialog, DialogDescription, DialogTitle},
        disclosure::{Disclosure, DisclosurePanel, DisclosureTrigger},
        field::{Description, FieldError, Label},
        input::Input,
        listbox::{
            ListBox, ListBoxItem, ListBoxItemDescription, ListBoxItemLabel, ListBoxSection,
            ListBoxSectionHeading,
        },
        menu::{Menu, MenuItem, MenuItemLabel, SubmenuTrigger},
        modal::{ModalBackdrop, ModalContent},
        popover::Popover,
        slider::{Slider, SliderOutput, SliderThumb, SliderTrack},
        text_field::TextField,
    },
    hooks::{collections::use_collection, dialog::DialogRole},
};
use leptos::prelude::*;

/// The section `#rp-<name>` with its controls: `shown` mounts the part, `second` a second one.
#[component]
fn Section(
    name: &'static str,
    shown: RwSignal<bool>,
    second: RwSignal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <div id=format!("rp-{name}") data-second=move || second.get().then_some("true")>
            <button id=format!("rp-{name}-toggle") on:click=move |_| shown.update(|s| *s = !*s)>
                {format!("Toggle {name}")}
            </button>
            <button id=format!("rp-{name}-second") on:click=move |_| second.set(true)>
                {format!("Second {name}")}
            </button>
            {children()}
        </div>
    }
}

/// The signals of a section: the part mounted, the second one not.
fn signals() -> (RwSignal<bool>, RwSignal<bool>) {
    (RwSignal::new(true), RwSignal::new(false))
}

#[component]
pub fn PageAtomRemountedParts() -> impl IntoView {
    let fruit = use_collection(|b| {
        b.item("apple", "Apple");
    });
    let sections = use_collection(|b| {
        b.section("fruit", |s| {
            s.header("fruit-header", "Fruit");
            s.item("pear", "Pear");
        });
    });
    let actions = use_collection(|b| {
        b.item("copy", "Copy");
    });
    let (description, description_second) = signals();
    let (error, error_second) = signals();
    let (label, label_second) = signals();
    let (input, input_second) = signals();
    let (option_label, option_label_second) = signals();
    let (option_description, option_description_second) = signals();
    let (heading, heading_second) = signals();
    let (title, title_second) = signals();
    let (panel, panel_second) = signals();
    let (item_label, item_label_second) = signals();
    let (checkbox, checkbox_second) = signals();
    let (alert_description, alert_description_second) = signals();
    let (output, output_second) = signals();
    let (track, track_second) = signals();
    let modal_open = RwSignal::new(false);
    let files = use_collection(|b| {
        b.item("open", "Open");
        b.item("share", "Share");
    });
    let share = use_collection(|b| {
        b.item("email", "Email");
    });

    view! {
        <h1>"Remounted parts"</h1>
        <Section name="description" shown=description second=description_second>
            <TextField>
                <Label>"Name"</Label>
                <Input />
                <Show when=move || description.get()>
                    <Description>"Your full name"</Description>
                </Show>
                <Show when=move || description_second.get()>
                    <Description>"Second description"</Description>
                </Show>
            </TextField>
        </Section>
        <Section name="field-error" shown=error second=error_second>
            <TextField is_invalid=true>
                <Label>"Email"</Label>
                <Input />
                <Show when=move || error.get()>
                    <FieldError>"Invalid email"</FieldError>
                </Show>
                <Show when=move || error_second.get()>
                    <FieldError>"Second error"</FieldError>
                </Show>
            </TextField>
        </Section>
        <Section name="label" shown=label second=label_second>
            <TextField aria_label="City">
                <Show when=move || label.get()>
                    <Label>"City"</Label>
                </Show>
                <Show when=move || label_second.get()>
                    <Label>"Second label"</Label>
                </Show>
                <Input />
            </TextField>
        </Section>
        <Section name="input" shown=input second=input_second>
            <TextField>
                <Label>"Street"</Label>
                <Show when=move || input.get()>
                    <Input />
                </Show>
                <Show when=move || input_second.get()>
                    <Input />
                </Show>
            </TextField>
        </Section>
        <Section name="option-label" shown=option_label second=option_label_second>
            <ListBox collection=fruit aria_label="Fruit">
                <ListBoxItem key="apple">
                    <Show when=move || option_label.get()>
                        <ListBoxItemLabel>"Apple"</ListBoxItemLabel>
                    </Show>
                    <Show when=move || option_label_second.get()>
                        <ListBoxItemLabel>"Second label"</ListBoxItemLabel>
                    </Show>
                </ListBoxItem>
            </ListBox>
        </Section>
        <Section
            name="option-description"
            shown=option_description
            second=option_description_second
        >
            <ListBox collection=fruit aria_label="Fruit">
                <ListBoxItem key="apple">
                    <ListBoxItemLabel>"Apple"</ListBoxItemLabel>
                    <Show when=move || option_description.get()>
                        <ListBoxItemDescription>"Crunchy"</ListBoxItemDescription>
                    </Show>
                    <Show when=move || option_description_second.get()>
                        <ListBoxItemDescription>"Second description"</ListBoxItemDescription>
                    </Show>
                </ListBoxItem>
            </ListBox>
        </Section>
        <Section name="section-heading" shown=heading second=heading_second>
            <ListBox collection=sections aria_label="Fruit sections">
                <ListBoxSection key="fruit">
                    <Show when=move || heading.get()>
                        <ListBoxSectionHeading />
                    </Show>
                    <Show when=move || heading_second.get()>
                        <ListBoxSectionHeading>"Second heading"</ListBoxSectionHeading>
                    </Show>
                    <ListBoxItem key="pear">"Pear"</ListBoxItem>
                </ListBoxSection>
            </ListBox>
        </Section>
        <Section name="dialog-title" shown=title second=title_second>
            <Dialog>
                <Show when=move || title.get()>
                    <DialogTitle>"Settings"</DialogTitle>
                </Show>
                <Show when=move || title_second.get()>
                    <DialogTitle>"Second title"</DialogTitle>
                </Show>
                <button>"Inside"</button>
            </Dialog>
        </Section>
        <Section name="disclosure-panel" shown=panel second=panel_second>
            <Disclosure default_expanded=true>
                <DisclosureTrigger>
                    <Button>"Details"</Button>
                </DisclosureTrigger>
                <Show when=move || panel.get()>
                    <DisclosurePanel>"Panel content"</DisclosurePanel>
                </Show>
                <Show when=move || panel_second.get()>
                    <DisclosurePanel>"Second panel"</DisclosurePanel>
                </Show>
            </Disclosure>
        </Section>
        <Section name="menu-item-label" shown=item_label second=item_label_second>
            <Menu collection=actions aria_label="Actions">
                <MenuItem key="copy">
                    <Show when=move || item_label.get()>
                        <MenuItemLabel>"Copy"</MenuItemLabel>
                    </Show>
                    <Show when=move || item_label_second.get()>
                        <MenuItemLabel>"Second label"</MenuItemLabel>
                    </Show>
                </MenuItem>
            </Menu>
        </Section>
        <Section name="checkbox-button" shown=checkbox second=checkbox_second>
            <CheckboxField>
                <Show when=move || checkbox.get()>
                    <CheckboxButton>"Subscribe"</CheckboxButton>
                </Show>
                <Show when=move || checkbox_second.get()>
                    <CheckboxButton>"Second button"</CheckboxButton>
                </Show>
            </CheckboxField>
        </Section>
        <Section name="dialog-description" shown=alert_description second=alert_description_second>
            <Dialog role=DialogRole::AlertDialog>
                <DialogTitle>"Delete?"</DialogTitle>
                <Show when=move || alert_description.get()>
                    <DialogDescription>"It is deleted permanently."</DialogDescription>
                </Show>
                <Show when=move || alert_description_second.get()>
                    <DialogDescription>"Second description"</DialogDescription>
                </Show>
                <button>"Delete"</button>
            </Dialog>
        </Section>
        <Section name="slider-output" shown=output second=output_second>
            <Slider min_value=0.0 max_value=100.0 default_values=vec![30.0_f64] aria_label="Volume">
                <Show when=move || output.get()>
                    <SliderOutput />
                </Show>
                <Show when=move || output_second.get()>
                    <SliderOutput />
                </Show>
                <SliderTrack>
                    <SliderThumb index=0 aria_label="Volume" />
                </SliderTrack>
            </Slider>
        </Section>
        <Section name="slider-track" shown=track second=track_second>
            <Slider
                min_value=0.0
                max_value=100.0
                default_values=vec![30.0_f64]
                aria_label="Balance"
            >
                <Show when=move || track.get()>
                    <SliderTrack>
                        <SliderThumb index=0 aria_label="Balance" />
                    </SliderTrack>
                </Show>
                <Show when=move || track_second.get()>
                    <SliderTrack>"Second track"</SliderTrack>
                </Show>
            </Slider>
        </Section>
        <div id="rp-modal">
            <button id="rp-modal-open" on:click=move |_| modal_open.set(true)>
                "Open the modal"
            </button>
            <ModalBackdrop is_open=modal_open set_open=modal_open is_dismissable=true>
                <ModalContent>
                    <Dialog aria_label="First">
                        <button>"Inside"</button>
                    </Dialog>
                </ModalContent>
                <ModalContent>
                    <p>"Second content"</p>
                </ModalContent>
            </ModalBackdrop>
        </div>
        <div id="rp-submenu">
            <Menu collection=files aria_label="Files">
                <MenuItem key="open">"Open"</MenuItem>
                <SubmenuTrigger key="share">
                    <MenuItem key="share">"Share"</MenuItem>
                    <Popover>
                        <Menu collection=share>
                            <MenuItem key="email">"Email"</MenuItem>
                        </Menu>
                        <Menu collection=share>
                            <MenuItem key="email">"Second email"</MenuItem>
                        </Menu>
                    </Popover>
                </SubmenuTrigger>
            </Menu>
        </div>
    }
}
