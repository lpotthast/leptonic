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
    pub const fn display(self, _lang: Language) -> &'static str {
        match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::F => "F",
            Self::G => "G",
            Self::H => "H",
            Self::I => "I",
            Self::J => "J",
            Self::K => "K",
            Self::L => "L",
            Self::M => "M",
            Self::N => "N",
            Self::O => "O",
            Self::P => "P",
            Self::Q => "Q",
            Self::R => "R",
            Self::S => "S",
            Self::T => "T",
            Self::U => "U",
            Self::V => "V",
            Self::W => "W",
            Self::X => "X",
            Self::Y => "Y",
            Self::Z => "Z",
            Self::N0 => "0",
            Self::N1 => "1",
            Self::N2 => "2",
            Self::N3 => "3",
            Self::N4 => "4",
            Self::N5 => "5",
            Self::N6 => "6",
            Self::N7 => "7",
            Self::N8 => "8",
            Self::N9 => "9",
            Self::F1 => "F1",
            Self::F2 => "F2",
            Self::F3 => "F3",
            Self::F4 => "F4",
            Self::F5 => "F5",
            Self::F6 => "F6",
            Self::F7 => "F7",
            Self::F8 => "F8",
            Self::F9 => "F9",
            Self::F10 => "F10",
            Self::F11 => "F11",
            Self::F12 => "F12",
            Self::ArrowUp => "↑",
            Self::ArrowRight => "→",
            Self::ArrowDown => "↓",
            Self::ArrowLeft => "←",
            Self::Plus => "+",
            Self::Star => "*",
            Self::Dash => "-",
            Self::Underscore => "_",
            Self::Slash => "/",
            Self::Backslash => "\\",
            Self::Dot => ".",
            Self::Comma => ",",
            Self::Colon => ":",
            Self::Semicolon => ";",
            Self::Hash => "#",
            Self::Escape => "Esc",
            Self::Enter => "Enter",
            Self::Backspace => "Backspace",
            Self::Control => "Ctrl",
            Self::Alt => "Alt",
            Self::Shift => "⇧",
            Self::CapsLock => "⇪",
            Self::Command => "⌘",
            Self::Option => "⌥",
            Self::Tab => "↹",
            Self::Tilde => "~",
            Self::Fn => "fn",
            Self::Custom(display) => display,
        }
    }
}
