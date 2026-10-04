//! Test fixtures driven by the browser tests in `leptonic/tests/`.
//!
//! Every fixture is reachable at `/{group}/{name}`, where `group` is `atoms`, `hooks` or
//! `components`. To add one, create a module with a page component and register it in
//! [`FIXTURES`].

pub mod atoms;
pub mod hooks;

use leptos::{prelude::*, web_sys};

/// A test page, reachable at `/{group}/{name}`.
pub struct Fixture {
    pub group: &'static str,
    pub name: &'static str,
    pub title: &'static str,
    pub view: fn() -> AnyView,
}

impl Fixture {
    pub fn path(&self) -> String {
        format!("/{}/{}", self.group, self.name)
    }
}

pub const FIXTURES: &[Fixture] = &[
    Fixture {
        group: "atoms",
        name: "button",
        title: "Button",
        view: || view! { <atoms::button::PageAtomButton /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "focus-scope",
        title: "Focus Scope",
        view: || view! { <atoms::focus_scope::PageAtomFocusScope /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "focus",
        title: "Focus",
        view: || view! { <hooks::focus::PageHookFocus /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "focus-within",
        title: "Focus Within",
        view: || view! { <hooks::focus_within::PageHookFocusWithin /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "focus-ring",
        title: "Focus Ring",
        view: || view! { <hooks::focus_ring::PageHookFocusRing /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "focusable",
        title: "Focusable",
        view: || view! { <hooks::focusable::PageHookFocusable /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "focus-manager",
        title: "Focus Manager",
        view: || view! { <hooks::focus_manager::PageHookFocusManager /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "focus-visible",
        title: "Focus Visible",
        view: || view! { <hooks::focus_visible::PageHookFocusVisible /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "has-tabbable-child",
        title: "Has Tabbable Child",
        view: || view! { <hooks::has_tabbable_child::PageHookHasTabbableChild /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "press",
        title: "Press",
        view: || view! { <hooks::press::PageHookPress /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "button",
        title: "Button",
        view: || view! { <hooks::button::PageHookButton /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "menu-trigger",
        title: "Menu Trigger",
        view: || view! { <hooks::menu_trigger::PageHookMenuTrigger /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "number-field",
        title: "Number Field",
        view: || view! { <hooks::number_field::PageHookNumberField /> }.into_any(),
    },
    Fixture {
        group: "hooks",
        name: "live-announcer",
        title: "Live Announcer",
        view: || view! { <hooks::live_announcer::PageLiveAnnouncer /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "listbox",
        title: "ListBox",
        view: || view! { <atoms::listbox::PageAtomListBox /> }.into_any(),
    },
    Fixture {
        group: "atoms",
        name: "select",
        title: "Select",
        view: || view! { <atoms::select::PageAtomSelect /> }.into_any(),
    },
];

pub fn find_fixture(group: &str, name: &str) -> Option<&'static Fixture> {
    FIXTURES
        .iter()
        .find(|fixture| fixture.group == group && fixture.name == name)
}

/// Prevent mousedown from stealing focus (used on control buttons in focus-manager tests).
pub fn prevent_focus_steal(e: web_sys::MouseEvent) {
    e.prevent_default();
}
