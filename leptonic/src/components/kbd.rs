use std::borrow::Cow;

use leptos::prelude::*;

use crate::{
    Language,
    utils::{classes::Classes, key::Key, styles::Styles},
};

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn KbdKey(
    key: Key,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let display = key.display(Language::En).to_owned();
    view! { <kbd class=classes.add("leptonic-kbd-key") style=styles>{display}</kbd> }
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
    keys: [Key; N],
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
