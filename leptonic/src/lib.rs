#![recursion_limit = "256"]
// Workspace-level clippy allows. These are specified in workspace Cargo.toml lints but
// must also be set here because CLI `-D clippy::pedantic` takes precedence over Cargo.toml lints.
#![allow(
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::wildcard_imports,
    // `ignored_unit_patterns` fires on Leptos `view!` macro expansions and `#[component]` artifacts.
    clippy::ignored_unit_patterns,
    // `type_complexity` fires on `#[component]` macro-generated prop types that cannot be annotated individually.
    clippy::type_complexity
)]

use leptos::prelude::*;
use leptos_use::use_window;

#[cfg(feature = "atoms")]
pub mod atoms;
mod attrs;
pub mod hooks;
#[cfg(test)]
pub(crate) mod testing;
pub(crate) mod utils;

// Leptonic has no prelude: atoms and hooks are imported from their modules
// (`leptonic::atoms::button::Button`, `leptonic::hooks::interactions::use_press`), everything else
// users need from the crate root below, the only path to each of these items (their modules are
// private; documentation/conventions.md, "No prelude; one path per public item").

/// The date crate of leptonic's date APIs (calendars, date fields).
pub use jiff;
/// Classes for the `classes` props of atoms (`Classes`).
pub use leptos_classes;
pub use leptos_element_capture::{CapturedElement, ElementCaptureAttr, ElementCaptureCallback};
/// Typed inline styles for the `styles` props of atoms (`Styles`, typed declarations, CSS values).
pub use leptos_styles;

