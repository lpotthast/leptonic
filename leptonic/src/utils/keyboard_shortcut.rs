// Upstream: react-aria/src/interactions/createKeyboardShortcutHandler.ts @ 99e6102368
// Upstream: react-aria/test/interactions/createKeyboardShortcutHandler.test.js @ 99e6102368
// Upstream: react-aria/test/interactions/useKeyboard.test.js @ 99e6102368
//! Keyboard shortcuts: a key plus an exact set of modifiers, mapped to handlers.
//!
//! This is leptonic's take on react-aria's `createKeyboardShortcutHandler`. React-aria describes
//! shortcuts as strings (`"Mod+Shift+z"`) that are parsed at runtime. Here, shortcuts are typed
//! values, built with `const` functions:
//!
//! ```
//! use leptonic::{KeyboardKey, Shortcut};
//!
//! const UNDO: Shortcut = Shortcut::new(KeyboardKey::Z).primary();
//! const REDO: Shortcut = Shortcut::new(KeyboardKey::Z).primary().shift();
//! const PAGE_UP: Shortcut = Shortcut::new(KeyboardKey::PageUp);
//! ```
//!
//! [`Shortcut::parse`] accepts react-aria's string syntax as well, for shortcuts that come from
//! configuration or user input.
//!
//! A shortcut matches a keyboard event when the key matches (letters and other keys of the
//! `Other` kind case-insensitively) and the *exact* set of modifiers is held.
//! `Shortcut::new(KeyboardKey::Z).primary()` does not match `Ctrl+Shift+Z`.
//
// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Shortcuts are typed values (`Shortcut`: a `KeyboardKey` and modifiers) bound with
//   `KeyboardShortcuts::on`, instead of a record of shortcut strings; `Shortcut::parse` takes
//   react-aria's string syntax.
// - A handler returns `()`, a `bool` or a `ShortcutOutcome` (react-aria: `void`, a boolean or an
//   object of `shouldPreventDefault`/`shouldContinuePropagation`).
//
// ## DIFFERENT BEHAVIOR
// - `Shortcut::parse` rejects a spec naming two keys (`"a+b"`; react-aria takes the last) and
//   accepts the `+` key (`"Mod++"`, which react-aria can't parse).
// - For a character without case, such as `/`, `?` or `7`, Shift is ignored unless the shortcut
//   requires it, as the keyboard layout decides whether typing the character takes Shift (`/` is
//   Shift+7 on German keyboards, `?` takes Shift on most). react-aria requires the exact
//   modifiers.
//
// =============================================================================

use std::{borrow::Cow, fmt, str::FromStr, sync::Arc};

use web_sys::KeyboardEvent;

use crate::{
    EventModifiers, Modifiers,
    utils::{aria::AriaKeyshortcuts, key::KeyboardKey, platform::device},
};

/// A key combination: one key plus the modifiers that must be held, and no others.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Shortcut {
    key: KeyboardKey,
    modifiers: Modifiers,
    /// The platform's primary modifier (Command on Apple platforms, Control elsewhere).
    primary: bool,
}

impl Shortcut {
    /// A shortcut for `key` without modifiers. Letters match in either case (`KeyboardKey::Z`
    /// matches `z` and `Z`, which Shift types).
    pub const fn new(key: KeyboardKey) -> Self {
        Self {
            key,
            modifiers: Modifiers {
                shift_key: false,
                ctrl_key: false,
                meta_key: false,
                alt_key: false,
            },
            primary: false,
        }
    }

    /// Also require Shift.
    #[must_use]
    pub const fn shift(mut self) -> Self {
        self.modifiers.shift_key = true;
        self
    }

    /// Also require Alt (Option on macOS).
    #[must_use]
    pub const fn alt(mut self) -> Self {
        self.modifiers.alt_key = true;
        self
    }

    /// Also require Control.
    #[must_use]
    pub const fn ctrl(mut self) -> Self {
        self.modifiers.ctrl_key = true;
        self
    }

    /// Also require Meta (Command on macOS, the Windows key on Windows).
    #[must_use]
    pub const fn meta(mut self) -> Self {
        self.modifiers.meta_key = true;
        self
    }

