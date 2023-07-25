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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ToggleIcons {
    pub off: leptos_icons::Icon,
    pub on: leptos_icons::Icon,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ToggleVariant {
    #[default]
    Sliding,
    Stationary,
}

impl ToggleVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            ToggleVariant::Sliding => "sliding",
            ToggleVariant::Stationary => "stationary",
        }
    }
}

#[component]
pub fn Toggle<S>(
    cx: Scope,
    #[prop(into)] state: MaybeSignal<bool>,
    on_toggle: S,
    #[prop(into, optional)] active: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] disabled: OptionalMaybeSignal<bool>,
    #[prop(optional)] id: Option<Uuid>,
    /// Sets the `class` attribute on the underlying `<Toggle>` tag, making it easier to style.
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(optional)] size: ToggleSize,
    #[prop(optional)] variant: ToggleVariant,
    #[prop(into, optional)] icons: Option<ToggleIcons>,
) -> impl IntoView
where
    S: Fn(bool) + 'static,
{
    let id = id.unwrap_or_else(|| Uuid::new_v4());

    let class = match class {
        Some(attr) => attr.into_attribute_boxed(cx),
        None => Attribute::String(std::borrow::Cow::Borrowed("leptonic-toggle-wrapper")),
    };

    view! { cx,
        <div class=class style=style>
            <label
                id=id.to_string()
                class=format!("leptonic-toggle {}", size)
                class:active=move || active.0.as_ref().map(|it| it.get()).unwrap_or(true)
                class:disabled=move || disabled.0.as_ref().map(|it| it.get()).unwrap_or(false)
                variant=variant.as_str()
                on:click=move |_| (on_toggle)(!state.get())
            >
                <span class="slider round" class:on=move || state.get()>
                    {
                        move || icons.as_ref().map(|icons| {
                            let off_icon = icons.off;
                            let on_icon = icons.on;
                            view! { cx,
                                <span class="icon-positioner">
                                    <Show when=move || state.get() fallback=move |cx| view! {cx, <Icon icon=off_icon/> }>
                                        <Icon icon=on_icon/>
                                    </Show>
                                </span>
                            }
                        })
                    }
                </span>
            </label>
        </div>
    }
}
