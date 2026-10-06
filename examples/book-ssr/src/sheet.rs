use leptonic::{
    atoms::prelude::{Dialog, ModalBackdrop, ModalContent},
    components::prelude::{Button, ButtonVariant, Icon},
    prelude::icondata,
};
use leptos::prelude::*;

/// The side of the screen a [`Sheet`] slides in from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SheetSide {
    Left,
    Right,
}

impl SheetSide {
    const fn class(self) -> &'static str {
        match self {
            Self::Left => "book-sheet-left",
            Self::Right => "book-sheet-right",
        }
    }
}

/// A menu panel covering the page on small screens, built on leptonic's modal atoms: Escape, a click on the backdrop
/// and the close button close it; focus stays inside while it is open and returns to the trigger afterwards; the page
/// behind it neither scrolls nor is reachable by screen readers.
#[component]
pub fn Sheet(
    #[prop(into)] is_open: Signal<bool>,
    #[prop(into)] on_close: Callback<()>,
    /// The dialog's accessible name, e.g. `"Documentation"`.
    label: &'static str,
    side: SheetSide,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <ModalBackdrop
            is_open
            set_open=move |open: bool| if !open { on_close.run(()) }
            is_dismissable=true
            classes="book-sheet-backdrop">
            <ModalContent classes=["book-sheet", side.class()]>
                <Dialog aria_label=label classes="book-sheet-dialog">
                    <div class="book-sheet-header">
                        <Button
                            on_press=move |_| on_close.run(())
                            variant=ButtonVariant::Flat
                            classes="book-icon-button"
                            attr:aria-label="Close menu"
                        >
                            <Icon icon=icondata::BsXLg/>
                        </Button>
                    </div>
                    {children.with_value(|children| children())}
                </Dialog>
            </ModalContent>
        </ModalBackdrop>
    }
}
