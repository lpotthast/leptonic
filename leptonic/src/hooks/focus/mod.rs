use std::sync::{atomic::{AtomicBool, Ordering}, Mutex};

use leptos_use::UseEventListenerOptions;
use wasm_bindgen::JsCast;
use leptos::{ev::{beforeunload, blur, click, focus, keydown, keyup, pointerdown, pointermove, pointerup}, leptos_dom::is_server};
use web_sys::{BeforeUnloadEvent, FocusEvent, KeyboardEvent, MouseEvent, PointerEvent};

use crate::utils::{platform::is_mac, MouseEventExt};

pub mod use_focus;

// TODO: focus_safely: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/focus/src/focusSafely.ts

// TODO: Removing the Mutex should increase performance, as get_global_modality, accessing this with locking, is called quite often!
static MODALITY: Mutex<Option<Modality>> = Mutex::new(None);
static HAS_EVENT_BEFORE_FOCUS: AtomicBool = AtomicBool::new(false);
static HAS_BLURRED_WINDOW_RECENTLY: AtomicBool = AtomicBool::new(false);
static CHANGE_HANDLERS: Mutex<Vec<Box<dyn Fn(Modality, &ModalityChangedBy) + Send + Sync + 'static>>> = Mutex::new(Vec::new());

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modality {
    Keyboard,
    Pointer,
    Virtual,
}

pub fn set_global_modality(modality: Option<Modality>) {
    *MODALITY.lock().expect("not poisoned") = modality
}

pub fn get_global_modality() -> Option<Modality> {
    *MODALITY.lock().expect("not poisoned")
}

/// Wether focus is visible.
pub fn is_focus_visible() -> bool {
    get_global_modality() == Some(Modality::Keyboard)
}

fn is_valid_key(e: &KeyboardEvent) -> bool {
  if e.meta_key() || (!is_mac() && e.alt_key()) || e.ctrl_key() {
    return false;
  }

  // Control and Shift keys trigger when navigating back to the tab with keyboard.
  let key = e.key();
  if key == "Control" || key == "Shift" || key == "Meta" {
    return false;
  }

  true
}

#[derive(Debug, Clone)]
pub enum ModalityChangedBy {
    KeyboardEvent(KeyboardEvent),
    PointerEvent(PointerEvent),
    FocusEvent(FocusEvent),
}

fn trigger_change_handlers(new_global_modality: Modality, changed_by: ModalityChangedBy) {
    for handler in &(*CHANGE_HANDLERS.lock().expect("non-poisoned")) {
        handler(new_global_modality, &changed_by);
    }
}

