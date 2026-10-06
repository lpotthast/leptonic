use std::borrow::Cow;

use leptos::prelude::*;

use crate::{
    Language,
    utils::{
        classes::Classes, key::KeyboardKey, styles::Styles, visually_hidden::visually_hidden_styles,
    },
};

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn KbdKey(
    key: KeyboardKey,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let display = key.display(Language::En).to_owned();
    // Glyphs and abbreviations are shown, their names read.
    let content = match key.spoken_name() {
        Some(name) => view! {
            <span aria-hidden="true">{display}</span>
            <span style=visually_hidden_styles()>{name}</span>
        }
        .into_any(),
        None => display.into_any(),
    };
    view! { <kbd class=classes.add("leptonic-kbd-key") style=styles>{content}</kbd> }
}

#[component]
pub fn KbdConcatenate(
    #[prop(into, optional)] with: Option<Cow<'static, str>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    view! { <span class=classes.add("leptonic-kbd-concatenate") style=styles>{with.unwrap_or(Cow::Borrowed("+"))}</span> }
}

#[component]
pub fn KbdShortcutRoot(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! { <kbd class=classes.add("leptonic-kbd-shortcut") style=styles>{children()}</kbd> }
}

#[component]
pub fn KbdShortcut<const N: usize>(
    keys: [KeyboardKey; N],
    #[prop(into, optional)] concatenate_with: Option<Cow<'static, str>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let concatenate_with = concatenate_with.unwrap_or(Cow::Borrowed("+"));
    view! {
        <KbdShortcutRoot classes=classes styles=styles>
            {keys
                .into_iter()
                .enumerate()
                .map(|(i, key)| {
                    view! {
                        <KbdKey key=key />
                        {if i == N - 1 {
                            ().into_any()
                        } else {
                            view! { <KbdConcatenate with=concatenate_with.clone() /> }.into_any()
                        }}
                    }
                })
                .collect_view()}
        </KbdShortcutRoot>
    }
}
