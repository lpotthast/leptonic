use std::borrow::Cow;

use leptos::prelude::*;

use crate::{utils::key::Key, Language};

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn KbdKey(key: Key) -> impl IntoView {
    let display = key.display(Language::En).to_owned();
    view! { <kbd class="leptonic-kbd-key">{display}</kbd> }
}

#[component]
pub fn KbdConcatenate(#[prop(into, optional)] with: Option<Cow<'static, str>>) -> impl IntoView {
    view! { <span class="leptonic-kbd-concatenate">{with.unwrap_or(Cow::Borrowed("+"))}</span> }
}

#[component]
pub fn KbdShortcutRoot(children: Children) -> impl IntoView {
    view! { <kbd class="leptonic-kbd-shortcut">{children()}</kbd> }
}

#[component]
pub fn KbdShortcut<const N: usize>(
    keys: [Key; N],
    #[prop(into, optional)] concatenate_with: Option<Cow<'static, str>>,
) -> impl IntoView {
    let concatenate_with = concatenate_with.unwrap_or(Cow::Borrowed("+"));
    view! {
        <KbdShortcutRoot>
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
