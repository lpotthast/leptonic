// Upstream: react-stately/src/toast/useToastState.ts @ 99e6102368
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
// - Timeouts are `Duration`s; keys count up (react-aria: random strings).
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

fn now() -> f64 {
    #[cfg(not(feature = "ssr"))]
    {
        js_sys::Date::now()
    }
    #[cfg(feature = "ssr")]
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

/// A toast in the queue.
#[derive(Debug, Clone)]
pub struct QueuedToast<T> {
    pub key: String,
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
    pub fn add(&self, content: T, options: ToastOptions) -> String {
        let number = self.next_key.get_value() + 1;
        self.next_key.set_value(number);
        let key = format!("toast-{number}");
        let queue = *self;
        let closing = key.clone();
        let timer = options
            .timeout
            .map(|timeout| ToastTimer::new(move || queue.close(&closing), timeout));
        self.toasts.update(|toasts| {
            toasts.insert(
                0,
                QueuedToast {
                    key: key.clone(),
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
    pub fn close(&self, key: &str) {
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

/// A toast queue for one place (react-stately's `useToastState`): showing up to
/// `max_visible_toasts` (default 1).
pub fn use_toast_state<T: Clone + Send + Sync + 'static>(
    max_visible_toasts: Option<usize>,
) -> ToastQueue<T> {
    ToastQueue::new(Some(max_visible_toasts.unwrap_or(1)))
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn shows_the_newest_toasts_and_closes_them() {
        let owner = Owner::new();
        owner.with(|| {
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
            let visible = || {
                queue
                    .visible_toasts
                    .get_untracked()
                    .into_iter()
                    .map(|toast| toast.content)
                    .collect::<Vec<_>>()
            };
            assert_that!(visible()).is_equal_to(vec!["third", "second"]);
            queue.close(&second);
            assert_that!(visible()).is_equal_to(vec!["third", "first"]);
            assert_that!(closed.get_untracked()).is_equal_to(1);
        });
    }
}
