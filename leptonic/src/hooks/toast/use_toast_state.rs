// Upstream: react-stately/src/toast/useToastState.ts @ 99e6102368
// Upstream: react-stately/test/toast/useToastState.test.js @ 99e6102368
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use leptos::prelude::*;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `ToastQueue` is a `Copy` handle of signals, created where it lives (e.g. at the app's
//   root) and used from anywhere (react-aria: a class instance with subscriptions); its
//   `visible_toasts` is a signal (react-aria: `useSyncExternalStore`).
// - Timeouts are `Duration`s; keys are a `Copy` `ToastKey` counting up (react-aria: random
//   strings).
//
// ## OMITTED FEATURES
// - `wrapUpdate` (react-aria-components wraps updates in view transitions).
//
// =============================================================================

/// Options of a toast.
#[derive(Clone, Default)]
pub struct ToastOptions {
    /// Closes the toast after this time (paused while the toasts are hovered or focused). At
    /// least 5 seconds are recommended; toasts with actions shouldn't time out.
    pub timeout: Option<Duration>,
    /// Called when the toast closes.
    pub on_close: Option<Callback<()>>,
}

/// A pausable timeout (react-stately's `Timer`).
#[derive(Clone)]
pub struct ToastTimer {
    inner: Arc<Mutex<TimerState>>,
}

struct TimerState {
    handle: Option<TimeoutHandle>,
    started: f64,
    remaining: f64,
    callback: Arc<dyn Fn() + Send + Sync>,
}

impl std::fmt::Debug for ToastTimer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToastTimer").finish_non_exhaustive()
    }
}

/// The time for pausing a timer. The JS clock exists only in WebAssembly; natively (server-side
/// rendering, native tests) no timer runs (`resume` needs the browser's `setTimeout`): a constant.
fn now() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0.0
    }
}

impl ToastTimer {
    fn new(callback: impl Fn() + Send + Sync + 'static, delay: Duration) -> Self {
        Self {
            inner: Arc::new(Mutex::new(TimerState {
                handle: None,
                started: 0.0,
                remaining: delay.as_secs_f64() * 1000.0,
                callback: Arc::new(callback),
            })),
        }
    }

    /// Starts over with `delay`.
    pub fn reset(&self, delay: Duration) {
        if let Ok(mut state) = self.inner.lock() {
            if let Some(handle) = state.handle.take() {
                handle.clear();
            }
            state.remaining = delay.as_secs_f64() * 1000.0;
        }
        self.resume();
    }

    pub fn pause(&self) {
        let Ok(mut state) = self.inner.lock() else {
            return;
        };
        let Some(handle) = state.handle.take() else {
            return;
        };
        handle.clear();
        state.remaining -= now() - state.started;
    }

    pub fn resume(&self) {
        let Ok(mut state) = self.inner.lock() else {
            return;
        };
        if state.remaining <= 0.0 || state.handle.is_some() {
            return;
        }
        state.started = now();
        let inner = Arc::clone(&self.inner);
        let delay = Duration::from_secs_f64(state.remaining / 1000.0);
        state.handle = set_timeout_with_handle(
            move || {
                let callback = inner.lock().ok().map(|mut state| {
                    state.handle = None;
                    state.remaining = 0.0;
                    Arc::clone(&state.callback)
                });
                if let Some(callback) = callback {
                    callback();
                }
            },
            delay,
        )
        .ok();
    }
}

/// Identifies a toast in its [`ToastQueue`] (returned by [`ToastQueue::add`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ToastKey(u64);

/// A toast in the queue.
#[derive(Debug, Clone)]
pub struct QueuedToast<T> {
    pub key: ToastKey,
    pub content: T,
    pub timeout: Option<Duration>,
    pub on_close: Option<Callback<()>>,
    pub timer: Option<ToastTimer>,
}

impl<T> PartialEq for QueuedToast<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

/// The toasts of an app, newest first (react-stately's `ToastQueue`): created once (e.g. at the
/// app's root), toasts added from anywhere, shown by a toast region.
pub struct ToastQueue<T: Clone + Send + Sync + 'static> {
    toasts: RwSignal<Vec<QueuedToast<T>>>,
    next_key: StoredValue<u64>,
    /// The toasts shown: the newest, up to the maximum.
    pub visible_toasts: Memo<Vec<QueuedToast<T>>>,
}

// Derived, it would require a `Copy` content type.
#[allow(clippy::expl_impl_clone_on_copy)]
impl<T: Clone + Send + Sync + 'static> Clone for ToastQueue<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Clone + Send + Sync + 'static> Copy for ToastQueue<T> {}

