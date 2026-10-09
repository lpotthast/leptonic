// Upstream: react-aria/src/calendar/useCalendarBase.ts @ 99e6102368
// Upstream: react-aria/src/calendar/useCalendar.ts @ 99e6102368
// Upstream: react-aria/src/calendar/useRangeCalendar.ts @ 99e6102368
// Upstream: react-aria/test/calendar/useCalendar.test.js @ 99e6102368
// Upstream: react-aria-components/test/Calendar.test.js @ 99e6102368
// Upstream: react-aria-components/test/RangeCalendar.test.tsx @ 99e6102368
// Upstream: @adobe/react-spectrum/test/calendar/CalendarBase.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/calendar/Calendar.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/calendar/RangeCalendar.test.js @ 99e6102368
use leptos::{
    attr::{self, Attr},
    ev::{self},
    prelude::*,
};
use wasm_bindgen::JsCast;
use web_sys::{FocusEvent, HtmlButtonElement};

use super::{
    states::{
        CalendarData, CalendarFormatters, CalendarStates, selected_date_description,
        visible_range_description,
    },
    use_calendar_state::CalendarState,
    use_range_calendar_state::RangeCalendarState,
};
use crate::{
    CapturedElement, EventHandler, IntoAttrs, OnEvent,
    hooks::button::UseButtonInput,
    labels,
    utils::{
        aria::AriaRole,
        dom_ext::EventAccessors,
        i18n::use_locale,
        id::use_id,
        intl_strings::{CalendarStrings, use_localized_strings},
        live_announcer::{Assertiveness, announce, announce_with_timeout},
        slot_id::{SlotProps, use_slot},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The previous and next buttons are configured, not rendered: `previous_button` and
//   `next_button` are `UseButtonInput`s for `use_button`.
// - The calendar's data for its grids and cells is returned (`data`; react-aria: a `WeakMap`
//   keyed by the state).
// - `commit_behavior` is an enum (react-aria: a string).
// - One input with the state (C8): `UseCalendarInput`, `UseRangeCalendarInput` (react-aria:
//   props, state and ref as arguments).
//
// ## OMITTED FEATURES
// - The title of several months is "May 2024 to July 2024" (react-aria: the locale's date range
//   format, "May – July 2024"): ICU4X has no range formatting yet.
//
// =============================================================================

/// Input of [`use_calendar`].
#[derive(Clone)]
pub struct UseCalendarInput {
    pub state: CalendarState,
    /// The calendar's id. Generated when `None`.
    pub id: Option<String>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub aria_details: Option<String>,
}

/// Input of [`use_range_calendar`].
#[derive(Clone)]
pub struct UseRangeCalendarInput {
    pub state: RangeCalendarState,
    /// What a press outside the dates or leaving the calendar does with a range being selected.
    pub commit_behavior: CommitBehavior,
    /// The calendar's id. Generated when `None`.
    pub id: Option<String>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub aria_details: Option<String>,
}

/// What `use_calendar_base` takes from either input.
struct CalendarAria {
    id: Option<String>,
    aria_label: MaybeProp<String>,
    aria_labelledby: Option<String>,
    aria_describedby: Option<String>,
    aria_details: Option<String>,
}

/// What a range calendar does with a range being selected when the user presses outside its
/// dates or leaves it (react-aria's `commitBehavior`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CommitBehavior {
    /// Clear the value too.
    Clear,
    /// Drop the range being selected, keeping the value.
    Reset,
    /// Finish the range at the focused date.
    #[default]
    Select,
}

/// Return value of [`use_calendar`] and [`use_range_calendar`].
pub struct UseCalendarReturn {
    /// For the calendar's element (a group of its heading, buttons and grids).
    pub calendar_props: UseCalendarProps,
    /// For the previous page button (`use_button`).
    pub previous_button: UseButtonInput,
    /// For the next page button (`use_button`).
    pub next_button: UseButtonInput,
    /// For the error message element, if there is one.
    pub error_message_props: SlotProps,
    /// The visible range, for a heading: "May 2024".
    pub title: Signal<String>,
    /// What the calendar's grids and cells need (`use_calendar_grid`, `use_calendar_cell`).
    pub data: CalendarData,
}

/// Props of the calendar's element.
#[derive(Debug, Clone)]
pub struct UseCalendarProps {
    pub id: String,
    pub role: AriaRole,
    pub aria_label: Signal<Option<String>>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub aria_details: Option<String>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub element_capture: crate::ElementCaptureAttr,
}

pub type UseCalendarAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabel, Signal<Option<String>>>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaDescribedby, Option<String>>,
    Attr<attr::AriaDetails, Option<String>>,
    OnEvent<ev::focusout>,
    crate::ElementCaptureAttr,
);