    /// Also require the platform's primary modifier: Command on Apple platforms, Control
    /// everywhere else. React-aria calls this `Mod`.
    #[must_use]
    pub const fn primary(mut self) -> Self {
        self.primary = true;
        self
    }

    /// Parse react-aria's shortcut syntax, e.g. `"Mod+Shift+z"`, `"Control+Alt+Enter"` or
    /// `"Escape"`. Modifier names (`Shift`, `Alt`, `Control`/`Ctrl`, `Meta`, `Mod`) and key names
    /// are case-insensitive, and modifiers may come in any order. The key aliases `Space`, `Esc`,
    /// `Del`, `Ins`, `Left`, `Right`, `Up` and `Down` are accepted. Exactly one non-modifier key
    /// is required.
    ///
    /// # Errors
    ///
    /// Returns an error if `spec` names no key or more than one.
    pub fn parse(spec: &str) -> Result<Self, InvalidShortcut> {
        let mut modifiers = Modifiers::default();
        let mut primary = false;
        let mut key: Option<&str> = None;
        // The `+` key itself: `"+"`, `"Mod++"`.
        let (modifier_part, plus_key) = if spec == "+" {
            ("", true)
        } else if let Some(modifier_part) = spec.strip_suffix("++") {
            (modifier_part, true)
        } else {
            (spec, false)
        };
        if plus_key {
            key = Some("+");
        }
        let parts = modifier_part
            .split('+')
            .filter(|part| !(plus_key && part.is_empty()));
        for part in parts {
            match part.to_ascii_lowercase().as_str() {
                "shift" => modifiers.shift_key = true,
                "alt" => modifiers.alt_key = true,
                "control" | "ctrl" => modifiers.ctrl_key = true,
                "meta" => modifiers.meta_key = true,
                "mod" => primary = true,
                _ if key.is_some() => return Err(InvalidShortcut(spec.to_owned())),
                _ => key = Some(part),
            }
        }
        match key {
            Some(key) if !key.is_empty() => Ok(Self {
                key: key_named(key),
                modifiers,
                primary,
            }),
            _ => Err(InvalidShortcut(spec.to_owned())),
        }
    }

    /// Whether `event` triggers this shortcut on the current platform.
    pub fn matches(&self, event: &KeyboardEvent) -> bool {
        self.matches_key(&event.key(), event.modifiers(), device::is_mac())
    }

    fn matches_key(&self, key: &str, pressed: Modifiers, is_mac: bool) -> bool {
        let required = Modifiers {
            ctrl_key: self.modifiers.ctrl_key || (self.primary && !is_mac),
            meta_key: self.modifiers.meta_key || (self.primary && is_mac),
            ..self.modifiers
        };
        // A character without case may take Shift to type, depending on the layout.
        let pressed = if !required.shift_key && is_uncased_character(key) {
            Modifiers {
                shift_key: false,
                ..pressed
            }
        } else {
            pressed
        };
        let Ok(event_key) = key.parse::<KeyboardKey>();
        let same_key = match (&self.key, &event_key) {
            (KeyboardKey::Other(own), KeyboardKey::Other(pressed)) => {
                own.to_lowercase() == pressed.to_lowercase()
            }
            (own, pressed) => own == pressed,
        };
        pressed == required && same_key
    }
}

