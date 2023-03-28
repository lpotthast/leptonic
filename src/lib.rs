use leptos::*;

pub mod alert;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod collapsible;
pub mod icon;
pub mod modal;
pub mod progress_bar;
pub mod separator;
pub mod tab;
pub mod tabs;
pub mod tile;
pub mod toast;
pub mod toggle;

#[derive(Debug, Clone, Copy)]
pub enum Bool {
    Static(bool),
    Reactive(Signal<bool>),
}

impl Default for Bool {
    fn default() -> Self {
        Self::Static(false)
    }
}

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        Self::Static(value)
    }
}
impl From<ReadSignal<bool>> for Bool {
    fn from(value: ReadSignal<bool>) -> Self {
        Self::Reactive(value.into())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Active {
    Static(bool),
    Reactive(ReadSignal<bool>),
}

#[derive(Debug, Clone, Copy)]
pub enum Disabled {
    Static(bool),
    Reactive(ReadSignal<bool>),
}

pub mod prelude {
    pub use super::alert::Alert;
    pub use super::alert::AlertProps;
    pub use super::alert::AlertVariant;
    pub use super::button::Button;
    pub use super::button::ButtonGroup;
    pub use super::button::ButtonGroupProps;
    pub use super::button::ButtonProps;
    pub use super::button::ButtonVariant;
    pub use super::button::ButtonWrapper;
    pub use super::button::ButtonWrapperProps;
    pub use super::card::Card;
    pub use super::card::CardProps;
    pub use super::checkbox::Checkbox;
    pub use super::checkbox::CheckboxProps;
    pub use super::collapsible::OnOpen;
    pub use super::collapsible::Collapsible;
    pub use super::collapsible::CollapsibleBody;
    pub use super::collapsible::CollapsibleBodyProps;
    pub use super::collapsible::CollapsibleHeader;
    pub use super::collapsible::CollapsibleHeaderProps;
    pub use super::collapsible::CollapsibleProps;
    pub use super::collapsible::Collapsibles;
    pub use super::collapsible::CollapsiblesProps;
    pub use super::icon::Icon;
    pub use super::icon::IconProps;
    pub use super::modal::Modal;
    pub use super::modal::ModalBody;
    pub use super::modal::ModalBodyProps;
    pub use super::modal::ModalFooter;
    pub use super::modal::ModalFooterProps;
    pub use super::modal::ModalHeader;
    pub use super::modal::ModalHeaderProps;
    pub use super::modal::ModalProps;
    pub use super::modal::ModalRoot;
    pub use super::modal::ModalRootProps;
    pub use super::modal::ModalTitle;
    pub use super::modal::ModalTitleProps;
    pub use super::progress_bar::ProgressBar;
    pub use super::progress_bar::ProgressBarProps;
    pub use super::separator::Separator;
    pub use super::separator::SeparatorProps;
    pub use super::tab::Tab;
    pub use super::tab::TabProps;
    pub use super::tabs::Tabs;
    pub use super::tabs::TabsProps;
    pub use super::tile::Tile;
    pub use super::tile::TileProps;
    pub use super::toast::Toast;
    pub use super::toast::ToastRoot;
    pub use super::toast::ToastRootProps;
    pub use super::toast::ToastVariant;
    pub use super::toast::Toasts;
    pub use super::toast::ToastTimeout;
    pub use super::toggle::Toggle;
    pub use super::toggle::ToggleProps;
    pub use super::Active;
    pub use super::Disabled;
    pub use super::FirstOf;
    pub use super::FirstOfProps;
    pub use super::If;
    pub use super::IfProps;
    pub use super::LastOf;
    pub use super::LastOfProps;
    pub use super::With;
    pub use super::WithProps;
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