#[cfg(feature = "clipboard")]
pub use crate::utils::clipboard::{ClipboardError, write_text, write_text_deferred};
#[cfg(feature = "syntax-highlight")]
pub use crate::utils::syntax_highlight::highlight_to_classed_html;
pub use crate::{
    attrs::{IntoAttrs, PropsWithStyles},
    utils::{
        aria::{
            AriaAtomic, AriaAutocomplete, AriaChecked, AriaCurrent, AriaDisabled, AriaExpanded,
            AriaHasPopup, AriaHidden, AriaInvalid, AriaKeyshortcuts, AriaLive, AriaModal,
            AriaMultiselectable, AriaOrientation, AriaPressed, AriaReadonly, AriaRelevant,
            AriaRequired, AriaRole, AriaSelected, AriaSort,
        },
        aria_hide_outside::{AriaHideOutsideOptions, HideMode, aria_hide_outside, keep_visible},
        color::{
            Alpha, AlphaChannel, AreaGradient, BlendMode, Color, ColorChannel, ColorChannelRange,
            ColorFormat, ColorProp, ColorSpaceAxes, ColorValue, HSL, HSV, HslChannel, HsvChannel,
            OpaqueColor, ParseColorError, RGB8, RgbChannel,
        },
        data_attributes::flag,
        date::{
            DateDuration, DateExt, DateRange, first_day_of_week, max_date, min_date, today,
            use_today,
        },
        date_time_formatter::{
            DateTimeFormat, DateTimeFormatOptions, DateTimeFormatter, DateTimeStyle, MonthFormat,
            NumericFormat, TimeZoneFormat,
        },
        dom_ext::ContainsTarget,
        event_handler::{EventHandler, OnEvent},
        filter::{
            Collator, CollatorOptions, CollatorSensitivity, Filter, FilterQuery, use_collator,
            use_filter,
        },
        focus::{focus_element, focus_safely},
        focusability::{
            FOCUSABLE_SELECTOR, TABBABLE_SELECTOR, is_focusable, is_tabbable, is_typing_target,
            prevent_focus_attr, will_open_keyboard,
        },
        fraction::Fraction,
        heading_level::HeadingLevel,
        i18n::{
            I18nContext, I18nProvider, InvalidLocale, Locale, WritingDirection, locale,
            use_direction, use_i18n, use_locale,
        },
        id::use_id,
        id_refs::IdRefs,
        intl_strings::{
            AtomStrings, BreadcrumbsStrings, Bundle, CalendarStrings, ColorInputLabelArgs,
            ColorNameAndValueArgs, ColorNameArgs, ColorNameStrings, ColorStrings, ComboBoxStrings,
            DatePickerStrings, DateRangeArgs, DateValidationStrings, DndStrings,
            FocusAnnouncementArgs, GridStrings, InsertBetweenArgs, LocalizedStrings, MenuStrings,
            NumberFieldStrings, OverlayStrings, SearchFieldStrings, SelectedRangeDescriptionArgs,
            SpinButtonStrings, Strings, TableStrings, TagStrings, ToastStrings,
            TransparentColorNameArgs, TreeStrings, use_localized_strings,
        },
        key::KeyboardKey,
        keyboard_shortcut::{InvalidShortcut, KeyboardShortcuts, Shortcut, ShortcutOutcome},
        labels::{Labels, labels},
        list_formatter::{ListFormatOptions, ListFormatStyle, ListFormatType, ListFormatter},
        live_announcer::{
            Announcement, Assertiveness, announce, announce_assertive, announce_polite,
            announce_with_timeout, clear_announcer, destroy_announcer,
        },
        modifiers::{EventModifiers, Modifiers},
        number_formatter::{
            CurrencyDisplay, CurrencySign, NumberFormatOptions, NumberFormatter, NumberPart,
            NumberPartKind, NumberStyle, NumberingSystem, SignDisplay, UnitDisplay,
            use_number_formatter,
        },
        number_parser::NumberParser,
        number_value::{NumberSignal, NumberValue, OptionalNumberSignal},
        orientation::Orientation,
        platform::{
            browser::{is_chrome, is_firefox, is_safari, is_webkit},
            device::{
                has_touch_events, is_android, is_apple_device, is_ios, is_ipad, is_iphone, is_mac,
            },
            use_platform_check,
        },
        plurals::plural_category,
        point::Point,
        pointer_type::PointerType,
        propagation_control::{Propagation, PropagationControl},
        scroll::{
            ScrollAlignment, ScrollIntoViewOpts, ScrollIntoViewportOpts, get_scroll_parent,
            get_scroll_parents, is_scrollable, scroll_into_view, scroll_into_viewport,
        },
        scroll_behavior::ScrollBehavior,
        slot_id::{Slot, SlotAttrs, SlotProps, use_slot},
        styles::css::{computed_pct, computed_px, computed_size},
        use_description::use_description,
        use_viewport_size::{ViewportSize, use_viewport_size},
        value_binding::ValueBinding,
        virtual_focus::move_virtual_focus,
        visually_hidden::{
            VISUALLY_HIDDEN_STYLE, visually_hidden_fixed_styles, visually_hidden_full_size_styles,
            visually_hidden_styles,
        },
    },
};

/// The `Out` type represents any outgoing / emittable value. Use it in components that should
/// return (propagate) a value upwards using a function-like property.
/// Out can be anything that can be written to:
/// - a `WriteSignal`,
/// - a combined `RwSignal` or
/// - a `Callback` which only consumes an input and returns `()`.
///
/// This helps you to define props where the user can choose to use a signal directly
/// or use a closure (which will be converted to a `Callback`).
#[derive(Debug)]
pub enum Out<O: 'static, S = SyncStorage> {
    Callback(Callback<O, ()>),
    WriteSignal(WriteSignal<O, S>),
    RwSignal(RwSignal<O, S>),
    StoredValue(StoredValue<O, S>),
}

impl<O: 'static, S> Copy for Out<O, S> {}

impl<O: 'static, S> Clone for Out<O, S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<O: 'static, S> Out<O, S> {
    /// Creates a new `Out` from the given function.
    pub fn new_callback(f: impl Fn(O) + Send + Sync + 'static) -> Self {
        Self::Callback(Callback::new(f))
    }
}

