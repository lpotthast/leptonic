//! `ListBoxSection`s in a `ListBox` (a multiple selection), for their performance test
//! (`leptonic/tests/perf_tests/atoms/list_box_section.rs`): sections "Section 0", "Section 1", ...
//! (`data-perf-section` holding the number, each with its `ListBoxSectionHeading`) of options
//! "Option 0", "Option 1", ... (numbered through, `data-perf-option`): [`SMALL`] at first,
//! [`LARGE`] after the control `large`, in a 200px high scroll view, between two plain buttons
//! (`#perf-before`, `#perf-after`). The listbox stays mounted; the controls ([`Control`]) mount and
//! unmount the sections and change the collection (an option appended to the first section and
//! removed, the first section renamed).

use leptonic::{
    atoms::listbox::{ListBox, ListBoxItem, ListBoxSection, ListBoxSectionHeading},
    hooks::collections::{Key, SelectionMode, use_collection},
    leptos_styles::Styles,
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
    // Options appended to the first section.
    let appended = RwSignal::new(0_u32);
    let renamed = RwSignal::new(false);
    let collection = use_collection(move |b| {
        let (sections, per_section) = size.get();
        let (appended, renamed) = (appended.get(), renamed.get());
        for section in 0..sections {
            b.section(format!("s{section}"), |b| {
                let name = if renamed && section == 0 {
                    "Renamed section 0".to_owned()
                } else {
                    format!("Section {section}")
                };
                b.header(format!("h{section}"), name);
                for i in options(section, per_section, appended) {
                    b.item(i, format!("Option {i}"));
                }
            });
        }
    });
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || size.set(LARGE)),
        Control::action("append", move || appended.update(|appended| *appended += 1)),
        Control::action("remove", move || appended.update(|appended| *appended -= 1)),
        Control::action("rename", move || renamed.set(true)),
        Control::action("restore-name", move || renamed.set(false)),
    ];
    view! {
        <h1>"ListBoxSection performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <ListBox
            collection=collection
            selection_mode=SelectionMode::Multiple
            aria_label="Options"
            styles=Styles::new()
                .add_unchecked("max-height", "200px")
                .add_unchecked("overflow", "auto")
        >
            <Show when=move || mounted.get()>
                <For each=move || 0..size.get().0 key=|section| *section let:section>
                    <ListBoxSection
                        key=Key::from(format!("s{section}"))
                        attr:data-perf-section=section
                    >
                        <ListBoxSectionHeading />
                        <For
                            each=move || {
                                let (_, per_section) = size.get();
                                options(section, per_section, appended.get())
                            }
                            key=|i| *i
                            let:i
                        >
                            <ListBoxItem key=i attr:data-perf-option=i>
                                {format!("Option {i}")}
                            </ListBoxItem>
                        </For>
                    </ListBoxSection>
                </For>
            </Show>
        </ListBox>
        <button id="perf-after">"After"</button>
    }
}
