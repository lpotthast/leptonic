use leptos::*;
use uuid::Uuid;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleSize {
    Small,
    Normal,
    Big,
}

impl ToggleSize {
    pub fn class_name(&self) -> &'static str {
        match self {
            ToggleSize::Small => "small",
            ToggleSize::Normal => "normal",
            ToggleSize::Big => "big",
        }
    }
}

impl std::fmt::Display for ToggleSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.class_name())
    }
}

impl Default for ToggleSize {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, PartialEq)]
pub struct ToggleIcons {
    pub off: leptos_icons::Icon,
    pub on: leptos_icons::Icon,
}

#[component]
pub fn Toggle(
    cx: Scope,
    #[prop(into)]
    on: Signal<bool>,
    #[prop(into)]
    set_on: WriteSignal<bool>,
    #[prop(optional)] active: Option<Active>,
    #[prop(optional)] disabled: Option<Disabled>,
    #[prop(optional)] id: Option<Uuid>,
    #[prop(optional)] size: ToggleSize,
    #[prop(into, optional)] icons: Option<ToggleIcons>,
) -> impl IntoView {
    let id = id.unwrap_or_else(|| Uuid::new_v4());
    view! { cx,
        <div class="leptonic-toggle-wrapper">
            <label
                id=id.to_string()
                class=format!("leptonic-toggle {}", size)
                class:active=move || active.map(|it| match it {
                    Active::Static(active) => active,
                    Active::Reactive(active) => active.get(),
                }).unwrap_or(true)
                class:disabled=move || disabled.map(|it| match it {
                    Disabled::Static(disabled) => disabled,
                    Disabled::Reactive(disabled) => disabled.get(),
                }).unwrap_or(false)
                on:click=move |_| set_on.update(|c| *c = !*c)
            >
                <span class="slider round" class:on=on>
                    {
                        move || icons.as_ref().map(|icons| view! { cx,
                            <span class="icon-positioner">
                                {match on.get() {
                                    true => view! {cx, <Icon icon=icons.on/> },
                                    false => view! {cx, <Icon icon=icons.off/> },
                                }}
                            </span>
                        })
                    }
                </span>
            </label>
        </div>
    }
}
