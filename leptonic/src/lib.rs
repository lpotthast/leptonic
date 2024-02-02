use std::fmt::Display;

use leptos::*;
use leptos_use::use_window;
use prelude::Consumer;

pub mod alert;
pub mod anchor;
pub mod app_bar;
pub mod r#box;
pub mod button;
pub mod callback;
pub mod card;
pub mod checkbox;
pub mod chip;
pub mod collapsible;
pub mod color;
pub mod color_picker;
pub mod contexts;
pub mod date_selector;
pub mod datetime;
pub mod datetime_input;
pub mod drawer;
pub mod field;
pub mod grid;
pub mod icon;
pub mod input;
pub mod kbd;
pub mod link;
pub mod math;
pub mod modal;
pub mod popover;
pub mod progress_bar;
pub mod quicksearch;
pub mod radio;
pub mod root;
pub mod safe_html;
pub mod select;
pub mod separator;
pub mod skeleton;
pub mod slider;
pub mod stack;
pub mod tab;
pub mod table;
pub mod tabs;
pub mod theme;
pub mod tile;
#[cfg(feature = "tiptap")]
pub mod tiptap_editor;
pub mod toast;
pub mod toggle;
pub mod transitions;
pub mod typography;

#[derive(Debug, Clone)]
pub struct OptionalSignal<T: 'static>(Option<Signal<T>>);

impl<T> Default for OptionalSignal<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: 'static, I: Into<Signal<T>>> From<I> for OptionalSignal<T> {
    fn from(value: I) -> Self {
        Self(Some(value.into()))
    }
}

#[derive(Debug, Clone)]
pub struct OptionalMaybeSignal<T: 'static>(Option<MaybeSignal<T>>);

impl<T: Clone> OptionalMaybeSignal<T> {
    pub fn or<D: Into<MaybeSignal<T>>>(self, default: D) -> MaybeSignal<T> {
        match self.0 {
            Some(maybe_signal) => maybe_signal,
            None => default.into(),
        }
    }

    pub fn or_default(self) -> MaybeSignal<T>
    where
        T: Default,
    {
        match self.0 {
            Some(maybe_signal) => maybe_signal,
            None => MaybeSignal::Static(T::default()),
        }
    }

    pub fn map<U: 'static, F: Fn(T) -> U + 'static>(self, map: F) -> OptionalMaybeSignal<U> {
        match self.0 {
            Some(maybe_signal) => match maybe_signal {
                MaybeSignal::Static(v) => MaybeSignal::Static(map(v)).into(),
                MaybeSignal::Dynamic(sig) => {
                    MaybeSignal::Dynamic(Signal::derive(move || map(sig.get()))).into()
                }
            },
            None => OptionalMaybeSignal(None),
        }
    }
}

impl<T: Copy> Copy for OptionalMaybeSignal<T> {}

impl<T> Default for OptionalMaybeSignal<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: 'static, I: Into<MaybeSignal<T>>> From<I> for OptionalMaybeSignal<T> {
    fn from(value: I) -> Self {
        Self(Some(value.into()))
    }
}

impl<T: Clone + Default> SignalGet for OptionalMaybeSignal<T> {
    type Value = T;

    fn get(&self) -> T {
        match &self.0 {
            Some(signal) => signal.get(),
            None => T::default(),
        }
    }

    fn try_get(&self) -> Option<T> {
        match &self.0 {
            Some(signal) => signal.try_get(),
            None => Some(T::default()),
        }
    }
}

impl<T: Clone + Default> SignalGetUntracked for OptionalMaybeSignal<T> {
    type Value = T;

    fn get_untracked(&self) -> T {
        match &self.0 {
            Some(signal) => signal.get_untracked(),
            None => T::default(),
        }
    }

    fn try_get_untracked(&self) -> Option<T> {
        match &self.0 {
            Some(signal) => signal.try_get_untracked(),
            None => Some(T::default()),
        }
    }
}

impl<T: IntoAttribute + Clone> IntoAttribute for OptionalMaybeSignal<T> {
    fn into_attribute(self) -> Attribute {
        match self.0 {
            Some(t) => t.into_attribute(), // Requires T to be Clone!
            None => Attribute::Option(None),
        }
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        match self.0 {
            Some(t) => t.into_attribute(), // Requires T to be Clone!
            None => Attribute::Option(None),
        }
    }
}

