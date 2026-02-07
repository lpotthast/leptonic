use leptos::prelude::*;

use crate::{components::icon::Icon, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleSize {
    Small,
    #[default]
    Normal,
    Big,
}

impl ToggleSize {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Normal => "normal",
            Self::Big => "big",
        }
    }
}

impl std::fmt::Display for ToggleSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ToggleIcons {
    pub off: icondata::Icon,
    pub on: icondata::Icon,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ToggleVariant {
    #[default]
    Sliding,
    Stationary,
}

impl ToggleVariant {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Sliding => "sliding",
            Self::Stationary => "stationary",
        }
    }
}

// TODO: Given id was previously spread onto inner leptonic-toggle, it now goes to the outer wrapper. Is this a problem?
#[component]
pub fn Toggle(
    #[prop(into)] state: Signal<bool>,
    #[prop(into, optional)] set_state: Option<Out<bool>>,
    #[prop(into, optional)] active: Option<Signal<bool>>,
    #[prop(into, optional)] disabled: Option<Signal<bool>>,
    #[prop(optional)] size: ToggleSize,
    #[prop(optional)] variant: ToggleVariant,
    #[prop(into, optional)] icons: Option<ToggleIcons>,
) -> impl IntoView {
    view! {
        <leptonic-toggle-wrapper>
            <leptonic-toggle
                class:active=move || active.get().unwrap_or(true)
                class:disabled=move || disabled.get().unwrap_or(false)
                data-size=size.as_str()
                data-variant=variant.as_str()
                on:click=move |_| {
                    if let Some(set) = &set_state {
                        set.set(!state.get_untracked())
                    }
                }
            >
                <span class="slider round" class:on=move || state.get()>
                    {move || {
                        icons
                            .as_ref()
                            .map(|icons| {
                                let off_icon = icons.off;
                                let on_icon = icons.on;
                                view! {
                                    <span class="icon-positioner">
                                        <Icon
                                            icon=off_icon
                                            attr:style=move || {
                                                if state.get() {
                                                    "display: none"
                                                } else {
                                                    "display: inherit"
                                                }
                                            }
                                        />
                                        <Icon
                                            icon=on_icon
                                            attr:style=move || {
                                                if state.get() {
                                                    "display: inherit"
                                                } else {
                                                    "display: none"
                                                }
                                            }
                                        />
                                    </span>
                                }
                            })
                    }}
                </span>
            </leptonic-toggle>
        </leptonic-toggle-wrapper>
    }
}
