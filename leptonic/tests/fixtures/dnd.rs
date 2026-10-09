use std::time::Duration;

use assertr::{matchers::eq, prelude::*};
use browser_test::{StepExt, thirtyfour::prelude::*};
use rootcause::Report;
use serde::Deserialize;

use crate::pages::{DragKind, ElementActions, Modifier, Page, SyntheticEvent, health};

/// Script defining `createTransfer(effectAllowed)`: a stand-in for a drag's `DataTransfer` (as
/// react-aria's tests mock it), since a constructed `DataTransfer` ignores `effectAllowed`
/// writes, has no drag image to inspect and can't hold directories. Its items list is an array
/// (`length`, indexing) with `add(data, type)`; files and directories come with
/// `webkitGetAsEntry()` entries.
const TRANSFER: &str = "
    const fileEntry = file => ({
        isFile: true, isDirectory: false, name: file.name, file: ok => ok(file),
    });
    const directoryEntry = (name, entries) => ({
        isFile: false, isDirectory: true, name,
        createReader() {
            let rest = entries;
            return { readEntries(ok) { const batch = rest; rest = []; ok(batch); } };
        },
    });
    const transferItem = (kind, type, data) => ({
        kind, type, _data: data,
        getAsString(callback) { if (kind === 'string') callback(data); },
        getAsFile() { return kind === 'file' && data.isFile ? data._file : null; },
        webkitGetAsEntry() { return kind === 'file' ? data : null; },
    });
    const createTransfer = effectAllowed => {
        const items = [];
        items.add = (data, type) => {
            items.push(data instanceof File
                ? transferItem('file', data.type, { ...fileEntry(data), _file: data })
                : transferItem('string', type, data));
        };
        items.clear = () => { items.splice(0); };
        items.remove = index => { items.splice(index, 1); };
        return {
            items, effectAllowed, dropEffect: 'none', dragImage: null,
            get types() { return [...new Set(items.map(i => i.kind === 'file' ? 'Files' : i.type))]; },
            getData(type) {
                const found = items.find(i => i.kind === 'string' && i.type === type);
                return found ? found._data : '';
            },
            setData(type, data) { this.clearData(type); items.add(data, type); },
            clearData(type) {
                const keep = items.filter(i => type !== undefined && i.type !== type);
                items.splice(0, items.length, ...keep);
            },
            setDragImage(element, x, y) { this.dragImage = { text: element.textContent, x, y }; },
        };
    };
    const externalItem = ([kind, ...data]) => {
        if (kind === 'text') return transferItem('string', data[0], data[1]);
        if (kind === 'file') {
            const file = new File([data[2]], data[0], { type: data[1] });
            return transferItem('file', file.type, { ...fileEntry(file), _file: file });
        }
        return transferItem('file', '', externalEntry([kind, ...data]));
    };
    const externalEntry = ([kind, ...data]) => {
        if (kind === 'file') {
            const file = new File([data[2]], data[0], { type: data[1] });
            return { ...fileEntry(file), _file: file };
        }
        return directoryEntry(data[0], data[1].map(externalEntry));
    };";

/// A script: fires the drag events `arguments[1]` (types, in one task) at `arguments[0]` (or, if
/// `null`, at the element of the last `dragstart`, even once removed), at its center or at the
/// offset `arguments[3]` from its top left corner, with the modifier keys `arguments[2]` (init
/// members such as `altKey`) and the page's current transfer (`window.__dndTransfer`; `dragstart`
/// creates a new one). Returns whether each event's default was prevented.
const FIRE_DRAG_EVENTS: &str = "
    const [target, types, modifiers, at] = arguments;
    const results = [];
    for (const type of types) {
        if (type === 'dragstart') {
            window.__dndTransfer = createTransfer('uninitialized');
            window.__dndSource = target;
        }
        const element = target ?? window.__dndSource;
        const rect = element.getBoundingClientRect();
        const event = new DragEvent(type, {
            // DragEvent coordinates are integers; round instead of truncating fractional layout
            // positions, so the requested offset is represented by its nearest screen pixel.
            clientX: Math.round(rect.left + (at ? at[0] : rect.width / 2)),
            clientY: Math.round(rect.top + (at ? at[1] : rect.height / 2)),
            bubbles: true,
            cancelable: true,
            composed: true,
            ...Object.fromEntries(modifiers.map(member => [member, true])),
        });
        Object.defineProperty(event, 'dataTransfer', { value: window.__dndTransfer });
        element.dispatchEvent(event);
        results.push(event.defaultPrevented);
    }
    return results;";

