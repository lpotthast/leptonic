use leptos::prelude::*;

use crate::{
    Out,
    atoms::switch::{Switch as SwitchAtom, SwitchProps as SwitchAtomProps},
    components::icon::Icon,
    utils::{classes::Classes, styles::Styles},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SwitchSize {
    Small,
    #[default]
    Normal,
    Big,
}

impl SwitchSize {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Normal => "normal",
            Self::Big => "big",
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SwitchVariant {
    /// The knob slides between off and on.
    #[default]
    Sliding,
    /// A single knob, showing the current icon.
    Stationary,
}

impl SwitchVariant {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sliding => "sliding",
            Self::Stationary => "stationary",
        }
    }
}

/// Icons shown in the switch's knob while off and on.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SwitchIcons {
    pub off: icondata::Icon,
    pub on: icondata::Icon,
}

/// A switch with its label (the children).
///
/// Its selection starts at `default_selected` and is reported through `on_change`; or it is
/// `is_selected`, and changes go to `set_selected` (e.g. both an `RwSignal<bool>`).
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
#[component]
pub fn Switch(
    #[prop(optional)] default_selected: bool,
    #[prop(into, optional)] on_change: Option<Callback<bool>>,
    /// The selection (controlled): a value or any signal.
    #[prop(into, optional)]
    is_selected: Option<Signal<bool>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_selected: Option<Out<bool>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] name: Option<String>,
    /// The value submitted with the form while on.
    #[prop(into, optional)]
    form_value: Option<String>,
    /// The accessible name, when there is no visible label.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(optional)] size: SwitchSize,
    #[prop(optional)] variant: SwitchVariant,
    #[prop(into, optional)] icons: Option<SwitchIcons>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let content = move || {
        view! {
            <span
                class="leptonic-switch-track"
                data-size=size.as_str()
                data-variant=variant.as_str()
                aria-hidden="true"
            >
                <span class="leptonic-switch-knob">
                    {icons.map(|icons| view! {
                        <Icon icon=icons.off classes="leptonic-switch-off-icon" />
                        <Icon icon=icons.on classes="leptonic-switch-on-icon" />
                    })}
                </span>
            </span>
            {children.map(|children| view! { <span class="leptonic-switch-label">{children()}</span> })}
        }
        .into_any()
    };
    // Built from the props struct: the view macro can't forward `Option` props.
    SwitchAtom(SwitchAtomProps {
        default_selected,
        on_change,
        is_selected,
        set_selected,
        is_disabled,
        is_read_only,
        is_required: Signal::stored(false),
        is_invalid: Signal::stored(false),
        validate: None,
        validation_behavior: None,
        name,
        form_value,
        form: None,
        id: None,
        aria_label,
        aria_labelledby: None,
        aria_describedby: None,
        auto_focus: false,
        on_focus_change: None,
        classes: classes.add("leptonic-switch"),
        styles,
        children: Some(Box::new(content)),
    })
}