pub trait MaybeSignalExt<T: 'static> {
    fn map<U: 'static, F: Fn(T) -> U + 'static>(self, mapper: F) -> MaybeSignal<U>;
}

impl<T: Clone + 'static> MaybeSignalExt<T> for MaybeSignal<T> {
    fn map<U: 'static, F: Fn(T) -> U + 'static>(self, mapper: F) -> MaybeSignal<U> {
        match self {
            Self::Static(v) => MaybeSignal::Static(mapper(v)),
            Self::Dynamic(sig) => MaybeSignal::Dynamic(Signal::derive(move || mapper(sig.get()))),
        }
    }
}

pub mod prelude {
    #[cfg(feature = "tiptap")]
    pub use leptos_tiptap::*;

    // Reexport
    pub use icondata;

    pub use super::alert::Alert;
    pub use super::alert::AlertAppend;
    pub use super::alert::AlertContent;
    pub use super::alert::AlertIcon;
    pub use super::alert::AlertIconSlot;
    pub use super::alert::AlertPrepend;
    pub use super::alert::AlertTitle;
    pub use super::alert::AlertVariant;
    pub use super::anchor::Anchor;
    pub use super::app_bar::AppBar;
    pub use super::button::Button;
    pub use super::button::ButtonColor;
    pub use super::button::ButtonGroup;
    pub use super::button::ButtonSize;
    pub use super::button::ButtonVariant;
    pub use super::button::ButtonWrapper;
    pub use super::button::LinkButton;
    pub use super::callback::consumer;
    pub use super::callback::producer;
    pub use super::callback::Consumer;
    pub use super::callback::Producer;
    pub use super::callback::ViewCallback;
    pub use super::callback::ViewProducer;
    pub use super::card::Card;
    pub use super::checkbox::Checkbox;
    pub use super::chip::Chip;
    pub use super::chip::ChipColor;
    pub use super::collapsible::Collapsible;
    pub use super::collapsible::CollapsibleBody;
    pub use super::collapsible::CollapsibleHeader;
    pub use super::collapsible::Collapsibles;
    pub use super::collapsible::OnOpen;
    pub use super::color::ColorSpace;
    pub use super::color::HSV;
    pub use super::color::RGB8;
    pub use super::color::RGBA8;
    pub use super::color_picker::ColorPalette;
    pub use super::color_picker::ColorPicker;
    pub use super::color_picker::ColorPreview;
    pub use super::color_picker::HueSlider;
    pub use super::contexts::global_click_event::GlobalClickEvent;
    pub use super::contexts::global_keyboard_event::GlobalKeyboardEvent;
    pub use super::create_signal_ls;
    pub use super::date_selector::DateSelector;
    pub use super::datetime_input::DateTimeInput;
    pub use super::drawer::Drawer;
    pub use super::drawer::DrawerSide;
    pub use super::field::Field;
    pub use super::field::FieldLabel;
    pub use super::grid::Col;
    pub use super::grid::ColAlign;
    pub use super::grid::Grid;
    pub use super::grid::Row;
    pub use super::icon::Icon;
    pub use super::input::NumberInput;
    pub use super::input::PasswordInput;
    pub use super::input::TextInput;
    pub use super::kbd::KbdConcatenate;
    pub use super::kbd::KbdKey;
    pub use super::kbd::KbdShortcut;
    pub use super::kbd::KbdShortcutRoot;
    pub use super::kbd::Key;
    pub use super::link::Link;
    pub use super::link::LinkExt;
    pub use super::link::LinkExtTarget;
    pub use super::modal::Modal;
    pub use super::modal::ModalBody;
    pub use super::modal::ModalFooter;
    pub use super::modal::ModalHeader;
    pub use super::modal::ModalRoot;
    pub use super::modal::ModalTitle;
    pub use super::popover::Popover;
    pub use super::progress_bar::ProgressBar;
    pub use super::quicksearch::Quicksearch;
    pub use super::quicksearch::QuicksearchOption;
    pub use super::quicksearch::QuicksearchTrigger;
    pub use super::r#box::Box;
    pub use super::radio::Radio;
    pub use super::radio::RadioGroup;
    pub use super::root::Leptonic;
    pub use super::root::Root;
    pub use super::safe_html::SafeHtml;
    pub use super::select::Multiselect;
    pub use super::select::OptionalSelect;
    pub use super::select::Select;
    pub use super::separator::Separator;
    pub use super::skeleton::Skeleton;
    pub use super::slider::RangeSlider;
    pub use super::slider::Slider;
    pub use super::slider::SliderMark;
    pub use super::slider::SliderMarkValue;
    pub use super::slider::SliderMarks;
    pub use super::slider::SliderPopover;
    pub use super::slider::SliderVariant;
    pub use super::stack::Stack;
    pub use super::stack::StackOrientation;
    pub use super::tab::Tab;
    pub use super::table::Table;
    pub use super::table::TableBody;
    pub use super::table::TableCell;
    pub use super::table::TableContainer;
    pub use super::table::TableFooter;
    pub use super::table::TableHeader;
    pub use super::table::TableHeaderCell;
    pub use super::table::TableRow;
    pub use super::tabs::Tabs;
    pub use super::theme::LeptonicTheme;
    pub use super::theme::Theme;
    pub use super::theme::ThemeContext;
    pub use super::theme::ThemeProvider;
    pub use super::theme::ThemeToggle;
    pub use super::tile::Tile;
    #[cfg(feature = "tiptap")]
    pub use super::tiptap_editor::TiptapEditor;
    pub use super::toast::Toast;
    pub use super::toast::ToastRoot;
    pub use super::toast::ToastTimeout;
    pub use super::toast::ToastVariant;
    pub use super::toast::Toasts;
    pub use super::toggle::Toggle;
    pub use super::toggle::ToggleIcons;
    pub use super::toggle::ToggleSize;
    pub use super::toggle::ToggleVariant;
    pub use super::transitions::collapse::Collapse;
    pub use super::transitions::collapse::CollapseAxis;
    pub use super::transitions::fade::Fade;
    pub use super::transitions::grow::Grow;
    pub use super::transitions::slide::Slide;
    pub use super::transitions::zoom::Zoom;
    pub use super::typography::Code;
    pub use super::typography::Li;
    pub use super::typography::Ul;
    pub use super::typography::H1;
    pub use super::typography::H2;
    pub use super::typography::H3;
    pub use super::typography::H4;
    pub use super::typography::H5;
    pub use super::typography::H6;
    pub use super::typography::P;
    pub use super::FontWeight;
    pub use super::Height;
    pub use super::Margin;
    pub use super::Mount;
    pub use super::OptionDeref;
    pub use super::OptionalMaybeSignal;
    pub use super::OptionalSignal;
    pub use super::Out;
    pub use super::Size;
    pub use super::Width;
}

