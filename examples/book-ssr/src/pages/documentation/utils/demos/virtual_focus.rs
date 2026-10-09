use leptonic::{
    IntoAttrs, KeyboardKey, KeyboardShortcuts, Shortcut,
    hooks::{
        focus::{UseFocusInput, UseFocusReturn, use_focus},
        interactions::{UseKeyboardInput, UseKeyboardReturn, use_keyboard},
    },
    move_virtual_focus, use_id,
};
use leptos::{html, prelude::*};

const FRUITS: [&str; 4] = ["Apple", "Banana", "Cherry", "Mango"];

/// An option of the list. It reacts to the synthetic `focus` and `blur` events of virtual focus
/// as it would to real ones. (`use_focus` only reports the element that has DOM focus, so the
/// option listens to the events directly.)
#[component]
fn FruitOption(id: String, name: &'static str, log: RwSignal<Vec<String>>) -> impl IntoView {
    let (is_focused, set_focused) = signal(false);

    view! {
        <li
            id=id
            role="option"
            class="demo-virtual-option"
            data-focused=move || is_focused.get().then_some("")
            on:focus=move |_| {
                set_focused.set(true);
                log.update(|log| log.push(format!("{name} got focus")));
            }
            on:blur=move |_| {
                set_focused.set(false);
                log.update(|log| log.push(format!("{name} lost focus")));
            }
        >
            {name}
        </li>
    }
}

#[component]
pub fn VirtualFocusDemo() -> impl IntoView {
    let input_id = use_id("fruit-input");
    let list_id = use_id("fruit-list");
    let option_ids: Vec<String> = FRUITS.iter().map(|_| use_id("fruit")).collect();

    let input = NodeRef::<html::Input>::new();
    let list = NodeRef::<html::Ul>::new();
    // The index of the virtually focused option; `None` while the input itself is focused.
    let focused = RwSignal::new(None::<usize>);
    // The focus events of the last move.
    let log = RwSignal::new(Vec::<String>::new());

    let move_to = move |index: Option<usize>| {
        let target = match index {
            Some(index) => list
                .get_untracked()
                .and_then(|list| list.children().item(u32::try_from(index).ok()?)),
            None => input.get_untracked().map(web_sys::Element::from),
        };
        log.set(Vec::new());
        // Before `aria-activedescendant` changes: it tells `move_virtual_focus` which element
        // loses virtual focus.
        move_virtual_focus(target.as_ref());
        focused.set(index);
    };

    let count = FRUITS.len();
    let shortcuts = KeyboardShortcuts::new()
        .on(Shortcut::new(KeyboardKey::ArrowDown), move |_| {
            move_to(Some(focused.get_untracked().map_or(0, |i| (i + 1) % count)));
        })
        .on(Shortcut::new(KeyboardKey::ArrowUp), move |_| {
            move_to(Some(
                focused
                    .get_untracked()
                    .map_or(count - 1, |i| (i + count - 1) % count),
            ));
        })
        // Back to the input: it gets a synthetic `focus` event and shows its focus ring again.
        .on(Shortcut::new(KeyboardKey::Escape), move |_| {
            if focused.get_untracked().is_none() {
                return false;
            }
            move_to(None);
            true
        });
    let UseKeyboardReturn { props: keyboard } = use_keyboard(UseKeyboardInput {
        shortcuts: Some(shortcuts),
        allow_repeats: true,
        ..Default::default()
    });

    // The input has DOM focus throughout, but loses its ring while an option is focused virtually.
    let (is_input_focused, set_input_focused) = signal(false);
    let UseFocusReturn { props: focus } = use_focus(UseFocusInput {
        on_focus: Some(Callback::new(move |_| {
            log.update(|log| log.push("the input got focus".to_owned()));
        })),
        on_blur: Some(Callback::new(move |_| {
            log.update(|log| log.push("the input lost focus".to_owned()));
        })),
        on_focus_change: Some(Callback::new(move |is_focused| {
            set_input_focused.set(is_focused);
        })),
        ..Default::default()
    });

    let active_descendant = {
        let option_ids = option_ids.clone();
        move || focused.get().map(|i| option_ids[i].clone())
    };

    view! {
        <div class="demo-virtual-focus">
            <label for=input_id.clone()>"Fruit"</label>
            <input
                {..keyboard.into_attrs()}
                {..focus.into_attrs()}
                node_ref=input
                id=input_id
                class="demo-virtual-input"
                role="combobox"
                aria-expanded="true"
                aria-controls=list_id.clone()
                aria-activedescendant=active_descendant
                data-focused=move || is_input_focused.get().then_some("")
                autocomplete="off"
            />
            <ul node_ref=list id=list_id role="listbox" aria-label="Fruits" class="demo-virtual-list">
                {FRUITS
                    .into_iter()
                    .zip(option_ids)
                    .map(|(name, id)| view! { <FruitOption id=id name=name log=log/> })
                    .collect_view()}
            </ul>
        </div>
        <p class="demo-status">
            {move || {
                let current = focused.get().map_or("no option", |i| FRUITS[i]);
                let events = log.with(|log| if log.is_empty() { "none".to_owned() } else { log.join(", ") });
                format!("Virtually focused: {current}. Last events: {events}.")
            }}
        </p>
    }
}
