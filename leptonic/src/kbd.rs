use std::borrow::Cow;

use leptos::*;
use strum::EnumIter;

use crate::Language;

#[component]
pub fn KbdKey(key: Key) -> impl IntoView {
    view! {
        <leptonic-kbd-key>
            { key.display(Language::En) }
        </leptonic-kbd-key>
    }
}

#[component]
pub fn KbdConcatenate(#[prop(into, optional)] with: Option<Cow<'static, str>>) -> impl IntoView {
    view! {
        <leptonic-kbd-concatenate>
            { with.unwrap_or(Cow::Borrowed("+")) }
        </leptonic-kbd-concatenate>
    }
}

#[component]
pub fn KbdShortcutRoot(children: Children) -> impl IntoView {
    view! {
        <leptonic-kbd-shortcut>
            {children()}
        </leptonic-kbd-shortcut>
    }
}

#[component]
pub fn KbdShortcut<const N: usize>(
    keys: [Key; N],
    #[prop(into, optional)] concatenate_with: Option<Cow<'static, str>>,
) -> impl IntoView {
    let concatenate_with = concatenate_with.unwrap_or(Cow::Borrowed("+"));
    view! {
        <KbdShortcutRoot>
            { keys.into_iter().enumerate().map(|(i, key)| view! {
                <KbdKey key=key/>
                { match i == N - 1 {
                    true => ().into_view(),
                    false => view! { <KbdConcatenate with=concatenate_with.clone()/>}.into_view(),
                }}
            }).collect_view() }
        </KbdShortcutRoot>
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    ArrowUp,
    ArrowRight,
    ArrowDown,
    ArrowLeft,
    Plus,
    Star,
    Dash,
    Underscore,
    Slash,
    Backslash,
    Dot,
    Comma,
    Colon,
    Semicolon,
    Hash,
    Escape,
    Enter,
    Backspace,
    Control,
    Alt,
    Shift,
    CapsLock,
    Command,
    Option,
    Tab,
    Tilde,
    Fn,
    Custom(&'static str),
}

impl Key {
    pub fn display(self, _lang: Language) -> &'static str {
        match self {
            Key::A => "A",
            Key::B => "B",
            Key::C => "C",
            Key::D => "D",
            Key::E => "E",
            Key::F => "F",
            Key::G => "G",
            Key::H => "H",
            Key::I => "I",
            Key::J => "J",
            Key::K => "K",
            Key::L => "L",
            Key::M => "M",
            Key::N => "N",
            Key::O => "O",
            Key::P => "P",
            Key::Q => "Q",
            Key::R => "R",
            Key::S => "S",
            Key::T => "T",
            Key::U => "U",
            Key::V => "V",
            Key::W => "W",
            Key::X => "X",
            Key::Y => "Y",
            Key::Z => "Z",
            Key::N0 => "0",
            Key::N1 => "1",
            Key::N2 => "2",
            Key::N3 => "3",
            Key::N4 => "4",
            Key::N5 => "5",
            Key::N6 => "6",
            Key::N7 => "7",
            Key::N8 => "8",
            Key::N9 => "9",
            Key::F1 => "F1",
            Key::F2 => "F2",
            Key::F3 => "F3",
            Key::F4 => "F4",
            Key::F5 => "F5",
            Key::F6 => "F6",
            Key::F7 => "F7",
            Key::F8 => "F8",
            Key::F9 => "F9",
            Key::F10 => "F10",
            Key::F11 => "F11",
            Key::F12 => "F12",
            Key::ArrowUp => "↑",
            Key::ArrowRight => "→",
            Key::ArrowDown => "↓",
            Key::ArrowLeft => "←",
            Key::Plus => "+",
            Key::Star => "*",
            Key::Dash => "-",
            Key::Underscore => "_",
            Key::Slash => "/",
            Key::Backslash => "\\",
            Key::Dot => ".",
            Key::Comma => ",",
            Key::Colon => ":",
            Key::Semicolon => ";",
            Key::Hash => "#",
            Key::Escape => "Esc",
            Key::Enter => "Enter",
            Key::Backspace => "Backspace",
            Key::Control => "Ctrl",
            Key::Alt => "Alt",
            Key::Shift => "⇧",
            Key::CapsLock => "⇪",
            Key::Command => "⌘",
            Key::Option => "⌥",
            Key::Tab => "↹",
            Key::Tilde => "~",
            Key::Fn => "fn",
            Key::Custom(display) => display,
        }
    }
}
