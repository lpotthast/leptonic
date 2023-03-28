use leptos::*;
use uuid::Uuid;

use crate::prelude::*;

use super::icon::Bi;

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
    pub off: Bi,
    pub on: Bi,
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
    #[prop(optional)] icons: Option<ToggleIcons>,
) -> impl IntoView {
    let id = id.unwrap_or_else(|| Uuid::new_v4());
    view! { cx,
        <div class="crud-toggle-wrapper">
            <label
                id=id.to_string()
                class=format!("crud-toggle {}", size)
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
                                    true => view! {cx, <Icon variant=icons.on/> },
                                    false => view! {cx, <Icon variant=icons.off/> },
                                }}
                            </span>
                        })
                    }
                </span>
            </label>
        </div>
    }
}


/* 
use yew::prelude::*;
use yew_bootstrap_icons::v1_10_3::Bi;

use super::prelude::*;

pub enum Msg {
    Toggle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    Small,
    Normal,
    Big,
}

// TODO: This con be computed statically!
impl From<Size> for Classes {
    fn from(size: Size) -> Self {
        match size {
            Size::Small => classes!("small"),
            Size::Normal => classes!("normal"), // TODO: is this even necessary?
            Size::Big => classes!("big"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CrudToggleIcons {
    pub off: Bi,
    pub on: Bi,
}

#[derive(Debug, PartialEq, Properties)]
pub struct CrudToggleProps {
    #[prop_or(false)]
    pub state: bool,
    #[prop_or(Size::Normal)]
    pub size: Size,
    #[prop_or(false)]
    pub active: bool,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub icons: Option<CrudToggleIcons>,
    #[prop_or_default]
    pub on_toggle: Callback<bool>,
}

pub struct CrudToggle {
    state: bool,
}

impl Component for CrudToggle {
    type Message = Msg;
    type Properties = CrudToggleProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            state: ctx.props().state,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle => {
                self.state = !self.state;
                ctx.props().on_toggle.emit(self.state);
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.state = ctx.props().state;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="crud-toggle-wrapper">
                <label
                    class={classes!(
                        "crud-toggle",
                        ctx.props().size,
                        ctx.props().active.then(|| "active"),
                        ctx.props().disabled.then(|| "disabled")
                    )}
                    onclick={&ctx.link().callback(|_| Msg::Toggle)}
                >
                    <span class={classes!("slider", "round", self.state.then(|| "on"))}>
                        if let Some(icons) = &ctx.props().icons {
                            <span class={"icon-positioner"}>
                                if self.state {
                                    <CrudIcon variant={icons.on}/>
                                } else {
                                    <CrudIcon variant={icons.off}/>
                                }
                            </span>
                        }
                    </span>
                </label>
            </div>
        }
    }
}

*/