/// The key a shortcut spec names: aliases resolved, named keys case-insensitively.
fn key_named(name: &str) -> KeyboardKey {
    let lower = name.to_ascii_lowercase();
    let name = match lower.as_str() {
        "space" => "Space",
        "esc" => "Escape",
        "del" => "Delete",
        "ins" => "Insert",
        "left" => "ArrowLeft",
        "right" => "ArrowRight",
        "up" => "ArrowUp",
        "down" => "ArrowDown",
        _ => name,
    };
    let Ok(key) = name.parse::<KeyboardKey>();
    if !matches!(key, KeyboardKey::Other(_)) {
        return key;
    }
    // Named keys (`escape`, `pageup`): their variant names are their `KeyboardEvent.key` names.
    KeyboardKey::known_keys()
        .find(|known| <&'static str>::from(known).eq_ignore_ascii_case(name))
        .unwrap_or(key)
}

/// The key's name in shortcut specs and `aria-keyshortcuts`: a character as on the key cap (`K`,
/// `/`), a named key by its `KeyboardEvent.key` name (`ArrowDown`, `PageUp`, `Space`).
fn key_name(key: &KeyboardKey) -> Cow<'static, str> {
    match key {
        KeyboardKey::Other(name) => name.clone(),
        key => {
            let display = key.display();
            let Ok(parsed) = display.parse::<KeyboardKey>();
            if parsed == *key {
                Cow::Owned(display.to_owned())
            } else {
                Cow::Borrowed(<&'static str>::from(key))
            }
        }
    }
}

/// Whether `key` is a single printable character without case (punctuation, digits, ...).
fn is_uncased_character(key: &str) -> bool {
    let mut chars = key.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => {
            !c.is_whitespace() && !c.is_control() && c.to_lowercase().eq(c.to_uppercase())
        }
        _ => false,
    }
}

impl Shortcut {
    /// The keys to show for this shortcut, modifiers first in the platform's order: on Apple
    /// platforms (`apple`) Control, Option, Shift, Command (the primary modifier), elsewhere
    /// Control (the primary modifier), Alt, Shift, Meta.
    pub fn keys(&self, apple: bool) -> Vec<KeyboardKey> {
        let Modifiers {
            shift_key,
            ctrl_key,
            meta_key,
            alt_key,
        } = self.modifiers;
        let mut keys = Vec::new();
        if apple {
            for (held, key) in [
                (ctrl_key, KeyboardKey::ControlSymbol),
                (alt_key, KeyboardKey::Option),
                (shift_key, KeyboardKey::Shift),
                (meta_key || self.primary, KeyboardKey::Command),
            ] {
                if held {
                    keys.push(key);
                }
            }
        } else {
            for (held, key) in [
                (ctrl_key || self.primary, KeyboardKey::Control),
                (alt_key, KeyboardKey::Alt),
                (shift_key, KeyboardKey::Shift),
                (meta_key, KeyboardKey::Meta),
            ] {
                if held {
                    keys.push(key);
                }
            }
        }
        keys.push(self.key.clone());
        keys
    }
}

impl FromStr for Shortcut {
    type Err = InvalidShortcut;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Shortcut {
    /// The shortcut as an `aria-keyshortcuts` value for the platform (`apple`: the primary
    /// modifier is Meta, else Control), e.g. `"Control+K"` or `"Shift+Meta+Z"`. Put it on the
    /// element the shortcut activates or focuses.
    pub fn to_aria_keyshortcuts(&self, apple: bool) -> AriaKeyshortcuts {
        let Modifiers {
            shift_key,
            ctrl_key,
            meta_key,
            alt_key,
        } = self.modifiers;
        let mut value = String::new();
        for (held, name) in [
            (ctrl_key || (self.primary && !apple), "Control"),
            (alt_key, "Alt"),
            (shift_key, "Shift"),
            (meta_key || (self.primary && apple), "Meta"),
        ] {
            if held {
                value.push_str(name);
                value.push('+');
            }
        }
        let name = key_name(&self.key);
        // A character: as on the key cap (letters in upper case).
        if name.chars().count() == 1 {
            value.push_str(&name.to_uppercase());
        } else {
            value.push_str(&name);
        }
        AriaKeyshortcuts::from_value(value)
    }
}

impl fmt::Display for Shortcut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (held, name) in [
            (self.primary, "Mod"),
            (self.modifiers.alt_key, "Alt"),
            (self.modifiers.ctrl_key, "Control"),
            (self.modifiers.meta_key, "Meta"),
            (self.modifiers.shift_key, "Shift"),
        ] {
            if held {
                write!(f, "{name}+")?;
            }
        }
        f.write_str(&key_name(&self.key))
    }
}

/// Error returned by [`Shortcut::parse`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidShortcut(String);

impl fmt::Display for InvalidShortcut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid keyboard shortcut {:?}: it must name exactly one non-modifier key (e.g. \"a\", \
             \"Enter\", \"ArrowDown\"), optionally combined with Shift, Alt, Control, Meta and Mod",
            self.0
        )
    }
}