#[derive(Debug, Clone, Copy)]
pub enum Language {
    En,
}

#[derive(Debug)]
pub enum Out<O: 'static> {
    Consumer(Consumer<O>),
    Callback(Callback<O, ()>),
    WriteSignal(WriteSignal<O>),
    RwSignal(RwSignal<O>),
}

impl<O: 'static> Copy for Out<O> {}

impl<O: 'static> Clone for Out<O> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<O: 'static> Out<O> {
    pub fn set(&self, new_value: O) {
        match self {
            Self::Consumer(consumer) => consumer.consume(new_value),
            Self::Callback(callback) => callback.call(new_value),
            Self::WriteSignal(write_signal) => write_signal.set(new_value),
            Self::RwSignal(rw_signal) => rw_signal.set(new_value),
        }
    }
}

impl<T: 'static, F: Fn(T) + 'static> From<F> for Out<T> {
    fn from(fun: F) -> Self {
        Self::Consumer(fun.into())
    }
}

impl<O: 'static> From<Consumer<O>> for Out<O> {
    fn from(consumer: Consumer<O>) -> Self {
        Self::Consumer(consumer)
    }
}

impl<O: 'static> From<Callback<O, ()>> for Out<O> {
    fn from(callback: Callback<O, ()>) -> Self {
        Self::Callback(callback)
    }
}

impl<O: 'static> From<WriteSignal<O>> for Out<O> {
    fn from(write_signal: WriteSignal<O>) -> Self {
        Self::WriteSignal(write_signal)
    }
}