/// Data a drag from outside the page carries (another application, the desktop).
#[derive(Debug, Clone)]
pub enum ExternalData<'a> {
    /// Text of the MIME type `kind`.
    Text { kind: &'a str, data: &'a str },
    /// A file of the MIME type `kind` (`""`: unknown).
    File {
        name: &'a str,
        kind: &'a str,
        content: &'a str,
    },
    /// A directory of files and directories.
    Directory {
        name: &'a str,
        entries: Vec<ExternalData<'a>>,
    },
}

impl ExternalData<'_> {
    fn to_json(&self) -> serde_json::Value {
        match self {
            Self::Text { kind, data } => serde_json::json!(["text", kind, data]),
            Self::File {
                name,
                kind,
                content,
            } => serde_json::json!(["file", name, kind, content]),
            Self::Directory { name, entries } => serde_json::json!([
                "directory",
                name,
                entries.iter().map(Self::to_json).collect::<Vec<_>>()
            ]),
        }
    }
}

/// The current native drag's transfer, as the page's handlers left it.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Transfer {
    /// The text items: (type, data), in order.
    pub data: Vec<(String, String)>,
    /// `types`: the text items' types, `Files` for files.
    pub types: Vec<String>,
    #[serde(rename = "dropEffect")]
    pub drop_effect: String,
    #[serde(rename = "effectAllowed")]
    pub effect_allowed: String,
    /// What `setDragImage` set.
    #[serde(rename = "dragImage")]
    pub drag_image: Option<DragImage>,
}

/// A drag image: the element's text and the pointer's offset in it.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DragImage {
    pub text: String,
    pub x: f64,
    pub y: f64,
}

/// Actions on the drag and drop fixtures (`/hooks/dnd`, `/hooks/dnd-targets`,
/// `/hooks/dnd-collection`): event logs, the fixture's mid-drag actions, and native drags
/// synthesized with a `DataTransfer` and `DragEvent`s.
pub struct DndActions<'a, 'd> {
    page: &'a Page<'d>,
}

impl<'a, 'd> DndActions<'a, 'd> {
    pub fn new(page: &'a Page<'d>) -> Self {
        Self { page }
    }
    /// The entries of the log list `#<id>`.
    pub async fn log(&self, id: &str) -> Result<Vec<String>, Report> {
        health::expect_no_panic(self.page.low_level().driver()).await?;
        self.page.inner_texts(format!("#{id} li")).await
    }