impl std::error::Error for InvalidShortcut {}

/// What a shortcut handler did with the event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutOutcome {
    /// The shortcut was handled: prevent the browser default and stop propagation.
    Handled,
    /// The handler decided not to act: leave the event alone, as if no shortcut matched.
    Ignored,
    /// Explicit control over both aspects.
    Custom {
        prevent_default: bool,
        continue_propagation: bool,
    },
}

impl ShortcutOutcome {
    pub(crate) fn prevent_default(self) -> bool {
        match self {
            Self::Handled => true,
            Self::Ignored => false,
            Self::Custom {
                prevent_default, ..
            } => prevent_default,
        }
    }

    pub(crate) fn continue_propagation(self) -> bool {
        match self {
            Self::Handled => false,
            Self::Ignored => true,
            Self::Custom {
                continue_propagation,
                ..
            } => continue_propagation,
        }
    }
}

impl From<()> for ShortcutOutcome {
    fn from((): ()) -> Self {
        Self::Handled
    }
}

impl From<bool> for ShortcutOutcome {
    /// `true`: handled, `false`: ignored.
    fn from(handled: bool) -> Self {
        if handled {
            Self::Handled
        } else {
            Self::Ignored
        }
    }
}

type ShortcutAction = Arc<dyn Fn(&KeyboardEvent) -> ShortcutOutcome + Send + Sync>;

/// A set of keyboard shortcuts and their handlers, consumed by
/// [`use_keyboard`](fn@crate::hooks::interactions::use_keyboard).
///
/// ```ignore
/// let shortcuts = KeyboardShortcuts::new()
///     .on(Shortcut::new(KeyboardKey::ArrowUp), move |_| increment())
///     .on(Shortcut::new(KeyboardKey::Home), move |_| {
///         // Returning `false` leaves the event alone (no preventDefault, keeps bubbling).
///         if let Some(min) = min { set(min); true } else { false }
///     });
/// ```
///
/// A handler may return `()` (handled), a `bool` (handled or ignored) or a [`ShortcutOutcome`].
/// If several entries match an event, the last one added wins.
#[derive(Clone, Default)]
pub struct KeyboardShortcuts {
    bindings: Vec<(Shortcut, ShortcutAction)>,
}

impl KeyboardShortcuts {
    pub fn new() -> Self {
        Self::default()
    }

    /// Run `action` when `shortcut` is pressed.
    #[must_use]
    pub fn on<O: Into<ShortcutOutcome>>(
        mut self,
        shortcut: Shortcut,
        action: impl Fn(&KeyboardEvent) -> O + Send + Sync + 'static,
    ) -> Self {
        self.bindings
            .push((shortcut, Arc::new(move |e| action(e).into())));
        self
    }

    /// These shortcuts and `other`'s; where both bind a shortcut, `other`'s handler wins.
    #[must_use]
    pub fn with(mut self, other: KeyboardShortcuts) -> Self {
        self.bindings.extend(other.bindings);
        self
    }

    /// Run the handler bound to the shortcut `event` triggers, if any.
    /// Returns `None` when no shortcut matched.
    pub fn handle(&self, event: &KeyboardEvent) -> Option<ShortcutOutcome> {
        self.bindings
            .iter()
            .rev()
            .find(|(shortcut, _)| shortcut.matches(event))
            .map(|(_, action)| action(event))
    }
}