/// Setup global event listeners to control when keyboard focus style should be visible.
/// NOTE: Make sure that the page this element is contained in is fully loaded.
pub fn setup_global_focus_events(element: web_sys::HtmlElement) {
    if is_server() {
        return;
    }

    // `owner_document` should only return `None` when the element is itself a document!
    let document = element.owner_document().unwrap_or_else(|| element.unchecked_into());
    let window = match document.default_view() {
        Some(window) => window,
        None => {
            tracing::warn!("Cannot set up global focus events. Document had no window (default_view)! This may be the case while the page is still loading, when running in a worker context or when running in an old browser.");
            // TODO: Consider fallback to `window()`.
            return;
        },
    };

    // TODO: Do not execute this twice for any given `(window, document)` tuple!

    // Programmatic focus() calls shouldn't affect the current input modality.
    // However, we need to detect other cases when a focus event occurs without
    // a preceding user event (e.g. screen reader focus). Overriding the focus
    // method on HTMLElement.prototype is a bit hacky, but works.
    /*let focus = window.HTMLElement.prototype.focus;
    window.HTMLElement.prototype.focus = function () {
        HAS_EVENT_BEFORE_FOCUS.store(true, Ordering::Release);
        focus.apply(this, arguments as unknown as [options?: FocusOptions | undefined]);
    };*/

    let handle_keyboard_event = move |e: KeyboardEvent| {
        HAS_EVENT_BEFORE_FOCUS.store(true, Ordering::Release);
        if is_valid_key(&e) {
            set_global_modality(Some(Modality::Keyboard));
            trigger_change_handlers(Modality::Keyboard, ModalityChangedBy::KeyboardEvent(e));
        }
    };

    let handle_pointer_event = move |e: PointerEvent| {
        set_global_modality(Some(Modality::Pointer));
        let ty = e.type_();
        if ty == "mousedown" || ty == "pointerdown" {
            HAS_EVENT_BEFORE_FOCUS.store(true, Ordering::Release);
            trigger_change_handlers(Modality::Pointer, ModalityChangedBy::PointerEvent(e));
        }
    };

    let handle_click_event = move |e: MouseEvent| {
        if e.is_virtual() {
            HAS_EVENT_BEFORE_FOCUS.store(true, Ordering::Release);
            set_global_modality(Some(Modality::Virtual));
        }
    };

    let window_clone = window.clone();
    let document_clone = document.clone();
    let handle_focus_event = move |e: FocusEvent| {
        // Firefox fires two extra focus events when the user first clicks into an iframe:
        // first on the window, then on the document. We ignore these events so they don't
        // cause keyboard focus rings to appear.
        let target = e.target();
        if let Some(target) = target {
            if target.loose_eq(&window_clone) || target.loose_eq(&document_clone) {
                return;
            }
        }

        // If a focus event occurs without a preceding keyboard or pointer event, switch to virtual modality.
        // This occurs, for example, when navigating a form with the next/previous buttons on iOS.
        if !HAS_EVENT_BEFORE_FOCUS.load(Ordering::Acquire) && !HAS_BLURRED_WINDOW_RECENTLY.load(Ordering::Acquire) {
            set_global_modality(Some(Modality::Virtual));
            trigger_change_handlers(Modality::Virtual, ModalityChangedBy::FocusEvent(e));
        }

        HAS_EVENT_BEFORE_FOCUS.store(false, Ordering::Release);
        HAS_BLURRED_WINDOW_RECENTLY.store(false, Ordering::Release);
    };

    let handle_window_blur = move |_e: FocusEvent| {
        // When the window is blurred, reset state. This is necessary when tabbing out of the window,
        // for example, since a subsequent focus event won't be fired.
        HAS_EVENT_BEFORE_FOCUS.store(false, Ordering::Release);
        HAS_BLURRED_WINDOW_RECENTLY.store(true, Ordering::Release);
    };

    let stop_keydown = leptos_use::use_event_listener_with_options(document.clone(), keydown, handle_keyboard_event, UseEventListenerOptions::default().capture(true));
    let stop_keyup = leptos_use::use_event_listener_with_options(document.clone(), keyup, handle_keyboard_event, UseEventListenerOptions::default().capture(true));
    let stop_click = leptos_use::use_event_listener_with_options(document.clone(), click, handle_click_event, UseEventListenerOptions::default().capture(true));

    // TODO: This comment does not apply, we use leptos! Do we really need access to `window`?
    // Register focus events on the window so they are sure to happen
    // before React's event listeners (registered on the document).

    let stop_focus = leptos_use::use_event_listener_with_options(window.clone(), focus, handle_focus_event, UseEventListenerOptions::default().capture(true));
    let stop_blur = leptos_use::use_event_listener_with_options(window.clone(), blur, handle_window_blur, UseEventListenerOptions::default().capture(false));

    let stop_pointerdown = leptos_use::use_event_listener_with_options(document.clone(), pointerdown, handle_pointer_event, UseEventListenerOptions::default().capture(true));
    let stop_pointermove = leptos_use::use_event_listener_with_options(document.clone(), pointermove, handle_pointer_event, UseEventListenerOptions::default().capture(true));
    let stop_pointerup = leptos_use::use_event_listener_with_options(document.clone(), pointerup, handle_pointer_event, UseEventListenerOptions::default().capture(true));

    // Add unmount handler
    leptos_use::use_event_listener_with_options(window, beforeunload, move |e: BeforeUnloadEvent| {
        stop_keydown();
        stop_keyup();
        stop_click();
        stop_focus();
        stop_blur();
        stop_pointerdown();
        stop_pointermove();
        stop_pointerup();
    }, UseEventListenerOptions::default().once(true));

    // hasSetupGlobalListeners.set(window, {focus});
  }

  /*
  const tearDownWindowFocusTracking = (element, loadListener?: () => void) => {
    const windowObject = getOwnerWindow(element);
    const documentObject = getOwnerDocument(element);
    if (loadListener) {
      documentObject.removeEventListener('DOMContentLoaded', loadListener);
    }
    if (!hasSetupGlobalListeners.has(windowObject)) {
      return;
    }
    windowObject.HTMLElement.prototype.focus = hasSetupGlobalListeners.get(windowObject)!.focus;

    documentObject.removeEventListener('keydown', handleKeyboardEvent, true);
    documentObject.removeEventListener('keyup', handleKeyboardEvent, true);
    documentObject.removeEventListener('click', handleClickEvent, true);
    windowObject.removeEventListener('focus', handleFocusEvent, true);
    windowObject.removeEventListener('blur', handleWindowBlur, false);

    if (typeof PointerEvent !== 'undefined') {
      documentObject.removeEventListener('pointerdown', handlePointerEvent, true);
      documentObject.removeEventListener('pointermove', handlePointerEvent, true);
      documentObject.removeEventListener('pointerup', handlePointerEvent, true);
    } else {
      documentObject.removeEventListener('mousedown', handlePointerEvent, true);
      documentObject.removeEventListener('mousemove', handlePointerEvent, true);
      documentObject.removeEventListener('mouseup', handlePointerEvent, true);
    }

    hasSetupGlobalListeners.delete(windowObject);
  };
   */

  /*
   * EXPERIMENTAL
   * Adds a window (i.e. iframe) to the list of windows that are being tracked for focus visible.
   *
   * Sometimes apps render portions of their tree into an iframe. In this case, we cannot accurately track if the focus
   * is visible because we cannot see interactions inside the iframe. If you have this in your application's architecture,
   * then this function will attach event listeners inside the iframe. You should call `addWindowFocusTracking` with an
   * element from inside the window you wish to add. We'll retrieve the relevant elements based on that.
   * Note, you do not need to call this for the default window, as we call it for you.
   *
   * When you are ready to stop listening, but you do not wish to unmount the iframe, you may call the cleanup function
   * returned by `addWindowFocusTracking`. Otherwise, when you unmount the iframe, all listeners and state will be cleaned
   * up automatically for you.
   *
   * @param element @default document.body - The element provided will be used to get the window to add.
   * @returns A function to remove the event listeners and cleanup the state.
   */
  /*
  export function addWindowFocusTracking(element?: HTMLElement | null): () => void {
    const documentObject = getOwnerDocument(element);
    let loadListener;
    if (documentObject.readyState !== 'loading') {
      setupGlobalFocusEvents(element);
    } else {
      loadListener = () => {
        setupGlobalFocusEvents(element);
      };
      documentObject.addEventListener('DOMContentLoaded', loadListener);
    }

    return () => tearDownWindowFocusTracking(element, loadListener);
  }
  */