    /// Wait for the log, reading once per observation.
    #[track_caller]
    pub fn wait_for_log<'b>(
        &'b self,
        id: &'b str,
        expected: &'b [&'b str],
    ) -> impl Future<Output = Result<(), Report>> + 'b {
        let check = assert_that_owned!(move || self.log(id))
            .with_subject_name(format!("log #{id}"))
            .eventually_ok()
            .giving_up_on(crate::pages::terminal_lookup_error)
            .try_matches(eq(expected));
        async move {
            if let Err(error) = check.await {
                let observed = self.log(id).await?;
                let error: Report = error.into();
                return Err(error
                    .context(format!(
                        "Expected log: {expected:?}; observed log: {observed:?}"
                    ))
                    .into_dynamic());
            }
            Ok(())
        }
        .step("wait_for_log")
    }

    /// Sample the log immediately and throughout the specified duration.
    #[track_caller]
    pub fn log_stays<'b>(
        &'b self,
        id: &'b str,
        expected: &'b [&'b str],
        duration: Duration,
    ) -> impl Future<Output = Result<(), Report>> + 'b {
        let check = assert_that_owned!(move || self.log(id))
            .with_subject_name(format!("log #{id}"))
            .consistently_ok()
            .for_at_least(duration)
            .try_matches(eq(expected));
        async move {
            if duration.is_zero() {
                rootcause::bail!("stability checks require a positive duration");
            }
            check.await?;
            Ok(())
        }
        .step("log_stays")
    }

    /// Dispatches the fixture's custom event `event` with the `action` as its detail on the
    /// element `#<target>`.
    pub async fn dispatch_action(
        &self,
        target: &str,
        event: &str,
        action: &str,
    ) -> Result<(), Report> {
        self.page
            .element(format!("#{target}"))
            .await?
            .dispatch(SyntheticEvent::custom(event, action).bubbles(false))
            .await?;
        Ok(())
    }

    /// Starts a drag from outside the page carrying `data`: the transfer later drag events
    /// carry (`effectAllowed` is `all`).
    pub async fn begin_external_drag(&self, data: &[ExternalData<'_>]) -> Result<(), Report> {
        let data: Vec<serde_json::Value> = data.iter().map(ExternalData::to_json).collect();
        let _: serde_json::Value = self
            .page
            .low_level()
            .eval(
                &format!(
                    "{TRANSFER} const transfer = createTransfer('all');
                    transfer.items.push(...arguments[0].map(externalItem));
                    window.__dndTransfer = transfer;
                    return null;"
                ),
                vec![serde_json::Value::Array(data)],
            )
            .await?;
        Ok(())
    }

    /// Fires the native drag event `kind` (`dragstart` creates a new transfer) at the center of
    /// `element`. Returns whether its default was prevented.
    pub async fn fire_drag_event(
        &self,
        element: &WebElement,
        kind: DragKind,
        modifiers: &[Modifier],
    ) -> Result<bool, Report> {
        self.fire_drag_event_at(element, kind, modifiers, None)
            .await
    }

    /// [`fire_drag_event`](Self::fire_drag_event) at `at` (x, y) from the element's top left
    /// corner instead of its center.
    pub async fn fire_drag_event_at(
        &self,
        element: &WebElement,
        kind: DragKind,
        modifiers: &[Modifier],
        at: Option<(f64, f64)>,
    ) -> Result<bool, Report> {
        let prevented = self.fire(Some(element), &[kind], modifiers, at).await?;
        Ok(prevented.first().copied().unwrap_or(false))
    }

    /// Fires the native drag events `kinds` at the center of `element` in one task, as the
    /// browser fires events no frame or timer separates.
    pub async fn fire_drag_events(
        &self,
        element: &WebElement,
        kinds: &[DragKind],
    ) -> Result<(), Report> {
        self.fire(Some(element), kinds, &[], None).await?;
        Ok(())
    }

    /// Fires the native drag event `kind` at the element the last `dragstart` was fired at, also
    /// once it was removed from the page (browsers keep sending `drag` and `dragend` to it).
    pub async fn fire_at_drag_source(&self, kind: DragKind) -> Result<(), Report> {
        self.fire(None, &[kind], &[], None).await?;
        Ok(())
    }

    async fn fire(
        &self,
        element: Option<&WebElement>,
        kinds: &[DragKind],
        modifiers: &[Modifier],
        at: Option<(f64, f64)>,
    ) -> Result<Vec<bool>, Report> {
        let modifiers: Vec<serde_json::Value> = modifiers
            .iter()
            .map(|modifier| modifier.init_member().into())
            .collect();
        let kinds: Vec<serde_json::Value> = kinds.iter().map(|kind| kind.as_str().into()).collect();
        self.page
            .low_level()
            .eval(
                &format!("{TRANSFER} {FIRE_DRAG_EVENTS}"),
                vec![
                    element.map_or(Ok(serde_json::Value::Null), WebElement::to_json)?,
                    serde_json::Value::Array(kinds),
                    serde_json::Value::Array(modifiers),
                    at.map_or(serde_json::Value::Null, |(x, y)| serde_json::json!([x, y])),
                ],
            )
            .await
    }

    /// The current native drag's transfer.
    pub async fn transfer(&self) -> Result<Transfer, Report> {
        self.page
            .low_level()
            .eval(
                "const transfer = window.__dndTransfer;
                return {
                    data: transfer.items.filter(i => i.kind === 'string').map(i => [i.type, i._data]),
                    types: transfer.types,
                    dropEffect: transfer.dropEffect,
                    effectAllowed: transfer.effectAllowed,
                    dragImage: transfer.dragImage,
                };",
                vec![],
            )
            .await
    }

    /// Sets the current native drag's `effectAllowed`, as browsers do when a modifier key is
    /// pressed (or WebKit does wrongly).
    pub async fn set_effect_allowed(&self, effect_allowed: &str) -> Result<(), Report> {
        let _: serde_json::Value = self
            .page
            .low_level()
            .eval(
                "window.__dndTransfer.effectAllowed = arguments[0]; return null;",
                vec![effect_allowed.into()],
            )
            .await?;
        Ok(())
    }
}
