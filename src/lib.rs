use leptos::*;

pub mod alert;
pub mod r#box;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod collapsible;
pub mod grid;
pub mod icon;
pub mod modal;
pub mod progress_bar;
pub mod separator;
pub mod skeleton;
pub mod stack;
pub mod tab;
pub mod tabs;
pub mod theme;
pub mod tile;
pub mod toast;
pub mod toggle;
pub mod transitions;

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
    pub use super::button::ButtonColor;
    pub use super::button::ButtonGroup;
    pub use super::button::ButtonGroupProps;
    pub use super::button::ButtonProps;
    pub use super::button::ButtonWrapper;
    pub use super::button::ButtonWrapperProps;
    pub use super::card::Card;
    pub use super::card::CardProps;
    pub use super::checkbox::Checkbox;
    pub use super::checkbox::CheckboxProps;
    pub use super::collapsible::Collapsible;
    pub use super::collapsible::CollapsibleBody;
    pub use super::collapsible::CollapsibleBodyProps;
    pub use super::collapsible::CollapsibleHeader;
    pub use super::collapsible::CollapsibleHeaderProps;
    pub use super::collapsible::CollapsibleProps;
    pub use super::collapsible::Collapsibles;
    pub use super::collapsible::CollapsiblesProps;
    pub use super::collapsible::OnOpen;
    pub use super::create_signal_ls;
    pub use super::grid::Col;
    pub use super::grid::ColProps;
    pub use super::grid::Grid;
    pub use super::grid::GridProps;
    pub use super::grid::Row;
    pub use super::grid::RowProps;
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
    pub use super::r#box::Box;
    pub use super::r#box::BoxProps;
    pub use super::separator::Separator;
    pub use super::separator::SeparatorProps;
    pub use super::skeleton::Skeleton;
    pub use super::skeleton::SkeletonProps;
    pub use super::stack::Stack;
    pub use super::stack::StackProps;
    pub use super::tab::Tab;
    pub use super::tab::TabProps;
    pub use super::tabs::Tabs;
    pub use super::tabs::TabsProps;
    pub use super::theme::DarkThemeToggle;
    pub use super::theme::DarkThemeToggleProps;
    pub use super::theme::Theme;
    pub use super::theme::ThemeContext;
    pub use super::theme::ThemeProvider;
    pub use super::theme::ThemeProviderProps;
    pub use super::tile::Tile;
    pub use super::tile::TileProps;
    pub use super::toast::Toast;
    pub use super::toast::ToastRoot;
    pub use super::toast::ToastRootProps;
    pub use super::toast::ToastTimeout;
    pub use super::toast::ToastVariant;
    pub use super::toast::Toasts;
    pub use super::toggle::Toggle;
    pub use super::toggle::ToggleIcons;
    pub use super::toggle::ToggleProps;
    pub use super::transitions::collapse::Collapse;
    pub use super::transitions::collapse::CollapseProps;
    pub use super::transitions::collapse::CollapseAxis;
    pub use super::transitions::fade::Fade;
    pub use super::transitions::fade::FadeProps;
    pub use super::transitions::grow::Grow;
    pub use super::transitions::grow::GrowProps;
    pub use super::transitions::slide::Slide;
    pub use super::transitions::slide::SlideProps;
    pub use super::transitions::zoom::Zoom;
    pub use super::transitions::zoom::ZoomProps;
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