impl<T: Clone + Send + Sync + 'static> ToastQueue<T> {
    /// A queue showing up to `max_visible_toasts` (`None`: all).
    pub fn new(max_visible_toasts: Option<usize>) -> Self {
        let toasts = RwSignal::new(Vec::<QueuedToast<T>>::new());
        let max_visible = StoredValue::new(max_visible_toasts.unwrap_or(usize::MAX));
        Self {
            toasts,
            next_key: StoredValue::new(0),
            visible_toasts: Memo::new(move |_| {
                toasts.with(|toasts| {
                    toasts
                        .iter()
                        .take(max_visible.get_value())
                        .cloned()
                        .collect()
                })
            }),
        }
    }

    /// Adds a toast (shown first); returns its key.
    pub fn add(&self, content: T, options: ToastOptions) -> ToastKey {
        let number = self.next_key.get_value() + 1;
        self.next_key.set_value(number);
        let key = ToastKey(number);
        let queue = *self;
        let timer = options
            .timeout
            .map(|timeout| ToastTimer::new(move || queue.close(key), timeout));
        self.toasts.update(|toasts| {
            toasts.insert(
                0,
                QueuedToast {
                    key,
                    content,
                    timeout: options.timeout,
                    on_close: options.on_close,
                    timer,
                },
            );
        });
        key
    }

    /// Closes a toast.
    pub fn close(&self, key: ToastKey) {
        let Some(toast) = self
            .toasts
            .try_with_untracked(|toasts| toasts.iter().find(|toast| toast.key == key).cloned())
            .flatten()
        else {
            return;
        };
        if let Some(on_close) = toast.on_close {
            let _ = on_close.try_run(());
        }
        self.toasts
            .update(|toasts| toasts.retain(|toast| toast.key != key));
    }

    /// Pauses the timeouts of the visible toasts (while hovered or focused).
    pub fn pause_all(&self) {
        for toast in self.visible_toasts.get_untracked() {
            if let Some(timer) = toast.timer {
                timer.pause();
            }
        }
    }

    pub fn resume_all(&self) {
        for toast in self.visible_toasts.get_untracked() {
            if let Some(timer) = toast.timer {
                timer.resume();
            }
        }
    }

    /// Removes all toasts.
    pub fn clear(&self) {
        self.toasts.set(Vec::new());
    }
}

/// Input of [`use_toast_state`].
#[derive(Debug, Clone, Copy)]
pub struct UseToastStateInput {
    /// How many toasts show at once (react-stately's default: 1).
    pub max_visible_toasts: usize,
}

impl Default for UseToastStateInput {
    fn default() -> Self {
        Self {
            max_visible_toasts: 1,
        }
    }
}

/// A toast queue for one place (react-stately's `useToastState`): showing up to
/// `max_visible_toasts`.
pub fn use_toast_state<T: Clone + Send + Sync + 'static>(
    input: UseToastStateInput,
) -> ToastQueue<T> {
    ToastQueue::new(Some(input.max_visible_toasts))
}