impl IntoAttrs for UseCalendarProps {
    type Attrs = UseCalendarAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::AriaDetails, self.aria_details),
            self.on_focusout.into_on(ev::focusout),
            self.element_capture,
        )
    }
}

/// Behavior and accessibility of a calendar (react-aria's `useCalendarBase`).
fn use_calendar_base(
    input: CalendarAria,
    state: &CalendarStates,
    element: CapturedElement,
) -> UseCalendarReturn {
    let CalendarAria {
        id,
        aria_label,
        aria_labelledby,
        aria_describedby,
        aria_details,
    } = input;
    let state = *state;
    let calendar = state.calendar();
    let locale = use_locale();
    let strings = use_localized_strings::<CalendarStrings>();
    let id = id.unwrap_or_else(|| use_id("calendar"));

    let title =
        Memo::new(move |_| visible_range_description(calendar.visible_range.get(), &locale.get()));
    // The visible range is announced when it changes by the previous or next button (not while
    // the grid has focus), the selection when it changes.
    Effect::new(move |previous: Option<String>| {
        let description = title.get();
        if previous.is_some_and(|previous| previous != description)
            && !calendar.is_focused.get_untracked()
        {
            announce(description.clone(), Assertiveness::Polite);
        }
        description
    });
    let selected_date_description =
        Memo::new(move |_| selected_date_description(&state, &locale.get()));
    Effect::new(move |previous: Option<String>| {
        let description = selected_date_description.get();
        if previous.is_some_and(|previous| previous != description) && !description.is_empty() {
            announce_with_timeout(
                description.clone(),
                Assertiveness::Polite,
                std::time::Duration::from_millis(4000),
            );
        }
        description
    });

    let error = use_slot("calendar-error");
    let data = CalendarData {
        state,
        aria_label,
        aria_labelledby: aria_labelledby.clone(),
        error_message_id: error.referenced_id,
        selected_date_description: selected_date_description.into(),
        formatters: CalendarFormatters::new(),
    };

    // Buttons disabled while focused hand the focus to the calendar's grid.
    let next_focused = RwSignal::new(false);
    let previous_focused = RwSignal::new(false);
    let next_disabled = Signal::derive(move || {
        calendar.is_disabled.get() || calendar.is_next_visible_range_invalid()
    });
    let previous_disabled = Signal::derive(move || {
        calendar.is_disabled.get() || calendar.is_previous_visible_range_invalid()
    });
    Effect::new(move |_| {
        if next_disabled.get() && next_focused.get_untracked() {
            next_focused.set(false);
            calendar.set_focused(true);
        }
        if previous_disabled.get() && previous_focused.get_untracked() {
            previous_focused.set(false);
            calendar.set_focused(true);
        }
    });

    let label = Signal::derive(move || {
        let label = [aria_label.get(), Some(title.get())]
            .into_iter()
            .flatten()
            .filter(|label| !label.is_empty())
            .collect::<Vec<_>>()
            .join(", ");
        Some(label)
    });
    let aria_labelledby =
        labels(&id, Some(String::new()), aria_labelledby.as_deref()).aria_labelledby;

    UseCalendarReturn {
        calendar_props: UseCalendarProps {
            id,
            role: AriaRole::Application,
            aria_label: label,
            aria_labelledby,
            aria_describedby,
            aria_details,
            on_focusout: EventHandler::empty(),
            element_capture: element.attr(),
        },
        previous_button: UseButtonInput {
            on_press: Some(Callback::new(move |_| calendar.focus_previous_page())),
            aria_label: Signal::derive(move || Some(strings.read().previous())).into(),
            is_disabled: previous_disabled,
            on_focus_change: Some(Callback::new(move |focused| {
                previous_focused.set(focused);
                if !focused
                    && previous_disabled.get_untracked()
                    && !calendar.is_disabled.get_untracked()
                {
                    calendar.set_focused(true);
                }
            })),
            ..UseButtonInput::default()
        },
        next_button: UseButtonInput {
            on_press: Some(Callback::new(move |_| calendar.focus_next_page())),
            aria_label: Signal::derive(move || Some(strings.read().next())).into(),
            is_disabled: next_disabled,
            on_focus_change: Some(Callback::new(move |focused| {
                next_focused.set(focused);
                if !focused
                    && next_disabled.get_untracked()
                    && !calendar.is_disabled.get_untracked()
                {
                    calendar.set_focused(true);
                }
            })),
            ..UseButtonInput::default()
        },
        error_message_props: error.props,
        title: title.into(),
        data,
    }
}

/// Behavior and accessibility of a calendar: its grouping element, the previous and next
/// buttons, the title, announcements of the visible range and the selection.
pub fn use_calendar(input: UseCalendarInput) -> UseCalendarReturn {
    let UseCalendarInput {
        state,
        id,
        aria_label,
        aria_labelledby,
        aria_describedby,
        aria_details,
    } = input;
    use_calendar_base(
        CalendarAria {
            id,
            aria_label,
            aria_labelledby,
            aria_describedby,
            aria_details,
        },
        &state.into(),
        CapturedElement::new(),
    )
}

