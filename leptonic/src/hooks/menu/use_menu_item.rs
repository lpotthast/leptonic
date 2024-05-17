use leptos::{expect_context, Callable, Callback, MaybeSignal, SignalGetUntracked};
use typed_builder::TypedBuilder;
use web_sys::FocusEvent;

use crate::{
    hooks::*,
    prelude::AriaHasPopup,
    state::{selection::SelectionMode, Key},
    utils::{
        aria::AriaRole, attributes::Attributes, event_handlers::EventHandlers,
        pointer_type::PointerType,
    },
    OptMaybeSignal,
};

use self::{focus::is_focus_visible, interactions::{use_hover::HoverResponder, use_press::PressResponder}};

use super::use_menu::UseMenuContext;

#[derive(Debug, TypedBuilder)]
pub struct UseMenuItemInput {
    pub(crate) key: Key, // TODO: Allow everything `IntoKey`?

    /// Whether this item should be disabled.
    #[builder(setter(into), default = false.into())]
    pub(crate) disabled: MaybeSignal<bool>,

    /// Whether the menu should close when this item is selected.
    #[builder(setter(into), default)]
    pub(crate) close_on_select: OptMaybeSignal<bool>,

    #[builder(setter(into), default)]
    pub(crate) aria_haspopup: AriaHasPopup,

    // use_focus props
    #[builder(default, setter(into, strip_option))]
    pub(crate) on_focus: Option<Callback<FocusEvent>>,
    #[builder(default, setter(into, strip_option))]
    pub(crate) on_blur: Option<Callback<FocusEvent>>,
    #[builder(default, setter(into, strip_option))]
    pub(crate) on_focus_change: Option<Callback<bool>>,
}

#[derive(Debug)]
pub struct UseMenuItemReturn {
    pub attrs: Attributes,
    pub handlers: EventHandlers,
}

/// Provides the behavior and accessibility implementation for an item in a menu.
/// See `useMenu` for more details about menus.
pub fn use_menu_item(
    input: UseMenuItemInput,
    press_responder: PressResponder,
    hover_responder: HoverResponder,
) -> UseMenuItemReturn {
    // TODO: Should we take this as a fn param instead?
    let menu_ctx = expect_context::<UseMenuContext>();

    let on_action = menu_ctx.on_action;
    let key = input.key;

    let is_trigger = input.aria_haspopup != AriaHasPopup::False;
    let role = AriaRole::MenuItem;

    let perform_action = move |e: PressEvent| {
        if is_trigger {
            return;
        }
        on_action.call(key.clone());

        // NOTE: react-aria performs specialized link handling at this point,
        // opening a property declared href through a custom `router` call.
        // TODO: Do we need that as well? How would menu items with `<a>` tags function?
        /*if e.target.is_some_and(|t| t instanceof HTMLAnchorElement) {
          router.open(e.target, e, item.props.href, item.props.routerOptions as RouterOptions);
        }*/
    };
    let perform_action_clone = perform_action.clone();

    press_responder.add_on_press_start(Callback::new(move |e: PressEvent| {
        // Immediately trigger this items action when using a keyboard.
        if e.pointer_type == PointerType::Keyboard {
            perform_action(e);
        }
    }));

    press_responder.add_on_press(Callback::new(move |e: PressEvent| {
        // We already handled keyboard interactions through on_press_start!
        if e.pointer_type == PointerType::Keyboard {
            return;
        }

        perform_action_clone(e);

        // TODO: "Closable menus": Can we extract this "closable" behavior?
        // Pressing a menu item should close by default in single selection mode but not multiple
        // selection mode, except if overridden by the close_on_select prop.
        if let Some(on_close) = menu_ctx.on_close {
            if !is_trigger
                && input
                    .close_on_select
                    .0
                    .map(|s| s.get_untracked())
                    .unwrap_or_else(|| {
                        menu_ctx.selection_manager.selection_mode() != SelectionMode::Multiple
                        // || menu_ctx.selection_manager.is_link(input.key) // TODO
                    })
            {
                on_close.call(());
            }
        }
    }));

    // Focus the menu and the current item whenever it is hovered.
    hover_responder.add_on_hover_start(Callback::new(move |e: HoverStartEvent| {
        if !is_focus_visible() {
            // state.selectionManager.setFocused(true);
            // state.selectionManager.setFocusedKey(key);
        }
    }));

    let UseFocusReturn {
        props:
            UseFocusProps {
                attrs: focus_attrs,
                handlers: focus_handlers,
            },
    } = use_focus(UseFocusInput {
        disabled: input.disabled,
        on_focus: input.on_focus,
        on_blur: input.on_blur,
        on_focus_change: input.on_focus_change,
    });

    let mut attrs = Attributes::new();
    let mut handlers = EventHandlers::builder().build();

    attrs = attrs.merge(focus_attrs);
    handlers = handlers.merge(focus_handlers);

    UseMenuItemReturn {
        attrs,
        handlers,
        //is_focused,
        //is_selected,
        //is_pressed,
        //is_disabled
    }
}
