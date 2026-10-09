use leptonic::{
    KeyboardKey, KeyboardShortcuts, Shortcut, atoms, focus_safely,
    hooks::interactions::{UseGlobalShortcutsInput, use_global_shortcuts},
};
use leptos::prelude::*;
use leptos_use::use_document;

/// Focuses the message field. A bare key: only while the user isn't typing.
const FOCUS_MESSAGE: Shortcut = Shortcut::new(KeyboardKey::Slash);
/// Sends the message: Control + Enter (Command + Enter on Apple devices), also while typing.
const SEND: Shortcut = Shortcut::new(KeyboardKey::Enter).primary();

/// The id of the message field, so that the shortcut finds it.
const MESSAGE_FIELD_ID: &str = "demo-global-shortcuts-message";

#[component]
pub fn GlobalShortcutsDemo() -> impl IntoView {
    let message = RwSignal::new(String::new());
    let sent = RwSignal::new(0u32);
    let last_sent = RwSignal::new(None::<String>);

    // `false` (not handled) while there is nothing to send: the key press then does what it
    // would do without the shortcut.
    let send = move || {
        let text = message.get_untracked();
        if text.trim().is_empty() {
            return false;
        }
        last_sent.set(Some(text));
        sent.update(|sent| *sent += 1);
        message.set(String::new());
        true
    };

    use_global_shortcuts(UseGlobalShortcutsInput {
        anywhere: KeyboardShortcuts::new().on(SEND, move |_| send()),
        outside_text_fields: KeyboardShortcuts::new().on(FOCUS_MESSAGE, |_| {
            if let Some(field) = use_document()
                .as_ref()
                .and_then(|document| document.get_element_by_id(MESSAGE_FIELD_ID))
            {
                focus_safely(&field);
            }
        }),
    });

    view! {
        <p>
            "Press "<atoms::kbd::ShortcutKeys shortcut=FOCUS_MESSAGE classes="demo-shortcut-keys"/>
            " anywhere on this page to write a message, "
            <atoms::kbd::ShortcutKeys shortcut=SEND classes="demo-shortcut-keys"/>" to send it."
        </p>
        <div class="demo-message-row">
            <atoms::text_field::TextField id=MESSAGE_FIELD_ID value=message set_value=message classes="demo-field">
                <atoms::field::Label classes="demo-field-label">"Message"</atoms::field::Label>
                <atoms::input::Input classes="demo-atom-input"/>
            </atoms::text_field::TextField>
            <atoms::button::Button on_press=move |_| { send(); } classes="demo-btn">"Send"</atoms::button::Button>
        </div>
        <p class="demo-status">
            {move || match (sent.get(), last_sent.get()) {
                (_, None) => "Nothing sent yet.".to_owned(),
                (1, Some(text)) => format!("1 message sent, the last: \u{201c}{text}\u{201d}."),
                (count, Some(text)) => format!("{count} messages sent, the last: \u{201c}{text}\u{201d}."),
            }}
        </p>
    }
}