impl<O: 'static> Out<O, LocalStorage> {
    pub fn set(&self, new_value: O) {
        match self {
            Self::Callback(callback) => Callable::run(callback, new_value),
            Self::WriteSignal(write_signal) => write_signal.set(new_value),
            Self::RwSignal(rw_signal) => rw_signal.set(new_value),
            Self::StoredValue(stored_value) => stored_value.set_value(new_value),
        }
    }
}

impl<O: Send + Sync + 'static> Out<O, SyncStorage> {
    pub fn set(&self, new_value: O) {
        match self {
            Self::Callback(callback) => Callable::run(callback, new_value),
            Self::WriteSignal(write_signal) => write_signal.set(new_value),
            Self::RwSignal(rw_signal) => rw_signal.set(new_value),
            Self::StoredValue(stored_value) => stored_value.set_value(new_value),
        }
    }
}

impl<T, F, S> From<F> for Out<T, S>
where
    T: 'static,
    F: Fn(T) + Send + Sync + 'static,
{
    fn from(fun: F) -> Self {
        Self::new_callback(fun)
    }
}

impl<O: 'static> From<Callback<O, ()>> for Out<O, SyncStorage> {
    fn from(callback: Callback<O, ()>) -> Self {
        Self::Callback(callback)
    }
}

#[cfg(not(feature = "nightly"))]
impl<O: 'static, S> From<WriteSignal<O, S>> for Out<O, S> {
    fn from(write_signal: WriteSignal<O, S>) -> Self {
        Self::WriteSignal(write_signal)
    }
}

#[cfg(not(feature = "nightly"))]
impl<O: 'static, S> From<RwSignal<O, S>> for Out<O, S> {
    fn from(rw_signal: RwSignal<O, S>) -> Self {
        Self::RwSignal(rw_signal)
    }
}

impl<O: 'static, S> From<StoredValue<O, S>> for Out<O, S> {
    fn from(stored_value: StoredValue<O, S>) -> Self {
        Self::StoredValue(stored_value)
    }
}

/// Create a read-write signal pair kept in the browser's `LocalStorage` under `key`.
///
/// It starts with `initial`, as the server renders it (the server has no storage), so hydration
/// matches; once hydrated, the stored value (if any) replaces it, and every change is stored. In
/// apps that render on the server, the stored value therefore shows one frame after loading.
pub fn signal_ls<
    T: Send + Sync + Clone + serde::Serialize + serde::de::DeserializeOwned + 'static,
>(
    key: &'static str,
    initial: T,
) -> (ReadSignal<T>, WriteSignal<T>) {
    let (value, set_value) = signal(initial);
    // Effects run on the client only, after hydration. The first run loads, later runs store (a
    // separate storing effect could store `initial` before the load).
    Effect::new(move |loaded: Option<()>| {
        if loaded.is_none() {
            if let Some(stored) = read_from_local_storage::<T>(key) {
                set_value.set(stored);
            }
            value.track();
            return;
        }
        if let Some(window) = &*use_window()
            && let Ok(Some(storage)) = window.local_storage()
            && let Ok(json) = value.with(serde_json::to_string)
        {
            let _ = storage.set(key, &json);
        }
    });
    (value, set_value)
}

fn read_from_local_storage<T: serde::de::DeserializeOwned>(key: &'static str) -> Option<T> {
    use_window().as_ref().and_then(|window| {
        let storage = window.local_storage().ok()??;
        let stored = storage.get(key).ok()??;
        match serde_json::from_str(&stored) {
            Ok(des) => Some(des),
            Err(err) => {
                tracing::error!(
                    "Could not deserialize local-storage value at key '{key}'. Received '{stored}'. Tried to convert to '{ty}'. App may continue using a default value. Err: {err}",
                    ty = std::any::type_name::<T>()
                );
                None
            }
        }
    })
}
