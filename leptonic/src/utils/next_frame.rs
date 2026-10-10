// No upstream: react-aria does this work in place (in the `ResizeObserver` callback).

use leptos::prelude::*;

/// Runs a piece of work in the next animation frame, once however often it was requested
/// before then. A pending run is cancelled by [`NextFrame::cancel`] and when the owner that
/// created it is cleaned up.
///
/// For work in a `ResizeObserver` callback that resizes an element not deeper in the DOM than an
/// observed one (repositioning an overlay, measuring a size the overlay is styled with): done in
/// the callback, the new size can't be delivered in the same pass, and the browser reports
/// "ResizeObserver loop completed with undelivered notifications" as an uncaught error. Done in
/// the next frame, it is still applied before that frame is painted.
#[derive(Debug, Clone, Copy)]
pub(crate) struct NextFrame {
    work: Callback<()>,
    pending: StoredValue<Option<AnimationFrameRequestHandle>>,
}

impl NextFrame {
    pub(crate) fn new(work: impl Fn() + Send + Sync + 'static) -> Self {
        let this = Self {
            work: Callback::new(move |()| work()),
            pending: StoredValue::new(None),
        };
        on_cleanup(move || this.cancel());
        this
    }

    /// Runs the work in the next animation frame, unless a run is pending already.
    pub(crate) fn request(self) {
        if self.pending.try_with_value(Option::is_none) != Some(true) {
            return;
        }
        let handle = request_animation_frame_with_handle(move || {
            // Nothing to do once the owner is gone.
            if self.pending.try_update_value(Option::take).is_some() {
                let _ = self.work.try_run(());
            }
        })
        .ok();
        self.pending.set_value(handle);
    }

    /// Cancels a pending run.
    pub(crate) fn cancel(self) {
        if let Some(Some(handle)) = self.pending.try_update_value(Option::take) {
            handle.cancel();
        }
    }
}
