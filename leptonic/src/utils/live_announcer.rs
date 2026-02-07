use leptos::prelude::*;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/live-announcer/src/LiveAnnouncer.tsx

/// The assertiveness level of a live announcement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Assertiveness {
    /// Polite announcements wait for the user to finish their current task.
    #[default]
    Polite,
    /// Assertive announcements interrupt the user immediately.
    Assertive,
}

impl Assertiveness {
    /// Returns the ARIA live attribute value.
    #[must_use]
    pub fn as_aria_live(&self) -> &'static str {
        match self {
            Self::Polite => "polite",
            Self::Assertive => "assertive",
        }
    }
}

/// A message to be announced to screen readers.
#[derive(Debug, Clone)]
pub struct Announcement {
    /// Unique ID for the announcement.
    pub id: String,
    /// The message to announce.
    pub message: String,
    /// The assertiveness level.
    pub assertiveness: Assertiveness,
}

impl Announcement {
    /// Creates a new announcement.
    #[must_use]
    pub fn new(message: impl Into<String>, assertiveness: Assertiveness) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            message: message.into(),
            assertiveness,
        }
    }
}

/// The live announcer context for managing screen reader announcements.
#[derive(Clone)]
pub struct LiveAnnouncerContext {
    /// Queue of polite announcements.
    polite_queue: Arc<Mutex<VecDeque<Announcement>>>,
    /// Queue of assertive announcements.
    assertive_queue: Arc<Mutex<VecDeque<Announcement>>>,
    /// Signal to trigger re-renders when announcements change.
    trigger: RwSignal<u32>,
}

impl Default for LiveAnnouncerContext {
    fn default() -> Self {
        Self::new()
    }
}

impl LiveAnnouncerContext {
    /// Creates a new live announcer context.
    #[must_use]
    pub fn new() -> Self {
        Self {
            polite_queue: Arc::new(Mutex::new(VecDeque::new())),
            assertive_queue: Arc::new(Mutex::new(VecDeque::new())),
            trigger: RwSignal::new(0),
        }
    }

    /// Announces a message to screen readers.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to announce.
    /// * `assertiveness` - The assertiveness level (polite or assertive).
    pub fn announce(&self, message: impl Into<String>, assertiveness: Assertiveness) {
        let announcement = Announcement::new(message, assertiveness);

        match assertiveness {
            Assertiveness::Polite => {
                if let Ok(mut queue) = self.polite_queue.lock() {
                    queue.push_back(announcement);
                }
            }
            Assertiveness::Assertive => {
                if let Ok(mut queue) = self.assertive_queue.lock() {
                    queue.push_back(announcement);
                }
            }
        }

        // Trigger re-render
        self.trigger.update(|n| *n = n.wrapping_add(1));
    }

    /// Announces a polite message to screen readers.
    ///
    /// Polite messages wait for the user to finish their current task.
    pub fn announce_polite(&self, message: impl Into<String>) {
        self.announce(message, Assertiveness::Polite);
    }

    /// Announces an assertive message to screen readers.
    ///
    /// Assertive messages interrupt the user immediately.
    pub fn announce_assertive(&self, message: impl Into<String>) {
        self.announce(message, Assertiveness::Assertive);
    }

    /// Clears all announcements.
    pub fn clear(&self) {
        if let Ok(mut queue) = self.polite_queue.lock() {
            queue.clear();
        }
        if let Ok(mut queue) = self.assertive_queue.lock() {
            queue.clear();
        }
        self.trigger.update(|n| *n = n.wrapping_add(1));
    }

    /// Clears polite announcements.
    pub fn clear_polite(&self) {
        if let Ok(mut queue) = self.polite_queue.lock() {
            queue.clear();
        }
        self.trigger.update(|n| *n = n.wrapping_add(1));
    }

    /// Clears assertive announcements.
    pub fn clear_assertive(&self) {
        if let Ok(mut queue) = self.assertive_queue.lock() {
            queue.clear();
        }
        self.trigger.update(|n| *n = n.wrapping_add(1));
    }

    /// Gets the current polite message (most recent).
    #[must_use]
    pub fn polite_message(&self) -> Option<String> {
        self.polite_queue
            .lock()
            .ok()
            .and_then(|queue| queue.back().map(|a| a.message.clone()))
    }

    /// Gets the current assertive message (most recent).
    #[must_use]
    pub fn assertive_message(&self) -> Option<String> {
        self.assertive_queue
            .lock()
            .ok()
            .and_then(|queue| queue.back().map(|a| a.message.clone()))
    }

    /// Gets the trigger signal for reactive updates.
    #[must_use]
    pub fn trigger(&self) -> RwSignal<u32> {
        self.trigger
    }
}