impl<O: 'static> From<RwSignal<O>> for Out<O> {
    fn from(rw_signal: RwSignal<O>) -> Self {
        Self::RwSignal(rw_signal)
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mount {
    /// Mount the child view once. Then keep it mounted as long as the parent lives.
    #[default]
    Once,
    /// Mount the child view once. May defer mounting to the point where the view is first needed. Then keep it mounted as long as the parent lives.
    // OnceShown, // TODO: Implement this variant in tabs.
    /// Always re-mount the child view when it is needed.
    WhenShown,
}

pub fn create_signal_ls<T: Clone + serde::Serialize + serde::de::DeserializeOwned>(
    key: &'static str,
    initial: T,
) -> (ReadSignal<T>, WriteSignal<T>) {
    let (signal, set_signal) = create_signal(read_from_local_storage::<T>(key).unwrap_or(initial));

    track_in_local_storage(key, signal);

    (signal, set_signal)
}

#[must_use]
pub fn read_from_local_storage<T: serde::de::DeserializeOwned>(key: &'static str) -> Option<T> {
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

pub fn track_in_local_storage<T: serde::Serialize + Clone>(
    key: &'static str,
    signal: ReadSignal<T>,
) {
    create_effect(move |_old| {
        if let Some(window) = &*use_window() {
            let storage = window.local_storage().ok()??;
            storage
                .set(key, serde_json::to_string(&signal.get()).ok()?.as_ref())
                .ok()
        } else {
            Some(())
        }
    });
}

pub trait OptionDeref<T: std::ops::Deref> {
    fn deref(&self) -> Option<&T::Target>;
    fn deref_or<'a>(&'a self, default: &'a T::Target) -> &'a T::Target;
    fn deref_or_else<'a, F: Fn() -> &'a T::Target>(&'a self, default: F) -> &'a T::Target;
}

impl<T: std::ops::Deref> OptionDeref<T> for Option<T> {
    fn deref(&self) -> Option<&T::Target> {
        self.as_ref().map(std::ops::Deref::deref)
    }

    fn deref_or<'a>(&'a self, default: &'a T::Target) -> &'a T::Target {
        self.as_ref().map_or(default, std::ops::Deref::deref)
    }

    fn deref_or_else<'a, F: Fn() -> &'a T::Target>(&'a self, default: F) -> &'a T::Target {
        self.as_ref().map_or_else(default, std::ops::Deref::deref)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Zero,
    Px(i32),
    Em(f32),
    Rem(f32),
    Percent(f32),
    Auto,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => f.write_str("0px"), // Having a unit is relevant here. Using it inside a calc() functions would otherwise not work!
            Self::Px(px) => f.write_fmt(format_args!("{px}px")),
            Self::Em(em) => f.write_fmt(format_args!("{em}em")),
            Self::Rem(rem) => f.write_fmt(format_args!("{rem}rem")),
            Self::Percent(percent) => f.write_fmt(format_args!("{percent}%")),
            Self::Auto => f.write_str("auto"),
        }
    }
}

pub type Width = Size;
pub type Height = Size;

#[derive(Debug, Clone, Copy)]
pub enum FontWeight {
    W100,
    W200,
    W300,
    W400,
    W500,
    W600,
    W700,
    W800,
    W900,
    WLighter,
    WNormal,
    WBold,
    WBolder,
}

impl Display for FontWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::W100 => f.write_str("100"),
            Self::W200 => f.write_str("200"),
            Self::W300 => f.write_str("300"),
            Self::W400 => f.write_str("400"),
            Self::W500 => f.write_str("500"),
            Self::W600 => f.write_str("600"),
            Self::W700 => f.write_str("700"),
            Self::W800 => f.write_str("800"),
            Self::W900 => f.write_str("900"),
            Self::WLighter => f.write_str("lighter"),
            Self::WNormal => f.write_str("normal"),
            Self::WBold => f.write_str("bold"),
            Self::WBolder => f.write_str("bolder"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Margin {
    Top(Size),
    Right(Size),
    Bottom(Size),
    Left(Size),
    All(Size),
    Double(Size, Size),
    Full(Size, Size, Size, Size),
}

impl Display for Margin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Top(size) => f.write_fmt(format_args!("{size} 0 0 0")),
            Self::Right(size) => f.write_fmt(format_args!("0 {size} 0 0")),
            Self::Bottom(size) => f.write_fmt(format_args!("0 0 {size} 0")),
            Self::Left(size) => f.write_fmt(format_args!("0 0 0 {size}")),
            Self::All(size) => f.write_fmt(format_args!("{size}")),
            Self::Double(vertical, horizontal) => {
                f.write_fmt(format_args!("{vertical} {horizontal}"))
            }
            Self::Full(top, right, bottom, left) => {
                f.write_fmt(format_args!("{top} {right} {bottom} {left}"))
            }
        }
    }
}

/// Keep track of an elements position and size.
/// Call `track_client_rect` to update the signal state.
#[derive(Debug, Clone)]
struct TrackedElementClientBoundingRect<T: Into<web_sys::Element> + Clone + 'static> {
    el: StoredValue<leptos_use::core::ElementMaybeSignal<T, web_sys::Element>>,
    /// Distance of the tracked element to the left of the viewport.
    pub(crate) left: ReadSignal<f64>,
    /// Distance of the tracked element to the top of the viewport.
    pub(crate) top: ReadSignal<f64>,
    /// Width of the tracked element.
    pub(crate) width: ReadSignal<f64>,
    /// Height of the tracked element.
    pub(crate) height: ReadSignal<f64>,
    set_left: WriteSignal<f64>,
    set_top: WriteSignal<f64>,
    set_width: WriteSignal<f64>,
    set_height: WriteSignal<f64>,
}