impl fmt::Debug for KeyboardShortcuts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(
                self.bindings
                    .iter()
                    .map(|(shortcut, _)| shortcut.to_string()),
            )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[allow(clippy::fn_params_excessive_bools)]
    fn mods(shift: bool, alt: bool, ctrl: bool, meta: bool) -> Modifiers {
        Modifiers {
            shift_key: shift,
            ctrl_key: ctrl,
            meta_key: meta,
            alt_key: alt,
        }
    }

    const NONE: Modifiers = Modifiers {
        shift_key: false,
        ctrl_key: false,
        meta_key: false,
        alt_key: false,
    };

    #[test]
    fn shift_is_ignored_for_characters_without_case() {
        let shift = mods(true, false, false, false);
        // `/` is Shift+7 on German keyboards, `?` takes Shift on most.
        assert_that!(Shortcut::new(KeyboardKey::Slash).matches_key("/", shift, false)).is_true();
        assert_that!(Shortcut::new(KeyboardKey::Slash).matches_key("/", NONE, false)).is_true();
        assert_that!(
            Shortcut::new(KeyboardKey::Other(Cow::Borrowed("?"))).matches_key("?", shift, false)
        )
        .is_true();
        assert_that!(Shortcut::new(KeyboardKey::N7).primary().matches_key(
            "7",
            mods(true, false, true, false),
            false
        ))
        .is_true();
        // Letters and named keys still need the exact modifiers.
        assert_that!(Shortcut::new(KeyboardKey::K).matches_key("K", shift, false)).is_false();
        assert_that!(Shortcut::new(KeyboardKey::Enter).matches_key("Enter", shift, false))
            .is_false();
        // A shortcut requiring Shift still requires it.
        assert_that!(
            Shortcut::new(KeyboardKey::Slash)
                .shift()
                .matches_key("/", NONE, false)
        )
        .is_false();
        // Other modifiers still count.
        assert_that!(Shortcut::new(KeyboardKey::Slash).matches_key(
            "/",
            mods(false, true, false, false),
            false
        ))
        .is_false();
    }

    #[test]
    fn parses_the_plus_key_and_round_trips() {
        let plus = Shortcut::parse("+").unwrap();
        assert_that!(plus.matches_key("+", NONE, false)).is_true();
        let zoom = Shortcut::parse("Mod++").unwrap();
        assert_that!(zoom.to_string()).is_equal_to("Mod++".to_owned());
        assert_that!(Shortcut::parse(&zoom.to_string()).unwrap()).is_equal_to(zoom);
        assert_that!(Shortcut::parse("a+b")).is_err();
        assert_that!(Shortcut::parse("Mod+")).is_err();
    }

    #[test]
    fn aria_keyshortcuts_values() {
        let palette = Shortcut::new(KeyboardKey::K).primary();
        assert_that!(palette.to_aria_keyshortcuts(false).to_string())
            .is_equal_to("Control+K".to_owned());
        assert_that!(palette.to_aria_keyshortcuts(true).to_string())
            .is_equal_to("Meta+K".to_owned());
        assert_that!(
            Shortcut::new(KeyboardKey::Z)
                .primary()
                .shift()
                .to_aria_keyshortcuts(true)
                .to_string()
        )
        .is_equal_to("Shift+Meta+Z".to_owned());
        assert_that!(
            Shortcut::new(KeyboardKey::Space)
                .alt()
                .to_aria_keyshortcuts(false)
                .to_string()
        )
        .is_equal_to("Alt+Space".to_owned());
        assert_that!(
            Shortcut::new(KeyboardKey::ArrowDown)
                .to_aria_keyshortcuts(false)
                .to_string()
        )
        .is_equal_to("ArrowDown".to_owned());
        assert_that!(
            Shortcut::new(KeyboardKey::PageUp)
                .to_aria_keyshortcuts(false)
                .to_string()
        )
        .is_equal_to("PageUp".to_owned());
        assert_that!(
            Shortcut::new(KeyboardKey::Slash)
                .to_aria_keyshortcuts(false)
                .to_string()
        )
        .is_equal_to("/".to_owned());
        let both: AriaKeyshortcuts = [
            palette.to_aria_keyshortcuts(false),
            Shortcut::new(KeyboardKey::Slash).to_aria_keyshortcuts(false),
        ]
        .into_iter()
        .collect();
        assert_that!(both.as_str()).is_equal_to("Control+K /");
    }

    #[test]
    fn apple_keys_show_control_as_its_symbol() {
        let keys = Shortcut::new(KeyboardKey::K).ctrl().primary().keys(true);
        let shown: String = keys.iter().map(|key| key.display().to_owned()).collect();
        assert_that!(shown).is_equal_to("⌃⌘K".to_owned());
        let keys = Shortcut::new(KeyboardKey::K).primary().keys(false);
        let shown: Vec<String> = keys.iter().map(|key| key.display().to_owned()).collect();
        assert_that!(shown).is_equal_to(vec!["Ctrl".to_owned(), "K".to_owned()]);
    }

    #[test]
    fn plain_key_matches_only_without_modifiers() {
        let shortcut = Shortcut::new(KeyboardKey::PageUp);
        assert_that!(shortcut.matches_key("PageUp", NONE, false)).is_true();
        assert_that!(shortcut.matches_key("PageUp", mods(true, false, false, false), false))
            .is_false();
    }

    #[test]
    fn keys_compare_case_insensitively() {
        let shortcut = Shortcut::new(KeyboardKey::Z).shift();
        assert_that!(shortcut.matches_key("Z", mods(true, false, false, false), false)).is_true();
    }

    #[test]
    fn primary_is_platform_dependent() {
        let undo = Shortcut::new(KeyboardKey::Z).primary();
        let ctrl = mods(false, false, true, false);
        let cmd = mods(false, false, false, true);
        assert_that!(undo.matches_key("z", ctrl, false)).is_true();
        assert_that!(undo.matches_key("z", cmd, false)).is_false();
        assert_that!(undo.matches_key("z", cmd, true)).is_true();
        assert_that!(undo.matches_key("z", ctrl, true)).is_false();
    }

    #[test]
    fn modifiers_must_match_exactly() {
        let undo = Shortcut::new(KeyboardKey::Z).primary();
        assert_that!(undo.matches_key("z", mods(true, false, true, false), false)).is_false();
    }

    #[test]
    fn aliases_are_resolved() {
        assert_that!(Shortcut::parse("Down"))
            .ok()
            .is_equal_to(Shortcut::new(KeyboardKey::ArrowDown));
        assert_that!(Shortcut::parse("Space"))
            .ok()
            .is_equal_to(Shortcut::new(KeyboardKey::Space));
        assert_that!(Shortcut::parse("esc"))
            .ok()
            .is_equal_to(Shortcut::new(KeyboardKey::Escape));
        assert_that!(Shortcut::new(KeyboardKey::Space).matches_key(" ", NONE, false)).is_true();
    }

    #[test]
    fn key_names_are_case_insensitive() {
        assert_that!(Shortcut::parse("escape"))
            .ok()
            .is_equal_to(Shortcut::new(KeyboardKey::Escape));
        assert_that!(Shortcut::parse("Mod+pageup"))
            .ok()
            .is_equal_to(Shortcut::new(KeyboardKey::PageUp).primary());
        assert_that!(Shortcut::parse("Z"))
            .ok()
            .is_equal_to(Shortcut::new(KeyboardKey::Z));
    }

    #[test]
    fn parses_react_aria_syntax() {
        assert_that!(Shortcut::parse("Mod+Shift+z"))
            .ok()
            .is_equal_to(Shortcut::new(KeyboardKey::Z).primary().shift());
        assert_that!(Shortcut::parse("shift+MOD+z"))
            .ok()
            .is_equal_to(Shortcut::new(KeyboardKey::Z).primary().shift());
        assert_that!(Shortcut::parse("Ctrl+Alt+Enter"))
            .ok()
            .is_equal_to(Shortcut::new(KeyboardKey::Enter).ctrl().alt());
        assert_that!(Shortcut::parse("Escape"))
            .ok()
            .is_equal_to(Shortcut::new(KeyboardKey::Escape));
    }

    #[test]
    fn rejects_shortcuts_without_exactly_one_key() {
        assert_that!(Shortcut::parse("Mod+Shift")).is_err();
        assert_that!(Shortcut::parse("a+b")).is_err();
        assert_that!(Shortcut::parse("")).is_err();
    }

    #[test]
    fn displays_in_parse_syntax() {
        let shortcut = Shortcut::new(KeyboardKey::Z).primary().shift();
        assert_that!(shortcut.to_string()).is_equal_to("Mod+Shift+Z".to_owned());
        assert_that!(Shortcut::parse(&shortcut.to_string()))
            .ok()
            .is_equal_to(shortcut);
    }

    #[test]
    fn outcome_conversions() {
        assert_that!(ShortcutOutcome::from(())).is_equal_to(ShortcutOutcome::Handled);
        assert_that!(ShortcutOutcome::from(false)).is_equal_to(ShortcutOutcome::Ignored);
        assert_that!(ShortcutOutcome::Handled.prevent_default()).is_true();
        assert_that!(ShortcutOutcome::Handled.continue_propagation()).is_false();
        assert_that!(ShortcutOutcome::Ignored.continue_propagation()).is_true();
    }

    // useKeyboard.test.js' "Mac (Mod = Meta)" and "Windows (Mod = Ctrl)" cases: the matching (the
    // propagation of handled and unhandled keys is covered by the browser tests).

    /// "matches Mod+key with metaKey", "matches Mod+key with ctrlKey".
    #[test]
    fn mod_is_meta_on_mac_and_control_elsewhere() {
        let save = Shortcut::parse("Mod+s").unwrap();
        let meta = mods(false, false, false, true);
        let ctrl = mods(false, false, true, false);
        assert_that!(save.matches_key("s", meta, true)).is_true();
        assert_that!(save.matches_key("s", ctrl, true)).is_false();
        assert_that!(save.matches_key("s", NONE, true)).is_false();
        assert_that!(save.matches_key("s", ctrl, false)).is_true();
        assert_that!(save.matches_key("s", meta, false)).is_false();
    }

    /// "plain key ignores meta".
    #[test]
    fn plain_key_does_not_match_with_meta() {
        let save = Shortcut::parse("s").unwrap();
        assert_that!(save.matches_key("s", mods(false, false, false, true), true)).is_false();
    }

    /// "Control+Shift distinct from Mod+Shift".
    #[test]
    fn control_shift_is_distinct_from_mod_shift_on_mac() {
        let mod_shift = Shortcut::parse("Mod+Shift+a").unwrap();
        let ctrl_shift = Shortcut::parse("Control+Shift+a").unwrap();
        let meta_shift = mods(true, false, false, true);
        let ctrl_shift_pressed = mods(true, false, true, false);
        assert_that!(mod_shift.matches_key("A", meta_shift, true)).is_true();
        assert_that!(ctrl_shift.matches_key("A", meta_shift, true)).is_false();
        assert_that!(mod_shift.matches_key("A", ctrl_shift_pressed, true)).is_false();
        assert_that!(ctrl_shift.matches_key("A", ctrl_shift_pressed, true)).is_true();
    }

    /// "Meta+Control+Alt combination".
    #[test]
    fn meta_control_alt_combination() {
        let shortcut = Shortcut::parse("Meta+Control+Alt+z").unwrap();
        assert_that!(shortcut.matches_key("z", mods(false, true, true, true), true)).is_true();
        assert_that!(shortcut.matches_key("z", mods(false, true, true, false), true)).is_false();
    }

    /// "Shift+Alt and key aliases".
    #[test]
    fn shift_alt_and_key_aliases() {
        let shortcut = Shortcut::parse("Shift+Alt+down").unwrap();
        assert_that!(shortcut.matches_key("ArrowDown", mods(true, true, false, false), true))
            .is_true();
    }

    /// "Mod+Shift+a matches only that binding, not Mod+a".
    #[test]
    fn mod_shift_a_is_not_mod_a() {
        let mod_a = Shortcut::parse("Mod+a").unwrap();
        let mod_shift_a = Shortcut::parse("Mod+Shift+a").unwrap();
        let meta_shift = mods(true, false, false, true);
        let meta = mods(false, false, false, true);
        assert_that!(mod_shift_a.matches_key("A", meta_shift, true)).is_true();
        assert_that!(mod_a.matches_key("A", meta_shift, true)).is_false();
        assert_that!(mod_a.matches_key("a", meta, true)).is_true();
        assert_that!(mod_shift_a.matches_key("a", meta, true)).is_false();
    }

    /// "treats control as an alias for ctrl", "parses modifiers case-insensitively and ignores
    /// order".
    #[test]
    fn modifier_names_and_order() {
        let expected = Shortcut::new(KeyboardKey::K).ctrl().shift().alt();
        for spec in [
            "Control+Shift+Alt+k",
            "ctrl+ALT+shift+k",
            "Shift+Alt+Ctrl+K",
        ] {
            assert_that!(Shortcut::parse(spec))
                .ok()
                .is_equal_to(expected.clone());
        }
    }
}
