use std::sync::{Arc, RwLock};

use leptos::*;
use leptos_icons::*;
use tracing::warn;
use uuid::Uuid;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OnOpen {
    DoNothing,
    CloseOthers,
}

impl Default for OnOpen {
    fn default() -> Self {
        Self::DoNothing
    }
}

// TODO: Debug
#[derive(Clone)]
pub struct CollapsiblesContext {
    pub default_on_open: OnOpen,
    pub collapsibles: Arc<RwLock<Vec<CollapsibleContext>>>,
}

impl CollapsiblesContext {
    pub fn register(&mut self, ctx: CollapsibleContext) {
        let mut vec = self.collapsibles.write().unwrap();
        vec.push(ctx);
        drop(vec);
    }

    pub fn collapsible_changed(&self, id: Uuid, on_open: Option<OnOpen>, new_state: bool) {
        //debug!("Collapsibles:: collapsible_changed:: {id} {new_state}");
        match on_open.unwrap_or(self.default_on_open) {
            OnOpen::DoNothing => (),
            OnOpen::CloseOthers => {
                if new_state == true {
                    let vec = self.collapsibles.read().unwrap();
                    for ctx in vec.iter() {
                        if ctx.id != id {
                            ctx.set_show.update(move |it| *it = false);
                        }
                    }
                }
            }
        }
    }
}

// TODO: Debug
#[derive(Clone)]
pub struct CollapsibleContext {
    pub id: Uuid,
    pub show: ReadSignal<bool>,
    pub set_show: WriteSignal<bool>,
    pub on_open: Option<OnOpen>,
    pub parent: Option<CollapsiblesContext>,
}

impl CollapsibleContext {
    pub fn toggle(&self) {
        self.set_show.update(|it| *it = !*it);
        if let Some(parent) = &self.parent {
            parent.collapsible_changed(self.id, self.on_open, self.show.get())
        }
    }
}

#[component]
pub fn Collapsibles(cx: Scope, default_on_open: OnOpen, children: Children) -> impl IntoView {
    provide_context(
        cx,
        CollapsiblesContext {
            default_on_open,
            collapsibles: Arc::new(RwLock::new(vec![])),
        },
    );
    view! { cx,
        <leptonic-collapsibles>
            { children(cx) }
        </leptonic-collapsibles>
    }
}

#[component]
pub fn Collapsible<H, B>(
    cx: Scope,
    /// Whether this collapsible should initially be opened.
    #[prop(optional, default = false)]
    open: bool,
    #[prop(optional)] on_open: Option<OnOpen>,
    header: H,
    body: B,
) -> impl IntoView
where
    H: IntoView + 'static,
    B: IntoView + 'static,
{
    let id = Uuid::new_v4();
    let id_str = id.to_string();

    let (show, set_show) = create_signal(cx, open);

    let parent = use_context::<CollapsiblesContext>(cx);

    if parent.is_none() && on_open.is_some() {
        warn!("Collapsible {id}: Setting on_open on a Collapsible when that collapsible is not a Child of a Collapsibles parent element is pointless. Remove the argument or wrap this Collapsible in a Collapsibles.");
    }

    let ctx = CollapsibleContext {
        id,
        show,
        set_show,
        on_open,
        parent: parent.clone(),
    };

    if let Some(mut parent) = parent {
        parent.register(ctx.clone());
    };
    provide_context(cx, ctx);

    view! { cx,
        <leptonic-collapsible id=id_str>
            <CollapsibleHeaderWrapper>
                <CollapsibleHeader>
                    { header }
                </CollapsibleHeader>
                {move || match show.get() {
                    true => view! {cx, <Icon icon=BsIcon::BsCaretUpFill/>},
                    false => view! {cx, <Icon icon=BsIcon::BsCaretDownFill/>}
                }}
            </CollapsibleHeaderWrapper>
            <CollapsibleBody>
                { body }
            </CollapsibleBody>
        </leptonic-collapsible>
    }
}

#[component]
pub fn CollapsibleHeaderWrapper(cx: Scope, children: Children) -> impl IntoView {
    let collapsible_ctx = use_context::<CollapsibleContext>(cx)
        .expect("A CollapsibleHeader musst be placed inside a Collapsible component.");

    view! { cx,
        <leptonic-collapsible-header-wrapper on:click=move |_| collapsible_ctx.toggle()>
            { children(cx) }
        </leptonic-collapsible-header-wrapper>
    }
}

#[component]
pub fn CollapsibleHeader(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <leptonic-collapsible-header>
            { children(cx) }
        </leptonic-collapsible-header>
    }
}

#[component]
pub fn CollapsibleBody(cx: Scope, children: Children) -> impl IntoView {
    let collapsible_ctx = use_context::<CollapsibleContext>(cx)
        .expect("A CollapsibleHeader musst be placed inside a Collapsible component.");

    view! { cx,
        <leptonic-collapsible-body class:show=collapsible_ctx.show>
            { children(cx) }
        </leptonic-collapsible-body>
    }
}
