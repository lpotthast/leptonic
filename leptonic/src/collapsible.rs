use std::sync::{Arc, RwLock};

use leptos::*;
use leptos_icons::BsIcon;
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

#[derive(Debug, Clone)]
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
                if new_state {
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

#[derive(Debug, Clone)]
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
            parent.collapsible_changed(self.id, self.on_open, self.show.get());
        }
    }
}

#[component]
pub fn Collapsibles(default_on_open: OnOpen, children: Children) -> impl IntoView {
    provide_context(CollapsiblesContext {
        default_on_open,
        collapsibles: Arc::new(RwLock::new(vec![])),
    });
    view! {
        <leptonic-collapsibles>
            { children() }
        </leptonic-collapsibles>
    }
}

pub fn use_collapsible(open: bool, on_open: Option<OnOpen>) -> CollapsibleContext {
    let id = Uuid::new_v4();

    let (show, set_show) = create_signal(open);

    let mut parent = use_context::<CollapsiblesContext>();

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

    if let Some(parent) = parent.as_mut() {
        parent.register(ctx.clone());
    }
    provide_context(ctx.clone());

    ctx
}

#[component]
pub fn Collapsible(
    /// Whether this collapsible should initially be opened.
    #[prop(optional, default = false)]
    open: bool,
    #[prop(optional)] on_open: Option<OnOpen>,
    collapsible_header: CollapsibleHeader,
    collapsible_body: CollapsibleBody,
) -> impl IntoView {
    let collapsible = use_collapsible(open, on_open);
    let id_str = collapsible.id.to_string();
    view! {
        <leptonic-collapsible id=id_str>
            <CollapsibleHeaderInternal collapsible_header/>
            <CollapsibleBodyInternal collapsible_body/>
        </leptonic-collapsible>
    }
}

#[derive(Debug, Clone)]
pub struct CollapsibleHeaderWrapperContext {
    collapsible_ctx: CollapsibleContext,
}

/// # Panics
///
/// Will panic if not called under a `Collapsible` component.
pub fn use_collapsible_header() -> CollapsibleHeaderWrapperContext {
    CollapsibleHeaderWrapperContext {
        collapsible_ctx: use_context::<CollapsibleContext>()
            .expect("A CollapsibleHeader must be placed inside a Collapsible component."),
    }
}

#[slot]
pub struct CollapsibleHeader {
    children: Children,
    #[prop(into, optional)]
    class: Option<AttributeValue>,
}

#[component]
fn CollapsibleHeaderInternal(collapsible_header: CollapsibleHeader) -> impl IntoView {
    let ctx = use_collapsible_header();
    let ctx2 = use_collapsible_header();
    view! {
        <leptonic-collapsible-header-wrapper on:click=move |_| ctx.collapsible_ctx.toggle()>
            <leptonic-collapsible-header class=collapsible_header.class>
                { (collapsible_header.children)() }
            </leptonic-collapsible-header>

            { move ||  match ctx2.collapsible_ctx.show.get() {
                true => view! { <Icon icon=BsIcon::BsCaretUpFill/>}.into_view(),
                false => view! { <Icon icon=BsIcon::BsCaretDownFill/>}.into_view()
            } }
        </leptonic-collapsible-header-wrapper>
    }
}

#[slot]
pub struct CollapsibleBody {
    children: Children,
    #[prop(into, optional)]
    class: Option<AttributeValue>,
}

#[component]
fn CollapsibleBodyInternal(collapsible_body: CollapsibleBody) -> impl IntoView {
    let collapsible_ctx = use_context::<CollapsibleContext>()
        .expect("A CollapsibleHeader must be placed inside a Collapsible component.");

    view! {
        <leptonic-collapsible-body class=collapsible_body.class class:show=move || collapsible_ctx.show.get()>
            { (collapsible_body.children)() }
        </leptonic-collapsible-body>
    }
}
