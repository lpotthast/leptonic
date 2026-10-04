// Upstream: react-aria/src/interactions/createKeyboardShortcutHandler.ts @ 99e6102368
//! Keyboard shortcuts: a key plus an exact set of modifiers, mapped to handlers.
//!
//! This is leptonic's take on react-aria's `createKeyboardShortcutHandler`. React-aria describes
//! shortcuts as strings (`"Mod+Shift+z"`) that are parsed at runtime. Here, shortcuts are typed
//! values, built with `const` functions:
//!
//! ```
//! use leptonic::utils::keyboard_shortcut::Shortcut;
//!
//! const UNDO: Shortcut = Shortcut::key("z").primary();
//! const REDO: Shortcut = Shortcut::key("z").primary().shift();
//! const PAGE_UP: Shortcut = Shortcut::key("PageUp");
//! ```
//!
//! [`Shortcut::parse`] accepts react-aria's string syntax as well, for shortcuts that come from
//! configuration or user input.
//!
//! A shortcut matches a keyboard event when the key matches (case-insensitively) and the *exact*
//! set of modifiers is held. `Shortcut::key("z").primary()` does not match `Ctrl+Shift+Z`.

use std::{borrow::Cow, fmt, str::FromStr, sync::Arc};

use web_sys::KeyboardEvent;

use crate::utils::{EventModifiers, Modifiers, platform::device};

/// A key combination: one key plus the modifiers that must be held, and no others.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Shortcut {
    key: Cow<'static, str>,
    modifiers: Modifiers,
    /// The platform's primary modifier (Command on Apple platforms, Control elsewhere).
    primary: bool,
}

impl Shortcut {
    /// A shortcut for `key` without modifiers.
    ///
    /// `key` is a [`KeyboardEvent.key`](https://developer.mozilla.org/docs/Web/API/KeyboardEvent/key)
    /// value such as `"a"`, `"Enter"`, `" "` or `"ArrowDown"`. It is compared case-insensitively.
    /// The aliases `Space`, `Esc`, `Del`, `Ins`, `Left`, `Right`, `Up` and `Down` are accepted.
    pub const fn key(key: &'static str) -> Self {
        Self {
            key: Cow::Borrowed(key),
            modifiers: Modifiers {
                shift_key: false,
                ctrl_key: false,
                meta_key: false,
                alt_key: false,
            },
            primary: false,
        }
    }

    /// Like [`Shortcut::key`], for keys only known at runtime.
    pub fn owned_key(key: String) -> Self {
        Self {
            key: Cow::Owned(key),
            ..Self::key("")
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
    /// `"Escape"`. Modifier names (`Shift`, `Alt`, `Control`/`Ctrl`, `Meta`, `Mod`) are
    /// case-insensitive and may come in any order. Exactly one non-modifier key is required.
    ///
    /// # Errors
    ///
    /// Returns an error if `spec` names no key or more than one.
    pub fn parse(spec: &str) -> Result<Self, InvalidShortcut> {
        let mut shortcut = Self::key("");
        let mut key: Option<&str> = None;
        for part in spec.split('+') {
            match part.to_ascii_lowercase().as_str() {
                "shift" => shortcut.modifiers.shift_key = true,
                "alt" => shortcut.modifiers.alt_key = true,
                "control" | "ctrl" => shortcut.modifiers.ctrl_key = true,
                "meta" => shortcut.modifiers.meta_key = true,
                "mod" => shortcut.primary = true,
                _ if key.is_some() => return Err(InvalidShortcut(spec.to_owned())),
                _ => key = Some(part),
            }
        }
        match key {
            // `"+"` itself splits into two empty parts; treat a trailing empty part as the key.
            Some(key) if !key.is_empty() => {
                shortcut.key = Cow::Owned(key.to_owned());
                Ok(shortcut)
            }
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
        pressed == required && canonical_key(&self.key) == canonical_key(key)
    }
}

impl FromStr for Shortcut {
    type Err = InvalidShortcut;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
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
        f.write_str(&self.key)
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

fn canonical_key(key: &str) -> Cow<'_, str> {
    let lower = key.to_lowercase();
    let aliased = match lower.as_str() {
        "space" => " ",
        "esc" => "escape",
        "del" => "delete",
        "ins" => "insert",
        "left" => "arrowleft",
        "right" => "arrowright",
        "up" => "arrowup",
        "down" => "arrowdown",
        _ => return Cow::Owned(lower),
    };
    Cow::Borrowed(aliased)
}

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
/// [`use_keyboard`](crate::hooks::use_keyboard).
///
/// ```ignore
/// let shortcuts = KeyboardShortcuts::new()
///     .on(Shortcut::key("ArrowUp"), move |_| increment())
///     .on(Shortcut::key("Home"), move |_| {
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
    fn plain_key_matches_only_without_modifiers() {
        let shortcut = Shortcut::key("PageUp");
        assert_that!(shortcut.matches_key("PageUp", NONE, false)).is_true();
        assert_that!(shortcut.matches_key("PageUp", mods(true, false, false, false), false))
            .is_false();
    }

    #[test]
    fn keys_compare_case_insensitively() {
        let shortcut = Shortcut::key("z").shift();
        assert_that!(shortcut.matches_key("Z", mods(true, false, false, false), false)).is_true();
    }

    #[test]
    fn primary_is_platform_dependent() {
        let undo = Shortcut::key("z").primary();
        let ctrl = mods(false, false, true, false);
        let cmd = mods(false, false, false, true);
        assert_that!(undo.matches_key("z", ctrl, false)).is_true();
        assert_that!(undo.matches_key("z", cmd, false)).is_false();
        assert_that!(undo.matches_key("z", cmd, true)).is_true();
        assert_that!(undo.matches_key("z", ctrl, true)).is_false();
    }

    #[test]
    fn modifiers_must_match_exactly() {
        let undo = Shortcut::key("z").primary();
        assert_that!(undo.matches_key("z", mods(true, false, true, false), false)).is_false();
    }

    #[test]
    fn aliases_are_resolved() {
        assert_that!(Shortcut::key("Down").matches_key("ArrowDown", NONE, false)).is_true();
        assert_that!(Shortcut::key("Space").matches_key(" ", NONE, false)).is_true();
        assert_that!(Shortcut::key("Esc").matches_key("Escape", NONE, false)).is_true();
    }

    #[test]
    fn parses_react_aria_syntax() {
        assert_that!(Shortcut::parse("Mod+Shift+z"))
            .get_ok()
            .is_equal_to(Shortcut::key("z").primary().shift());
        assert_that!(Shortcut::parse("shift+MOD+z"))
            .get_ok()
            .is_equal_to(Shortcut::key("z").primary().shift());
        assert_that!(Shortcut::parse("Ctrl+Alt+Enter"))
            .get_ok()
            .is_equal_to(Shortcut::key("Enter").ctrl().alt());
        assert_that!(Shortcut::parse("Escape"))
            .get_ok()
            .is_equal_to(Shortcut::key("Escape"));
    }

    #[test]
    fn rejects_shortcuts_without_exactly_one_key() {
        assert_that!(Shortcut::parse("Mod+Shift")).is_err();
        assert_that!(Shortcut::parse("a+b")).is_err();
        assert_that!(Shortcut::parse("")).is_err();
    }

    #[test]
    fn displays_in_parse_syntax() {
        let shortcut = Shortcut::key("z").primary().shift();
        assert_that!(shortcut.to_string()).is_equal_to("Mod+Shift+z".to_owned());
        assert_that!(Shortcut::parse(&shortcut.to_string()))
            .get_ok()
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
}