/// Behavior and accessibility of a range calendar: [`use_calendar`]'s, and finishing (per
/// `commit_behavior`) a range being selected when a pointer is released outside its dates or
/// the focus leaves the calendar.
pub fn use_range_calendar(input: UseRangeCalendarInput) -> UseCalendarReturn {
    let UseRangeCalendarInput {
        state,
        commit_behavior,
        id,
        aria_label,
        aria_labelledby,
        aria_describedby,
        aria_details,
    } = input;
    let element = CapturedElement::new();
    let mut calendar = use_calendar_base(
        CalendarAria {
            id,
            aria_label,
            aria_labelledby,
            aria_describedby,
            aria_details,
        },
        &state.into(),
        element,
    );
    let commit = move || match commit_behavior {
        CommitBehavior::Clear => state.clear_selection(),
        CommitBehavior::Reset => state.set_anchor_date(None),
        CommitBehavior::Select => state.commit_selection(),
    };

    #[cfg(not(feature = "ssr"))]
    {
        use leptos_use::{
            UseEventListenerOptions, use_event_listener, use_event_listener_with_options,
            use_window,
        };

        use crate::utils::shadow_dom::{get_active_element, node_contains};

        let is_focus_within = |element: &web_sys::Element| {
            element
                .owner_document()
                .as_ref()
                .and_then(get_active_element)
                .is_some_and(|active| node_contains(element.as_ref(), active.as_ref()))
        };

        // VoiceOver's virtual clicks fire pointer events before the click (react-aria ignores
        // them, as `use_press` does).
        let is_virtual_click = StoredValue::new(false);
        let _ = use_event_listener(
            use_window(),
            ev::pointerdown,
            move |e: web_sys::PointerEvent| {
                is_virtual_click.set_value(e.width() == 0 && e.height() == 0);
            },
        );
        // A pointer released outside the dates (not on a button of the calendar) finishes the
        // range being selected.
        let _ = use_event_listener(
            use_window(),
            ev::pointerup,
            move |e: web_sys::PointerEvent| {
                if is_virtual_click.get_value() {
                    is_virtual_click.set_value(false);
                    return;
                }
                state.set_dragging(false);
                if state.anchor_date.get_untracked().is_none() {
                    return;
                }
                let Some(calendar_element) = element.get_untracked() else {
                    return;
                };
                let target = crate::utils::shadow_dom::get_event_target(&e)
                    .and_then(|target| target.dyn_into::<web_sys::Element>().ok());
                let outside = target.as_ref().is_none_or(|target| {
                    !node_contains(calendar_element.as_ref(), target.as_ref())
                });
                let on_button = target
                    .as_ref()
                    .and_then(|target| target.closest("button, [role=\"button\"]").ok().flatten())
                    .is_some();
                if is_focus_within(&calendar_element) && (outside || !on_button) {
                    commit();
                }
            },
        );
        // No touch scrolling while dragging a range.
        Effect::new(move |_| {
            let Some(calendar_element) = element.get() else {
                return;
            };
            let _ = use_event_listener_with_options(
                (*calendar_element).clone(),
                ev::touchmove,
                move |e: web_sys::TouchEvent| {
                    if state.is_dragging.get_untracked() {
                        e.prevent_default();
                    }
                },
                UseEventListenerOptions::default()
                    .passive(false)
                    .capture(true),
            );
        });
    }

    // Leaving the calendar finishes the range being selected too.
    calendar.calendar_props.on_focusout = EventHandler::new(move |e: FocusEvent| {
        // A removed focused cell blurs after the calendar's disposal ("Blur After Disposal"):
        // the anchor, of the same owner, tells.
        let Some(anchor) = state.anchor_date.try_get_untracked() else {
            return;
        };
        let Some(calendar_element) = element.get_untracked() else {
            return;
        };
        let related = e
            .related_target()
            .and_then(|target| wasm_bindgen::JsCast::dyn_into::<web_sys::Node>(target).ok());
        // Chrome blurs a navigation button synchronously when paging disables it. Focus is
        // being handed to the grid, so this is not a departure that commits the range.
        if related.is_none()
            && e.expect_target()
                .dyn_ref::<HtmlButtonElement>()
                .is_some_and(HtmlButtonElement::disabled)
            && !state.calendar.is_disabled.get_untracked()
        {
            state.calendar.set_focused(true);
            return;
        }
        let leaves = related.is_none_or(|related| {
            !crate::utils::shadow_dom::node_contains(calendar_element.as_ref(), &related)
        });
        if leaves && anchor.is_some() {
            commit();
        }
    });
    calendar
}