// Timeouts that run out ("should be able to display three toasts and remove the middle toast via
// timeout") need the browser's `setTimeout`: the browser tests cover them (`test_toast.rs`).
#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::with_owner;

    fn contents(queue: ToastQueue<&'static str>) -> Vec<&'static str> {
        queue
            .visible_toasts
            .get_untracked()
            .into_iter()
            .map(|toast| toast.content)
            .collect()
    }

    fn key_of(queue: ToastQueue<&'static str>, index: usize) -> ToastKey {
        queue.visible_toasts.get_untracked()[index].key
    }

    /// "should add a new toast via add": a toast without a timeout has no timer.
    #[test]
    fn adds_a_toast() {
        with_owner(|| {
            let queue = use_toast_state::<&'static str>(UseToastStateInput::default());
            assert_that!(contents(queue)).is_empty();
            let key = queue.add("Toast Message", ToastOptions::default());
            let toasts = queue.visible_toasts.get_untracked();
            assert_that!(toasts.len()).is_equal_to(1);
            assert_that!(toasts[0].content).is_equal_to("Toast Message");
            assert_that!(toasts[0].timeout).is_none();
            assert_that!(toasts[0].timer.is_none()).is_true();
            assert_that!(toasts[0].key).is_equal_to(key);
        });
    }

    /// "should add a new toast with a timer": a toast with a timeout gets a timer (started by the
    /// toast once it shows).
    #[test]
    fn adds_a_toast_with_a_timer() {
        with_owner(|| {
            let queue = use_toast_state::<&'static str>(UseToastStateInput::default());
            queue.add(
                "Test",
                ToastOptions {
                    timeout: Some(Duration::from_secs(5)),
                    ..ToastOptions::default()
                },
            );
            let toasts = queue.visible_toasts.get_untracked();
            assert_that!(toasts.len()).is_equal_to(1);
            assert_that!(toasts[0].content).is_equal_to("Test");
            assert_that!(toasts[0].timeout).is_equal_to(Some(Duration::from_secs(5)));
            assert_that!(toasts[0].timer.is_some()).is_true();
        });
    }

    /// "should be able to add multiple toasts": the newest shows first.
    #[test]
    fn adds_several_toasts() {
        with_owner(|| {
            let queue = use_toast_state::<&'static str>(UseToastStateInput {
                max_visible_toasts: 2,
            });
            queue.add("Toast Message", ToastOptions::default());
            assert_that!(contents(queue)).is_equal_to(vec!["Toast Message"]);
            queue.add("Second Toast", ToastOptions::default());
            assert_that!(contents(queue)).is_equal_to(vec!["Second Toast", "Toast Message"]);
        });
    }

    /// "should be able to display one toast, add multiple toasts, and remove the middle not visible
    /// one programmatically": closing a queued toast leaves the visible one, and the one before it
    /// shows next.
    #[test]
    fn closes_a_queued_toast() {
        with_owner(|| {
            let queue = use_toast_state::<&'static str>(UseToastStateInput::default());
            queue.add("First Toast", ToastOptions::default());
            assert_that!(contents(queue)).is_equal_to(vec!["First Toast"]);
            let second = queue.add("Second Toast", ToastOptions::default());
            assert_that!(contents(queue)).is_equal_to(vec!["Second Toast"]);
            queue.add("Third Toast", ToastOptions::default());
            assert_that!(contents(queue)).is_equal_to(vec!["Third Toast"]);
            queue.close(second);
            assert_that!(contents(queue)).is_equal_to(vec!["Third Toast"]);
            queue.close(key_of(queue, 0));
            assert_that!(contents(queue)).is_equal_to(vec!["First Toast"]);
        });
    }

    /// "should be able to display one toast, add multiple toasts": by default only the newest
    /// toast shows.
    #[test]
    fn shows_one_toast_by_default() {
        with_owner(|| {
            let queue = use_toast_state::<&'static str>(UseToastStateInput::default());
            for toast in ["First Toast", "Second Toast", "Third Toast"] {
                queue.add(toast, ToastOptions::default());
                assert_that!(contents(queue)).is_equal_to(vec![toast]);
            }
        });
    }

    /// "should maintain the toast queue order on close": closing the middle of three visible
    /// toasts keeps the others' order.
    #[test]
    fn keeps_the_order_on_close() {
        with_owner(|| {
            let queue = use_toast_state::<&'static str>(UseToastStateInput {
                max_visible_toasts: 3,
            });
            queue.add("First Toast", ToastOptions::default());
            queue.add("Second Toast", ToastOptions::default());
            queue.add("Third Toast", ToastOptions::default());
            assert_that!(contents(queue)).is_equal_to(vec![
                "Third Toast",
                "Second Toast",
                "First Toast",
            ]);
            queue.close(key_of(queue, 1));
            assert_that!(contents(queue)).is_equal_to(vec!["Third Toast", "First Toast"]);
        });
    }

    /// "should close a toast".
    #[test]
    fn closes_a_toast() {
        with_owner(|| {
            let queue = use_toast_state::<&'static str>(UseToastStateInput::default());
            queue.add("Toast Message", ToastOptions::default());
            queue.close(key_of(queue, 0));
            assert_that!(contents(queue)).is_empty();
        });
    }

    /// "should queue toasts": a closed toast makes room for the one before it.
    #[test]
    fn queues_toasts() {
        with_owner(|| {
            let queue = use_toast_state::<&'static str>(UseToastStateInput::default());
            queue.add("Toast Message", ToastOptions::default());
            assert_that!(contents(queue)).is_equal_to(vec!["Toast Message"]);
            queue.add("Second Toast", ToastOptions::default());
            assert_that!(contents(queue)).is_equal_to(vec!["Second Toast"]);
            queue.close(key_of(queue, 0));
            assert_that!(contents(queue)).is_equal_to(vec!["Toast Message"]);
        });
    }

    /// A queue showing two toasts shows the newest two; closing one shows the next and calls its
    /// `on_close` once.
    #[test]
    fn shows_the_newest_toasts_and_closes_them() {
        with_owner(|| {
            let closed = RwSignal::new(0);
            let queue = ToastQueue::<&'static str>::new(Some(2));
            queue.add("first", ToastOptions::default());
            let second = queue.add(
                "second",
                ToastOptions {
                    on_close: Some(Callback::new(move |()| {
                        closed.update(|closed| *closed += 1);
                    })),
                    ..ToastOptions::default()
                },
            );
            queue.add("third", ToastOptions::default());
            assert_that!(contents(queue)).is_equal_to(vec!["third", "second"]);
            queue.close(second);
            assert_that!(contents(queue)).is_equal_to(vec!["third", "first"]);
            assert_that!(closed.get_untracked()).is_equal_to(1);
        });
    }
}
