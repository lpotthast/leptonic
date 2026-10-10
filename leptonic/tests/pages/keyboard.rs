//! Keyboard input that doesn't depend on the host's keyboard layout (`Page::send_keys`,
//! `ElementActions::type_keys`).
//!
//! chromedriver types a character through the host's active input source: it looks up the key
//! that produces the character there and the browser then reports that key. With a German layout
//! on macOS, `/` arrives as Shift+7 with `key` `&` and `z` as the key `y`. So the characters a US
//! keyboard has are typed here through CDP (`Input.dispatchKeyEvent`), as that keyboard
//! produces them: `key`, `code`, key code and text of its key, with Shift held (its own
//! `keydown`/`keyup` around the character) for the characters that take Shift (`?` is
//! Shift+Slash). Everything else stays on WebDriver, which handles it the same on every layout:
//! named keys (`Key::Enter`, `Key::Tab`, arrows) and chords, i.e. whatever follows a modifier
//! (`Key::Control + "a"`) up to the `Key::Null` that releases it.

use std::sync::Arc;

use browser_test::thirtyfour::{
    Key, TypingData, WebElement,
    cdp::{Cdp, CdpCommand, Empty},
    session::handle::SessionHandle,
};
use rootcause::{Report, prelude::ResultExt};
use serde::Serialize;

/// Send `keys` to whatever has focus, each key at the moment it is typed: text a US keyboard
/// types through CDP, the rest through WebDriver (see the module documentation).
pub(super) async fn send_keys(
    session: &Arc<SessionHandle>,
    keys: &TypingData,
) -> Result<(), Report> {
    for part in split(keys) {
        match part {
            Part::Typed(keys) => {
                let cdp = Cdp::new(Arc::clone(session));
                for key in keys {
                    key.type_on(&cdp).await?;
                }
            }
            Part::WebDriver(keys) => {
                focused_element(session)
                    .await?
                    .send_keys(keys.as_str())
                    .await
                    .context_with(|| format!("failed to send {keys:?} to the focused element"))?;
            }
        }
    }
    Ok(())
}

/// The deeply focused element, including inside open shadow roots (`<body>` when none).
pub(super) async fn focused_element(session: &Arc<SessionHandle>) -> Result<WebElement, Report> {
    session
        .execute(
            "let element = document.activeElement;
             while (element?.shadowRoot?.activeElement) element = element.shadowRoot.activeElement;
             return element;",
            vec![],
        )
        .await
        .context("failed to get the focused element")?
        .element()
        .map_err(Into::into)
}

/// A run of `keys` that one channel sends.
#[derive(Debug, PartialEq)]
enum Part {
    /// Characters of a US keyboard, typed through CDP.
    Typed(Vec<UsKey>),
    /// Named keys and chords (and characters no US keyboard has), sent through WebDriver.
    WebDriver(String),
}

/// Split `keys` into the runs each channel sends. Once a modifier is pressed, everything up to
/// `Key::Null` (which releases it) stays on WebDriver, which holds the modifier meanwhile.
fn split(keys: &TypingData) -> Vec<Part> {
    let modifiers = [Key::Shift, Key::Control, Key::Alt, Key::Meta].map(char::from);
    let mut parts = Vec::new();
    let mut modifier_held = false;
    for character in keys.as_vec() {
        let typed = if modifier_held {
            None
        } else {
            UsKey::of(character)
        };
        match (typed, parts.last_mut()) {
            (Some(key), Some(Part::Typed(keys))) => keys.push(key),
            (Some(key), _) => parts.push(Part::Typed(vec![key])),
            (None, Some(Part::WebDriver(keys))) => keys.push(character),
            (None, _) => parts.push(Part::WebDriver(character.to_string())),
        }
        if modifiers.contains(&character) {
            modifier_held = true;
        } else if character == char::from(Key::Null) {
            modifier_held = false;
        }
    }
    parts
}

/// The key of a US keyboard that types a character.
#[derive(Debug, PartialEq)]
struct UsKey {
    /// The character: the event's `key` and the text it types.
    character: char,
    /// The physical key (`KeyA`, `Digit1`, `Slash`).
    code: String,
    /// Its Windows virtual key code, the event's `keyCode`.
    key_code: u32,
    /// Whether the character takes Shift (`A`, `?`).
    shift: bool,
}

