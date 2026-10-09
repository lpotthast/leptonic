//! The virtual list fixture (`/atoms/virtual-list`): a log (`#test-vl-log`) whose rows render a
//! `.line` each, inside the row's wrapper (positioned at its layout info) and content wrapper.
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;
use serde::Deserialize;

use crate::pages::{Page, ScrollExtent};

/// `wrapper(line)`: the positioned row wrapper of a `.line` (row wrapper > content wrapper >
/// line).
const WRAPPER: &str = "const wrapper = line => line.parentElement.parentElement;";

/// A rendered row of the log.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Row {
    /// The line's text.
    pub text: String,
    /// The row's position and height in the content (its wrapper's `top` and `height`).
    pub top: f64,
    pub height: f64,
}

/// The rendered rows (in DOM order) and the log's scroll position, read at once.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct LogView {
    pub rows: Vec<Row>,
    pub extent: ScrollExtent,
}

impl LogView {
    /// The rows' texts, in DOM order.
    pub fn texts(&self) -> Vec<String> {
        self.rows.iter().map(|row| row.text.clone()).collect()
    }

    /// The rows' tops, in DOM order.
    pub fn tops(&self) -> Vec<f64> {
        self.rows.iter().map(|row| row.top).collect()
    }

    /// Whether the rendered rows cover the visible part of the log.
    pub fn covers_the_view(&self) -> bool {
        let top = self.rows.iter().map(|row| row.top).reduce(f64::min);
        let bottom = self
            .rows
            .iter()
            .map(|row| row.top + row.height)
            .reduce(f64::max);
        matches!(
            (top, bottom),
            (Some(top), Some(bottom))
                if top <= self.extent.top && bottom >= self.extent.top + self.extent.client_height
        )
    }

    /// The first row in view and its offset from the view's top, e.g. `Line 1234@-3`.
    pub fn anchor(&self) -> Option<String> {
        self.rows
            .iter()
            .filter(|row| row.top >= self.extent.top)
            .min_by(|a, b| a.top.total_cmp(&b.top))
            .map(|row| {
                let text: String = row.text.chars().take(12).collect();
                format!("{text}@{}", row.top - self.extent.top)
            })
    }
}

/// Actions on the virtual list fixture's log.
pub struct VirtualListActions<'a, 'd> {
    page: &'a Page<'d>,
}

impl<'a, 'd> VirtualListActions<'a, 'd> {
    pub fn new(page: &'a Page<'d>) -> Self {
        Self { page }
    }

    /// The log element (it scrolls).
    pub async fn log(&self) -> Result<WebElement, Report> {
        self.page.element("#test-vl-log").await
    }

    /// The rendered rows and the scroll position.
    pub async fn view(&self) -> Result<LogView, Report> {
        let log = self.log().await?;
        self.page
            .low_level()
            .eval(
                &format!(
                    "{WRAPPER}
                     const log = arguments[0];
                     return {{
                         rows: Array.from(log.querySelectorAll('.line')).map(line => ({{
                             text: line.textContent,
                             top: parseFloat(wrapper(line).style.top),
                             height: parseFloat(wrapper(line).style.height),
                         }})),
                         extent: {{
                             top: log.scrollTop,
                             client_height: log.clientHeight,
                             scroll_height: log.scrollHeight,
                         }},
                     }};"
                ),
                vec![log.to_json()?],
            )
            .await
    }

    /// Selects the contents of the rendered line `text`; returns once the document dispatched
    /// the `selectionchange` (the list's listener ran by then).
    pub async fn select_line(&self, text: &str) -> Result<(), Report> {
        let log = self.log().await?;
        self.page
            .low_level()
            .eval_async(
                "const [log, text, done] = arguments;
                 const line = Array.from(log.querySelectorAll('.line'))
                     .find(l => l.textContent === text);
                 document.addEventListener('selectionchange', () => setTimeout(done), {once: true});
                 const range = document.createRange();
                 range.selectNodeContents(line);
                 const selection = window.getSelection();
                 selection.removeAllRanges();
                 selection.addRange(range);",
                vec![log.to_json()?, text.into()],
            )
            .await
    }

    /// Selects from the first to the third rendered short line ("Line <n>") in visual order;
    /// returns the selection's text (whitespace collapsed) and the texts of its first and last
    /// line.
    pub async fn select_three_short_lines(&self) -> Result<(String, [String; 2]), Report> {
        let log = self.log().await?;
        self.page
            .low_level()
            .eval(
                &format!(
                    "{WRAPPER}
                     const lines = Array.from(arguments[0].querySelectorAll('.line'))
                         .filter(l => /^Line \\d+$/.test(l.textContent))
                         .sort((a, b) => parseFloat(wrapper(a).style.top)
                             - parseFloat(wrapper(b).style.top));
                     const range = document.createRange();
                     range.setStartBefore(lines[0]);
                     range.setEndAfter(lines[2]);
                     return [range.toString().replace(/\\s+/g, ' ').trim(),
                         [lines[0].textContent, lines[2].textContent]];"
                ),
                vec![log.to_json()?],
            )
            .await
    }

    /// The document's selected text.
    pub async fn selection_text(&self) -> Result<String, Report> {
        self.page
            .low_level()
            .eval("return window.getSelection().toString();", vec![])
            .await
    }

    /// Removes the document's selection.
    pub async fn clear_selection(&self) -> Result<(), Report> {
        self.page
            .low_level()
            .eval("window.getSelection().removeAllRanges();", vec![])
            .await
    }

    /// Makes the page taller than the window and scrolls it down 1000px, so the append button
    /// is out of view (a click scrolls the page to it first).
    pub async fn scroll_the_page_down(&self) -> Result<(), Report> {
        self.page
            .low_level()
            .eval(
                "document.body.style.minHeight = '300vh'; window.scrollBy(0, 1000);",
                vec![],
            )
            .await
    }

    /// Starts recording the rows whose height goes from a measured one back to the 20px
    /// estimate (see [`Self::reestimated_rows`]).
    pub async fn record_reestimated_rows(&self) -> Result<(), Report> {
        let log = self.log().await?;
        self.page
            .low_level()
            .eval(
                "const log = arguments[0];
                 window.__vlReestimated = [];
                 new MutationObserver(records => {
                     for (const record of records) {
                         const wrapper = record.target;
                         if (!wrapper.querySelector(':scope > * > .line')) continue;
                         const old = /height: ([0-9.]+)px/.exec(record.oldValue ?? '')?.[1];
                         if (old !== undefined && old !== '20' && wrapper.style.height === '20px') {
                             window.__vlReestimated.push(wrapper.textContent.slice(0, 12));
                         }
                     }
                 }).observe(log, {subtree: true, attributes: true, attributeFilter: ['style'],
                     attributeOldValue: true});",
                vec![log.to_json()?],
            )
            .await
    }

    /// The rows recorded since [`Self::record_reestimated_rows`] (their first 12 characters).
    pub async fn reestimated_rows(&self) -> Result<Vec<String>, Report> {
        self.page
            .low_level()
            .eval("return window.__vlReestimated;", vec![])
            .await
    }
}
