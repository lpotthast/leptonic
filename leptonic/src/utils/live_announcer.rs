// Upstream: react-aria/src/live-announcer/LiveAnnouncer.tsx @ 99e6102368
//! Announcements for screen readers, through ARIA live regions.
//!
//! Use this to tell assistive technology about changes that are not otherwise conveyed by focus
//! or by the accessibility tree, like "3 results available" in a combo box, or the new value of a
//! spin button.
//!
//! ```ignore
//! use leptonic::utils::live_announcer::{announce_polite, clear_announcer, Assertiveness};
//!
//! announce_polite("Sorted by name, ascending.");
//! clear_announcer(Some(Assertiveness::Polite));
//! ```
//!
//! Like react-aria's `LiveAnnouncer`, a single, visually hidden element with one `assertive` and
//! one `polite` log region is added to `<body>` on first use. Each message is a child of a log
//! region and is removed after a timeout. The functions do nothing during server-side rendering.

use std::{cell::RefCell, time::Duration};

use leptos::prelude::set_timeout;
use leptos_use::use_document;
use web_sys::Element;

/// How long an announcement stays in its live region.
pub const DEFAULT_ANNOUNCEMENT_TIMEOUT: Duration = Duration::from_secs(7);

/// Wait this long after creating the live regions before announcing into them. Otherwise
/// Safari does not announce the first message.
const FIRST_ANNOUNCEMENT_DELAY: Duration = Duration::from_millis(100);

/// The urgency of an announcement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Assertiveness {
    /// Announced at the next graceful opportunity, e.g. after the current sentence.
    #[default]
    Polite,
    /// Announced immediately, interrupting the current speech.
    Assertive,
}

impl Assertiveness {
    /// The value for the `aria-live` attribute.
    pub fn as_aria_live(&self) -> &'static str {
        match self {
            Self::Polite => "polite",
            Self::Assertive => "assertive",
        }
    }
}

/// What to announce.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Announcement {
    /// A plain text message.
    Text(String),
    /// The accessible name of existing elements, given by their ids (like `aria-labelledby`, in
    /// order). Useful to announce rich content that is already in the page.
    LabelledBy(Vec<String>),
}