impl UsKey {
    /// The key typing `character`, if a US keyboard has one.
    fn of(character: char) -> Option<Self> {
        let key = |code: &str, key_code: u32, shift: bool| UsKey {
            character,
            code: code.to_owned(),
            key_code,
            shift,
        };
        Some(match character {
            'a'..='z' | 'A'..='Z' => {
                let upper = character.to_ascii_uppercase();
                key(
                    &format!("Key{upper}"),
                    u32::from(upper),
                    character.is_ascii_uppercase(),
                )
            }
            '0'..='9' => key(&format!("Digit{character}"), u32::from(character), false),
            ' ' => key("Space", 32, false),
            ')' => key("Digit0", 48, true),
            '!' => key("Digit1", 49, true),
            '@' => key("Digit2", 50, true),
            '#' => key("Digit3", 51, true),
            '$' => key("Digit4", 52, true),
            '%' => key("Digit5", 53, true),
            '^' => key("Digit6", 54, true),
            '&' => key("Digit7", 55, true),
            '*' => key("Digit8", 56, true),
            '(' => key("Digit9", 57, true),
            ';' | ':' => key("Semicolon", 186, character == ':'),
            '=' | '+' => key("Equal", 187, character == '+'),
            ',' | '<' => key("Comma", 188, character == '<'),
            '-' | '_' => key("Minus", 189, character == '_'),
            '.' | '>' => key("Period", 190, character == '>'),
            '/' | '?' => key("Slash", 191, character == '?'),
            '`' | '~' => key("Backquote", 192, character == '~'),
            '[' | '{' => key("BracketLeft", 219, character == '{'),
            '\\' | '|' => key("Backslash", 220, character == '|'),
            ']' | '}' => key("BracketRight", 221, character == '}'),
            '\'' | '"' => key("Quote", 222, character == '"'),
            _ => return None,
        })
    }

    /// Press and release the key, with Shift held around it if the character takes Shift.
    async fn type_on(&self, cdp: &Cdp) -> Result<(), Report> {
        let modifiers = if self.shift { SHIFT } else { 0 };
        if self.shift {
            send(cdp, KeyEvent::shift(KeyEventType::RawKeyDown, SHIFT)).await?;
        }
        let text = self.character.to_string();
        send(
            cdp,
            KeyEvent {
                r#type: KeyEventType::KeyDown,
                modifiers,
                key: text.clone(),
                code: self.code.clone(),
                windows_virtual_key_code: self.key_code,
                location: 0,
                text: Some(text.clone()),
                unmodified_text: Some(text.clone()),
            },
        )
        .await?;
        send(
            cdp,
            KeyEvent {
                r#type: KeyEventType::KeyUp,
                modifiers,
                key: text,
                code: self.code.clone(),
                windows_virtual_key_code: self.key_code,
                location: 0,
                text: None,
                unmodified_text: None,
            },
        )
        .await?;
        if self.shift {
            send(cdp, KeyEvent::shift(KeyEventType::KeyUp, 0)).await?;
        }
        Ok(())
    }
}

/// CDP's modifier bit of Shift.
const SHIFT: u32 = 8;

async fn send(cdp: &Cdp, event: KeyEvent) -> Result<(), Report> {
    let description = format!("{:?} of {:?}", event.r#type, event.key);
    cdp.send(event)
        .await
        .context_with(|| format!("failed to dispatch the {description} through CDP"))?;
    Ok(())
}

/// `Input.dispatchKeyEvent` with the fields a key of a US keyboard needs (thirtyfour's typed
/// command has no `location`).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct KeyEvent {
    r#type: KeyEventType,
    modifiers: u32,
    key: String,
    code: String,
    windows_virtual_key_code: u32,
    location: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unmodified_text: Option<String>,
}

impl KeyEvent {
    /// The left Shift key going down or up.
    fn shift(r#type: KeyEventType, modifiers: u32) -> Self {
        Self {
            r#type,
            modifiers,
            key: "Shift".to_owned(),
            code: "ShiftLeft".to_owned(),
            windows_virtual_key_code: 16,
            location: 1,
            text: None,
            unmodified_text: None,
        }
    }
}

impl CdpCommand for KeyEvent {
    const METHOD: &'static str = "Input.dispatchKeyEvent";
    type Returns = Empty;
}

/// `keyDown` types the event's text, `rawKeyDown` (a key without text) types nothing.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
enum KeyEventType {
    RawKeyDown,
    KeyDown,
    KeyUp,
}
