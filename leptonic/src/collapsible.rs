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

#[component]
pub fn Collapsible(
    /// Whether this collapsible should initially be opened.
    #[prop(optional, default = false)] open: bool,
    #[prop(optional)] on_open: Option<OnOpen>,
    #[prop(into)] header: Producer<View>,
    #[prop(into)] body: Producer<View>,
) -> impl IntoView {
    let id = Uuid::new_v4();
    let id_str = id.to_string();

    let (show, set_show) = create_signal(open);

    let parent = use_context::<CollapsiblesContext>();

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
    provide_context(ctx);

    view! {
        <leptonic-collapsible id=id_str>
            <CollapsibleHeaderWrapper>
                <CollapsibleHeader>
                    { header.produce() }
                </CollapsibleHeader>
                {move || match show.get() {
                    true => view! { <Icon icon=BsIcon::BsCaretUpFill/>},
                    false => view! { <Icon icon=BsIcon::BsCaretDownFill/>}
                }}
            </CollapsibleHeaderWrapper>
            <CollapsibleBody>
                { body.produce() }
            </CollapsibleBody>
        </leptonic-collapsible>
    }
}

#[component]
pub fn CollapsibleHeaderWrapper(children: Children) -> impl IntoView {
    let collapsible_ctx = use_context::<CollapsibleContext>()
        .expect("A CollapsibleHeader musst be placed inside a Collapsible component.");

    view! {
        <leptonic-collapsible-header-wrapper on:click=move |_| collapsible_ctx.toggle()>
            { children() }
        </leptonic-collapsible-header-wrapper>
    }
}

#[component]
pub fn CollapsibleHeader(children: Children) -> impl IntoView {
    view! {
        <leptonic-collapsible-header>
            { children() }
        </leptonic-collapsible-header>
    }
}

#[component]
pub fn CollapsibleBody(children: Children) -> impl IntoView {
    let collapsible_ctx = use_context::<CollapsibleContext>()
        .expect("A CollapsibleHeader musst be placed inside a Collapsible component.");

    view! {
        <leptonic-collapsible-body class:show=move || collapsible_ctx.show.get()>
            { children() }
        </leptonic-collapsible-body>
    }
}