/// Provides live announcer context to descendant components.
///
/// This component creates the visually hidden live regions that screen readers
/// use to announce dynamic content changes.
///
/// # Example
///
/// ```ignore
/// view! {
///     <LiveAnnouncerProvider>
///         <App />
///     </LiveAnnouncerProvider>
/// }
/// ```
#[component]
pub fn LiveAnnouncerProvider(
    /// Children to render.
    children: Children,
) -> impl IntoView {
    let context = LiveAnnouncerContext::new();
    provide_context(context.clone());

    let polite_message = {
        let ctx = context.clone();
        Memo::new(move |_| {
            // Subscribe to trigger
            ctx.trigger.get();
            ctx.polite_message().unwrap_or_default()
        })
    };

    let assertive_message = {
        let ctx = context.clone();
        Memo::new(move |_| {
            // Subscribe to trigger
            ctx.trigger.get();
            ctx.assertive_message().unwrap_or_default()
        })
    };

    view! {
        {children()}

        // Visually hidden live regions
        <div style="position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border: 0;">
            <div role="log" aria-live="polite" aria-relevant="additions">
                {move || polite_message.get()}
            </div>
            <div role="log" aria-live="assertive" aria-relevant="additions">
                {move || assertive_message.get()}
            </div>
        </div>
    }
}

/// Returns the live announcer context.
///
/// # Panics
///
/// Panics if called outside of a `LiveAnnouncerProvider`.
#[must_use]
pub fn use_live_announcer() -> LiveAnnouncerContext {
    use_context::<LiveAnnouncerContext>()
        .expect("use_live_announcer must be used within a LiveAnnouncerProvider")
}

/// Returns the live announcer context, if available.
#[must_use]
pub fn try_use_live_announcer() -> Option<LiveAnnouncerContext> {
    use_context::<LiveAnnouncerContext>()
}

/// Announces a message to screen readers.
///
/// This is a convenience function that uses the live announcer context
/// if available, otherwise creates a standalone announcement.
///
/// # Arguments
///
/// * `message` - The message to announce.
/// * `assertiveness` - The assertiveness level (polite or assertive).
pub fn announce(message: impl Into<String>, assertiveness: Assertiveness) {
    if let Some(ctx) = try_use_live_announcer() {
        ctx.announce(message, assertiveness);
    } else {
        // Fallback: create a temporary live region
        // This is less ideal but ensures announcements still work
        #[cfg(target_arch = "wasm32")]
        {
            let message = message.into();
            let assertiveness_str = assertiveness.as_aria_live();

            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                if let Ok(div) = document.create_element("div") {
                    let _ = div.set_attribute("role", "log");
                    let _ = div.set_attribute("aria-live", assertiveness_str);
                    let _ = div.set_attribute("aria-relevant", "additions");
                    let _ = div.set_attribute(
                        "style",
                        "position: absolute; width: 1px; height: 1px; padding: 0; \
                         margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); \
                         white-space: nowrap; border: 0;",
                    );

                    if let Some(body) = document.body() {
                        let _ = body.append_child(&div);

                        // Set the message after a small delay to ensure screen readers pick it up
                        let div_clone = div.clone();
                        set_timeout(
                            move || {
                                div_clone.set_text_content(Some(&message));

                                // Remove after announcement
                                let div_remove = div_clone.clone();
                                set_timeout(
                                    move || {
                                        let _ = div_remove.remove();
                                    },
                                    std::time::Duration::from_millis(1000),
                                );
                            },
                            std::time::Duration::from_millis(100),
                        );
                    }
                }
            }
        }
    }
}

/// Announces a polite message to screen readers.
///
/// Convenience function for `announce(message, Assertiveness::Polite)`.
pub fn announce_polite(message: impl Into<String>) {
    announce(message, Assertiveness::Polite);
}

/// Announces an assertive message to screen readers.
///
/// Convenience function for `announce(message, Assertiveness::Assertive)`.
pub fn announce_assertive(message: impl Into<String>) {
    announce(message, Assertiveness::Assertive);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assertiveness_aria_live() {
        assert_eq!(Assertiveness::Polite.as_aria_live(), "polite");
        assert_eq!(Assertiveness::Assertive.as_aria_live(), "assertive");
    }

    #[test]
    fn test_live_announcer_context() {
        let ctx = LiveAnnouncerContext::new();

        ctx.announce_polite("Hello");
        assert_eq!(ctx.polite_message(), Some("Hello".to_string()));

        ctx.announce_assertive("Alert!");
        assert_eq!(ctx.assertive_message(), Some("Alert!".to_string()));

        ctx.clear();
        assert_eq!(ctx.polite_message(), None);
        assert_eq!(ctx.assertive_message(), None);
    }

    #[test]
    fn test_announcement_new() {
        let announcement = Announcement::new("Test message", Assertiveness::Polite);
        assert_eq!(announcement.message, "Test message");
        assert_eq!(announcement.assertiveness, Assertiveness::Polite);
        assert!(!announcement.id.is_empty());
    }
}
