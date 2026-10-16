//! `use_listbox_section` on plain elements in a `use_listbox` listbox (a multiple selection), for
//! its performance test (`leptonic/tests/perf_tests/hooks/use_listbox_section.rs`): sections (a
//! `<div>` with the group props, `data-perf-section` holding the number, holding a `<span>` heading
//! "Section 0", "Section 1", ... and the section's options) of options "Option 0", "Option 1", ...
//! (numbered through, `use_option`, `data-perf-option`): [`SMALL`] at first, [`LARGE`] after the
//! control `large`, in a 200px high scroll view, between two plain buttons (`#perf-before`,
//! `#perf-after`). The listbox stays mounted; the controls ([`Control`]) mount and unmount the
//! sections and change the collection (an option appended to the first section and removed).

use leptonic::{
    CapturedElement, IntoAttrs, Orientation,
    hooks::{
        collections::{
            CollectionOptions, Key, ListLayout, SelectionMode, SelectionOptions, UseListStateInput,
            use_collection, use_list_state,
        },
        listbox::{
            ListBoxData, UseListBoxInput, UseListBoxReturn, UseListBoxSectionInput,
            UseListBoxSectionReturn, UseOptionInput, UseOptionReturn, use_listbox,
            use_listbox_section, use_option,
        },
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// Sections and options per section at first: 10 options.
pub const SMALL: (u32, u32) = (2, 5);
/// Sections and options per section after the control `large`: 1,000 options.
pub const LARGE: (u32, u32) = (100, 10);
/// The number of the first option appended to the first section (the control `append`).
pub const APPENDED: u32 = 10_000;

/// The options of section `section`, `per_section` per section, and `appended` options more in
/// the first one.
fn options(section: u32, per_section: u32, appended: u32) -> Vec<u32> {
    let start = section * per_section;
    let mut options: Vec<u32> = (start..start + per_section).collect();
    if section == 0 {
        options.extend(APPENDED..APPENDED + appended);
    }
    options
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let size = RwSignal::new(SMALL);
    let appended = RwSignal::new(0_u32);
    let collection = use_collection(move |b| {
        let (sections, per_section) = size.get();
        let appended = appended.get();
        for section in 0..sections {
            b.section(format!("s{section}"), |b| {
                b.header(format!("h{section}"), format!("Section {section}"));
                for i in options(section, per_section, appended) {
                    b.item(i, format!("Option {i}"));
                }
            });
        }
    });
    let state = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            ..SelectionOptions::default()
        },
    });
    let UseListBoxReturn { props, data } = use_listbox(UseListBoxInput {
        state,
        element: CapturedElement::new(),
        id: None,
        aria_label: "Options".into(),
        aria_labelledby: Signal::stored(None),
        orientation: Orientation::Vertical.into(),
        layout: ListLayout::Stack,
        keyboard_delegate: None,
        layout_delegate: None,
        is_virtualized: false,
        options: CollectionOptions::default(),
        should_select_on_press_up: false,
        should_focus_on_hover: false,
        on_action: None,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });
    let data = StoredValue::new(data);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || size.set(LARGE)),
        Control::action("append", move || appended.update(|appended| *appended += 1)),
        Control::action("remove", move || appended.update(|appended| *appended -= 1)),
    ];
    view! {
        <h1>"use_listbox_section performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <div
            {..props.into_attrs()}
            style="max-height: 200px; overflow: auto"
        >
            <Show when=move || mounted.get()>
                <For each=move || 0..size.get().0 key=|section| *section let:section>
                    <PlainSection
                        list=data.get_value()
                        section
                        options=Signal::derive(move || {
                            options(section, size.get().1, appended.get())
                        })
                    />
                </For>
            </Show>
        </div>
        <button id="perf-after">"After"</button>
    }
}

/// A section (`use_listbox_section`) with its heading and options.
#[component]
fn PlainSection(list: ListBoxData, section: u32, options: Signal<Vec<u32>>) -> impl IntoView {
    let UseListBoxSectionReturn {
        heading_props,
        group_props,
        heading,
        ..
    } = use_listbox_section(UseListBoxSectionInput {
        list: list.clone(),
        key: Key::from(format!("s{section}")),
    });
    let list = StoredValue::new(list);
    view! {
        <div {..group_props.into_attrs()} data-perf-section=section>
            {heading_props
                .map(|props| {
                    view! { <span {..props.into_attrs()}>{move || heading.get()}</span> }
                })}
            <For each=move || options.get() key=|i| *i let:i>
                <PlainOption list=list.get_value() i />
            </For>
        </div>
    }
}

/// An option (`use_option`).
#[component]
fn PlainOption(list: ListBoxData, i: u32) -> impl IntoView {
    let UseOptionReturn { props, .. } = use_option(UseOptionInput {
        list,
        key: Key::from(i),
        on_context_menu: None,
    });
    let (attrs, styles) = props.into_parts();
    view! {
        <div {..attrs} style=styles data-perf-option=i>
            {format!("Option {i}")}
        </div>
    }
}
