//! Headless color swatch picker atoms: a list of colors to pick one from.
// Upstream: react-aria-components/src/ColorSwatchPicker.tsx @ 99e6102368

use std::collections::HashSet;

use leptos::{context::Provider, prelude::*};

use super::{
    color_picker::ColorPickerContext,
    color_swatch::ColorSwatch,
    listbox::{ListBox, ListBoxItem},
};
use crate::{
    Out,
    hooks::collections::{Key, ListLayout, Selection, SelectionMode, use_list_collection},
    utils::{
        ValueBinding,
        classes::Classes,
        color::{Color, ColorValue, RGB8},
        default_class::with_default_class,
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The colors are given to the picker (`colors`), which builds its collection from them; the
//   items render them (react-aria-components: items register their colors while rendering).
// - The value is split into `value` (a value or any signal) and `set_value` (an `Out`), plus
//   `default_value` and `on_change` (C4).
//
// =============================================================================

/// An item's key: the color as hex with alpha (react-aria-components' `hexa`).
fn color_key(color: Color) -> Key {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let alpha = (color.alpha.clamp(0.0, 1.0) * 255.0).round() as u8;
    Key::from(format!("#{:X}{alpha:02X}", color.to::<RGB8>()))
}

/// What the items of a [`ColorSwatchPicker`] need from it.
#[derive(Clone, Copy)]
struct PickerContext {
    colors: Signal<Vec<Color>>,
    /// The keys of the disabled items (they register themselves).
    disabled: RwSignal<HashSet<Key>>,
}

/// The color of the [`ColorSwatchPickerItem`] around a [`ColorSwatch`].
#[derive(Debug, Clone, Copy)]
pub struct ColorSwatchPickerItemContext(pub Color);

/// A list of colors to pick one from: render a [`ColorSwatchPickerItem`] per color (or use
/// [`ColorSwatchPickerItems`]), each with a [`ColorSwatch`] (which shows the item's color).
///
/// ```ignore
/// <ColorSwatchPicker colors=palette value=color set_value=color>
///     <ColorSwatchPickerItems />
/// </ColorSwatchPicker>
/// ```
///
/// Default class: `leptonic-ColorSwatchPicker`.
#[component]
pub fn ColorSwatchPicker<C: ColorValue>(
    /// The colors to pick from (distinct as hex).
    #[prop(into)]
    colors: Signal<Vec<C>>,
    /// The initially picked color.
    #[prop(optional)]
    default_value: Option<C>,
    /// The picked color (controlled): a value or any signal. Default: the `ColorPicker`'s
    /// around it.
    #[prop(into, optional)]
    value: Option<Signal<C>>,
    /// Receives the picked color: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<C>>,
    /// Called with the picked color.
    #[prop(into, optional)]
    on_change: Option<Callback<C>>,
    /// Names the picker. Default: "Color swatches", without `aria_labelledby`.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    /// How the swatches are laid out (for the arrow keys). Default: a grid.
    #[prop(optional)]
    layout: Option<ListLayout>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ColorSwatchPicker", classes);
    let collection = use_list_collection(
        colors,
        |color: &C| color_key((*color).into()),
        |color: &C| color.color_name(),
    );
    let (binding, on_change) = ValueBinding::from_state_props(value, set_value, on_change);
    let binding = binding.or_else(ColorPickerContext::binding::<C>);
    // Without app state, the picker owns the picked color.
    let owned = RwSignal::new(default_value);
    let picked = Signal::derive(move || match binding {
        Some(binding) => Some(binding.value.get()),
        None => owned.get(),
    });
    let set_picked = Callback::new(move |color: C| {
        match binding {
            Some(binding) => binding.set(color),
            None => owned.set(Some(color)),
        }
        if let Some(on_change) = on_change {
            on_change.run(color);
        }
    });
    let selection =
        Signal::derive(move || Selection::keys(picked.get().map(|color| color_key(color.into()))));
    let set_selection = Callback::new(move |selection: Selection| {
        // Single selection: the one key ("all" can't occur).
        let Selection::Keys(keys) = selection else {
            return;
        };
        let key = keys.iter().next().cloned();
        if let Some(color) = key.and_then(|key| {
            colors.with_untracked(|colors| {
                colors
                    .iter()
                    .find(|c| color_key((**c).into()) == key)
                    .copied()
            })
        }) {
            set_picked.run(color);
        }
    });
    let has_labelledby = aria_labelledby.is_some();
    let aria_label = MaybeProp::derive(move || {
        aria_label
            .get()
            .or_else(|| (!has_labelledby).then(|| "Color swatches".to_owned()))
    });
    let disabled = RwSignal::new(HashSet::new());
    let context = PickerContext {
        colors: Signal::derive(move || colors.get().into_iter().map(Into::into).collect()),
        disabled,
    };

    view! {
        <ListBox
            collection=collection
            selection_mode=SelectionMode::Single
            selection=selection
            set_selection=set_selection
            disallow_empty_selection=true
            disabled_keys=Signal::derive(move || disabled.get())
            aria_label=aria_label
            nostrip:aria_labelledby=aria_labelledby
            layout=layout.unwrap_or(ListLayout::Grid)
            classes=classes
            styles=styles
        >
            <Provider value=context>{children()}</Provider>
        </ListBox>
    }
}

/// A color of the [`ColorSwatchPicker`] around it: an option (`ListBoxItem`) named after the
/// color; put a [`ColorSwatch`] in it (which shows this color).
///
/// Data attributes: those of `ListBoxItem` (`data-selected`, `data-disabled`, ...).
///
/// Default class: `leptonic-ColorSwatchPickerItem`.
#[component]
pub fn ColorSwatchPickerItem(
    /// The color, one of the picker's `colors`: any color value.
    #[prop(into)]
    color: Color,
    /// Whether the color can't be picked.
    #[prop(into, optional)]
    is_disabled: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ColorSwatchPickerItem", classes);
    let key = color_key(color);
    if let Some(PickerContext { disabled, .. }) = use_context::<PickerContext>() {
        // Registered while rendering (so the server renders the item disabled too), then kept in
        // sync.
        if is_disabled.get_untracked() {
            disabled.update(|keys| {
                keys.insert(key.clone());
            });
        }
        let registered = key.clone();
        Effect::new(move |_| {
            let is_disabled = is_disabled.get();
            disabled.update(|keys| {
                if is_disabled {
                    keys.insert(registered.clone());
                } else {
                    keys.remove(&registered);
                }
            });
        });
        let unregistered = key.clone();
        on_cleanup(move || {
            disabled.try_update(|keys| keys.remove(&unregistered));
        });
    }
    view! {
        <ListBoxItem key=key classes=classes styles=styles>
            <Provider value=ColorSwatchPickerItemContext(color)>{children()}</Provider>
        </ListBoxItem>
    }
}

/// A [`ColorSwatchPickerItem`] for each color of the [`ColorSwatchPicker`] around it, showing a
/// [`ColorSwatch`] of it, or rendering `children` (which can show it with a `ColorSwatch`).
#[component]
pub fn ColorSwatchPickerItems(
    /// The items' classes.
    #[prop(into, optional)]
    classes: Classes,
    /// Renders an item's content. Default: a `ColorSwatch`.
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    let PickerContext { colors, .. } = expect_context::<PickerContext>();
    view! {
        <For
            each=move || colors.get()
            key=|color| color_key(*color)
            children=move |color| {
                let content = children.clone();
                view! {
                    <ColorSwatchPickerItem color=color classes=classes.clone()>
                        {match &content {
                            Some(content) => content().into_any(),
                            None => view! { <ColorSwatch /> }.into_any(),
                        }}
                    </ColorSwatchPickerItem>
                }
            }
        />
    }
}