impl From<String> for Announcement {
    fn from(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<&str> for Announcement {
    fn from(text: &str) -> Self {
        Self::Text(text.to_owned())
    }
}

/// Announce `message` to screen readers.
pub fn announce(message: impl Into<Announcement>, assertiveness: Assertiveness) {
    announce_with_timeout(message, assertiveness, DEFAULT_ANNOUNCEMENT_TIMEOUT);
}

/// Announce `message` politely: after whatever is currently being read.
pub fn announce_polite(message: impl Into<Announcement>) {
    announce(message, Assertiveness::Polite);
}

/// Announce `message` assertively: interrupting whatever is currently being read.
pub fn announce_assertive(message: impl Into<Announcement>) {
    announce(message, Assertiveness::Assertive);
}

/// Announce `message`, removing it from its live region after `timeout`.
pub fn announce_with_timeout(
    message: impl Into<Announcement>,
    assertiveness: Assertiveness,
    timeout: Duration,
) {
    let message = message.into();
    ANNOUNCER.with(|announcer| {
        let mut announcer = announcer.borrow_mut();
        if let Some(existing) = announcer.as_ref() {
            existing.announce(&message, assertiveness, timeout);
            return;
        }
        let Some(created) = LiveAnnouncer::create() else {
            // No document: server-side rendering.
            return;
        };
        *announcer = Some(created);
        set_timeout(
            move || {
                ANNOUNCER.with(|announcer| {
                    if let Some(announcer) = announcer.borrow().as_ref()
                        && announcer.is_attached()
                    {
                        announcer.announce(&message, assertiveness, timeout);
                    }
                });
            },
            FIRST_ANNOUNCEMENT_DELAY,
        );
    });
}

/// Remove all pending announcements of the given assertiveness (`None`: of both kinds).
pub fn clear_announcer(assertiveness: Option<Assertiveness>) {
    ANNOUNCER.with(|announcer| {
        if let Some(announcer) = announcer.borrow().as_ref() {
            announcer.clear(assertiveness);
        }
    });
}

/// Remove the live regions from the document. The next announcement creates them again.
pub fn destroy_announcer() {
    ANNOUNCER.with(|announcer| {
        if let Some(announcer) = announcer.borrow_mut().take() {
            announcer.node.remove();
        }
    });
}

thread_local! {
    // The browser runs leptonic on a single thread. During SSR there is no document, so no
    // announcer is ever created.
    static ANNOUNCER: RefCell<Option<LiveAnnouncer>> = const { RefCell::new(None) };
}

struct LiveAnnouncer {
    node: Element,
    assertive_log: Element,
    polite_log: Element,
}

impl LiveAnnouncer {
    fn create() -> Option<Self> {
        let document = use_document();
        let document = document.as_ref()?;
        let body = document.body()?;

        let node = document.create_element("div").ok()?;
        node.set_attribute("data-live-announcer", "true").ok()?;
        // Visually hidden, but read by screen readers.
        node.set_attribute("style", super::visually_hidden::VISUALLY_HIDDEN_STYLE)
            .ok()?;

        let create_log = |assertiveness: Assertiveness| -> Option<Element> {
            let log = document.create_element("div").ok()?;
            log.set_attribute("role", "log").ok()?;
            log.set_attribute("aria-live", assertiveness.as_aria_live())
                .ok()?;
            log.set_attribute("aria-relevant", "additions").ok()?;
            node.append_child(&log).ok()?;
            Some(log)
        };
        let assertive_log = create_log(Assertiveness::Assertive)?;
        let polite_log = create_log(Assertiveness::Polite)?;

        body.prepend_with_node_1(&node).ok()?;
        Some(Self {
            node,
            assertive_log,
            polite_log,
        })
    }

    fn is_attached(&self) -> bool {
        self.node.is_connected()
    }

    fn log(&self, assertiveness: Assertiveness) -> &Element {
        match assertiveness {
            Assertiveness::Assertive => &self.assertive_log,
            Assertiveness::Polite => &self.polite_log,
        }
    }

    fn announce(&self, message: &Announcement, assertiveness: Assertiveness, timeout: Duration) {
        let Some(document) = self.node.owner_document() else {
            return;
        };
        let Ok(entry) = document.create_element("div") else {
            return;
        };
        let is_empty = match message {
            Announcement::Text(text) => {
                entry.set_text_content(Some(text));
                text.is_empty()
            }
            Announcement::LabelledBy(ids) => {
                // To read an aria-labelledby, the element needs a role that supports naming.
                let _ = entry.set_attribute("role", "img");
                let _ = entry.set_attribute("aria-labelledby", &ids.join(" "));
                false
            }
        };
        if self.log(assertiveness).append_child(&entry).is_err() {
            return;
        }
        if !is_empty {
            set_timeout(move || entry.remove(), timeout);
        }
    }

    fn clear(&self, assertiveness: Option<Assertiveness>) {
        if assertiveness.is_none_or(|a| a == Assertiveness::Assertive) {
            self.assertive_log.set_inner_html("");
        }
        if assertiveness.is_none_or(|a| a == Assertiveness::Polite) {
            self.polite_log.set_inner_html("");
        }
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn assertiveness_maps_to_aria_live() {
        assert_that!(Assertiveness::Polite.as_aria_live()).is_equal_to("polite");
        assert_that!(Assertiveness::Assertive.as_aria_live()).is_equal_to("assertive");
    }

    // `use_document()` only reports "no document" with the `ssr` feature; otherwise it calls into
    // web-sys, which is unavailable natively.
    #[cfg(feature = "ssr")]
    #[test]
    fn announcing_without_a_document_does_nothing() {
        // During SSR there is no document: no announcer is created, no panic.
        announce_polite("Hello");
        let created = ANNOUNCER.with(|announcer| announcer.borrow().is_some());
        assert_that!(created).is_false();
    }
}
