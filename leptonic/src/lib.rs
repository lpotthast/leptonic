use std::fmt::Display;

use leptos::*;

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
pub mod contexts;
pub mod date_selector;
pub mod datetime;
pub mod datetime_input;
pub mod drawer;
pub mod grid;
pub mod icon;
pub mod input;
pub mod link;
pub mod modal;
pub mod progress_bar;
pub mod quicksearch;
pub mod root;
pub mod safe_html;
pub mod select;
pub mod separator;
pub mod skeleton;
pub mod slider;
pub mod stack;
pub mod tab;
pub mod tabs;
pub mod theme;
pub mod tile;
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

pub mod prelude {
    pub use leptos_tiptap::*;

    pub use super::alert::Alert;
    pub use super::alert::AlertVariant;
    pub use super::anchor::Anchor;
    pub use super::app_bar::AppBar;
    pub use super::button::Button;
    pub use super::button::ButtonColor;
    pub use super::button::ButtonGroup;
    pub use super::button::ButtonSize;
    pub use super::button::ButtonVariant;
    pub use super::button::ButtonWrapper;
    pub use super::callback::create_callback;
    pub use super::callback::create_callback_arc;
    pub use super::callback::create_callback_rc;
    pub use super::callback::create_simple_callback;
    pub use super::callback::Callable;
    pub use super::callback::Callback;
    pub use super::callback::CallbackArc;
    pub use super::callback::CallbackRc;
    pub use super::callback::SimpleCallback;
    pub use super::card::Card;
    pub use super::checkbox::Checkbox;
    pub use super::chip::Chip;
    pub use super::chip::ChipColor;
    pub use super::collapsible::Collapsible;
    pub use super::collapsible::CollapsibleBody;
    pub use super::collapsible::CollapsibleHeader;
    pub use super::collapsible::Collapsibles;
    pub use super::collapsible::OnOpen;
    pub use super::contexts::global_click_event::GlobalClickEvent;
    pub use super::contexts::global_keyboard_event::GlobalKeyboardEvent;
    pub use super::create_signal_ls;
    pub use super::date_selector::DateSelector;
    pub use super::datetime_input::DateTimeInput;
    pub use super::drawer::Drawer;
    pub use super::grid::Col;
    pub use super::grid::ColAlign;
    pub use super::grid::Grid;
    pub use super::grid::Row;
    pub use super::icon::Icon;
    pub use super::input::Input;
    pub use super::input::InputType;
    pub use super::link::Link;
    pub use super::link::LinkExt;
    pub use super::link::LinkExtTarget;
    pub use super::modal::Modal;
    pub use super::modal::ModalBody;
    pub use super::modal::ModalData;
    pub use super::modal::ModalFn;
    pub use super::modal::ModalFooter;
    pub use super::modal::ModalHeader;
    pub use super::modal::ModalRoot;
    pub use super::modal::ModalTitle;
    pub use super::progress_bar::ProgressBar;
    pub use super::quicksearch::Quicksearch;
    pub use super::r#box::Box;
    pub use super::root::Root;
    pub use super::safe_html::SafeHtml;
    pub use super::select::Multiselect;
    pub use super::select::OptionalSelect;
    pub use super::select::Select;
    pub use super::separator::Separator;
    pub use super::skeleton::Skeleton;
    pub use super::slider::Slider;
    pub use super::slider::SliderVariant;
    pub use super::stack::Stack;
    pub use super::stack::StackOrientation;
    pub use super::tab::Tab;
    pub use super::tabs::Tabs;
    pub use super::theme::Theme;
    pub use super::theme::ThemeContext;
    pub use super::theme::ThemeProvider;
    pub use super::theme::ThemeToggle;
    pub use super::tile::Tile;
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
    pub use super::typography::Typography;
    pub use super::typography::TypographyVariant;
    pub use super::typography::H1;
    pub use super::typography::H2;
    pub use super::typography::H3;
    pub use super::typography::H4;
    pub use super::typography::H5;
    pub use super::typography::H6;
    pub use super::typography::P;
    pub use super::FirstOf;
    pub use super::FontWeight;
    pub use super::Height;
    pub use super::If;
    pub use super::LastOf;
    pub use super::Margin;
    pub use super::Mount;
    pub use super::OptionDeref;
    pub use super::OptionalMaybeSignal;
    pub use super::OptionalSignal;
    pub use super::Size;
    pub use super::Width;
    pub use super::With;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mount {
    /// Mount the child view once. Then keep it mounted as long as the parent lives.
    Once,
    /// Mount the child view once. May defer mounting to the point where the view is first needed. Then keep it mounted as long as the parent lives.
    OnceShown,
    /// Always re-mount the child view when it is needed.
    WhenShown,
}

#[component]
pub fn If<IF>(cx: Scope, sig: IF, children: ChildrenFn) -> impl IntoView
where
    IF: Fn() -> bool + 'static,
{
    move || sig().then(|| children(cx))
}

#[component]
pub fn IfEl<IF, THEN, ELSE, T, E>(cx: Scope, sig: IF, then: THEN, el: ELSE) -> impl IntoView
where
    IF: Fn() -> bool + 'static,
    THEN: Fn() -> T + 'static,
    ELSE: Fn() -> E + 'static,
    T: IntoView + 'static,
    E: IntoView + 'static,
{
    move || match sig() {
        true => (then()).into_view(cx),
        false => (el()).into_view(cx),
    }
}

#[component]
pub fn FirstOf<IF, I, T, EF, N>(cx: Scope, iter: IF, view: EF) -> impl IntoView
where
    IF: Fn() -> I + 'static,
    I: IntoIterator<Item = T>,
    EF: Fn(Scope, T) -> N + 'static,
    N: IntoView,
    T: 'static,
{
    move || iter().into_iter().next().map(|t| view(cx, t))
}

#[component]
pub fn LastOf<IF, I, T, EF, N>(cx: Scope, iter: IF, view: EF) -> impl IntoView
where
    IF: Fn() -> I + 'static,
    I: IntoIterator<Item = T>,
    EF: Fn(Scope, T) -> N + 'static,
    N: IntoView,
    T: 'static,
{
    move || iter().into_iter().last().map(|t| view(cx, t))
}

#[component]
pub fn LastOfRef<'a, IF, I, T, EF, N>(cx: Scope, iter: IF, view: EF) -> impl IntoView
where
    IF: Fn() -> I + 'static,
    I: IntoIterator<Item = &'a T> + 'a,
    EF: Fn(Scope, &'a T) -> N + 'static,
    N: IntoView + 'a,
    T: 'static,
{
    move || iter().into_iter().last().map(|t| view(cx, t))
}

#[component]
pub fn With<'t, IF, T, EF, N>(cx: Scope, item: IF, view: EF) -> impl IntoView
where
    IF: Fn() -> Option<&'t T> + 'static,
    EF: Fn(Scope, &'t T) -> N + 'static,
    N: IntoView + 't,
    T: 't,
{
    move || item().map(|t| view(cx, t))
}

pub fn create_signal_ls<T: Clone + serde::Serialize + serde::de::DeserializeOwned>(
    cx: Scope,
    key: &'static str,
    initial: T,
) -> (ReadSignal<T>, WriteSignal<T>) {
    let (signal, set_signal) =
        create_signal(cx, read_from_local_storage::<T>(key).unwrap_or(initial));

    track_in_local_storage(cx, key, signal);

    (signal, set_signal)
}

pub fn read_from_local_storage<T: serde::de::DeserializeOwned>(key: &'static str) -> Option<T> {
    let storage = window().local_storage().ok()??;
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
}

pub fn track_in_local_storage<T: serde::Serialize + Clone>(
    cx: Scope,
    key: &'static str,
    signal: ReadSignal<T>,
) {
    create_effect(cx, move |_old| {
        let storage = window().local_storage().ok()??;
        storage
            .set(key, serde_json::to_string(&signal.get()).ok()?.as_ref())
            .ok()
    })
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
        self.as_ref().map(std::ops::Deref::deref).unwrap_or(default)
    }

    fn deref_or_else<'a, F: Fn() -> &'a T::Target>(&'a self, default: F) -> &'a T::Target {
        self.as_ref()
            .map(std::ops::Deref::deref)
            .unwrap_or_else(default)
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
            Size::Zero => f.write_str("0"),
            Size::Px(px) => f.write_fmt(format_args!("{px}px")),
            Size::Em(em) => f.write_fmt(format_args!("{em}em")),
            Size::Rem(rem) => f.write_fmt(format_args!("{rem}rem")),
            Size::Percent(percent) => f.write_fmt(format_args!("{percent}%")),
            Size::Auto => f.write_str("auto"),
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
            FontWeight::W100 => f.write_str("100"),
            FontWeight::W200 => f.write_str("200"),
            FontWeight::W300 => f.write_str("300"),
            FontWeight::W400 => f.write_str("400"),
            FontWeight::W500 => f.write_str("500"),
            FontWeight::W600 => f.write_str("600"),
            FontWeight::W700 => f.write_str("700"),
            FontWeight::W800 => f.write_str("800"),
            FontWeight::W900 => f.write_str("900"),
            FontWeight::WLighter => f.write_str("lighter"),
            FontWeight::WNormal => f.write_str("normal"),
            FontWeight::WBold => f.write_str("bold"),
            FontWeight::WBolder => f.write_str("bolder"),
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
            Margin::Top(size) => f.write_fmt(format_args!("{size} 0 0 0")),
            Margin::Right(size) => f.write_fmt(format_args!("0 {size} 0 0")),
            Margin::Bottom(size) => f.write_fmt(format_args!("0 0 {size} 0")),
            Margin::Left(size) => f.write_fmt(format_args!("0 0 0 {size}")),
            Margin::All(size) => f.write_fmt(format_args!("{size}")),
            Margin::Double(vertical, horizontal) => {
                f.write_fmt(format_args!("{vertical} {horizontal}"))
            }
            Margin::Full(top, right, bottom, left) => {
                f.write_fmt(format_args!("{top} {right} {bottom} {left}"))
            }
        }
    }
}