impl<T: Into<web_sys::Element> + Clone + 'static> Copy for TrackedElementClientBoundingRect<T> {}

impl<T> TrackedElementClientBoundingRect<T>
where
    T: Into<web_sys::Element> + Clone + 'static,
{
    pub(crate) fn new<El>(el: El) -> Self
    where
        El: Clone + Into<leptos_use::core::ElementMaybeSignal<T, web_sys::Element>>,
    {
        let (left, set_left) = create_signal(0.0);
        let (top, set_top) = create_signal(0.0);
        let (width, set_width) = create_signal(0.0);
        let (height, set_height) = create_signal(0.0);

        Self {
            el: store_value(el.into()),
            left,
            set_left,
            top,
            set_top,
            width,
            set_width,
            height,
            set_height,
        }
    }

    pub(crate) fn track_client_rect(&self) {
        self.el.with_value(|maybe_signal| {
            if let Some(el) = maybe_signal.get_untracked() {
                let el: web_sys::Element = el.into();
                let rect = el.get_bounding_client_rect();
                self.set_left.set(rect.left());
                self.set_top.set(rect.top());
                self.set_width.set(rect.width());
                self.set_height.set(rect.height());
            }
        });
    }
}

struct RelativeMousePosition {
    rel_mouse_pos: Memo<(f64, f64)>,
}

impl RelativeMousePosition {
    pub(crate) fn new<T>(client_bounding_rect: TrackedElementClientBoundingRect<T>) -> Self
    where
        T: Into<web_sys::Element> + Clone + 'static,
    {
        let leptos_use::UseMouseReturn {
            x: cursor_x,
            y: cursor_y,
            ..
        } = leptos_use::use_mouse();

        let (x, set_x) = create_signal(0.0);
        let (y, set_y) = create_signal(0.0);

        let _ = leptos_use::watch_throttled_with_options(
            move || (cursor_x.get(), cursor_y.get()),
            move |(cursor_x, cursor_y), _, _| {
                set_x.set(*cursor_x);
                set_y.set(*cursor_y);
            },
            5.0, // Limit to 200 updates / sec.
            leptos_use::WatchThrottledOptions::default()
                .leading(true)
                .trailing(true),
        );

        Self {
            rel_mouse_pos: create_memo(move |_| {
                let x = x.get() - client_bounding_rect.left.get();
                let y = y.get() - client_bounding_rect.top.get();
                let px = (x / client_bounding_rect.width.get()).clamp(0.0, 1.0);
                let py = (y / client_bounding_rect.height.get()).clamp(0.0, 1.0);
                (px, py)
            }),
        }
    }
}
