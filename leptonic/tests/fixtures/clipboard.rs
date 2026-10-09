//! The system clipboard of the `/hooks/clipboard-write` fixture: permissions, reading it back,
//! and a page without a clipboard.
use rootcause::Report;

use crate::pages::Page;

/// The clipboard permissions of a test's page (its browser context: browser-test runs every
/// test in a context of its own).
pub enum ClipboardAccess {
    /// Writing and reading back (`navigator.clipboard.readText()`) are allowed.
    ReadWrite,
    /// Writing is denied.
    WriteDenied,
}

/// Actions on the page's clipboard.
pub struct ClipboardActions<'a, 'd> {
    page: &'a Page<'d>,
}

impl<'a, 'd> ClipboardActions<'a, 'd> {
    pub fn new(page: &'a Page<'d>) -> Self {
        Self { page }
    }

    /// Sets what the page may do with the clipboard.
    pub async fn set_access(&self, access: ClipboardAccess) -> Result<(), Report> {
        let cdp = self.page.low_level().driver().cdp();
        let tab = cdp
            .send_raw("Target.getTargetInfo", serde_json::json!({}))
            .await?;
        let context = tab["targetInfo"]["browserContextId"].clone();
        let origin = self.page.base_url().trim_end_matches('/').to_owned();
        match access {
            ClipboardAccess::ReadWrite => {
                cdp.send_raw(
                    "Browser.grantPermissions",
                    serde_json::json!({
                        "origin": origin,
                        "permissions": ["clipboardReadWrite", "clipboardSanitizedWrite"],
                        "browserContextId": context,
                    }),
                )
                .await?;
            }
            ClipboardAccess::WriteDenied => {
                for name in ["clipboard-write", "clipboard-read"] {
                    cdp.send_raw(
                        "Browser.setPermission",
                        serde_json::json!({
                            "origin": origin,
                            "permission": { "name": name, "allowWithoutSanitization": true },
                            "setting": "denied",
                            "browserContextId": context,
                        }),
                    )
                    .await?;
                }
            }
        }
        Ok(())
    }

    /// Removes the page's clipboard (`navigator.clipboard`), as browsers offer none outside
    /// secure contexts (a page served over plain HTTP from another host than `localhost`).
    pub async fn remove_clipboard(&self) -> Result<(), Report> {
        let _: serde_json::Value = self
            .page
            .low_level()
            .eval(
                "Object.defineProperty(Navigator.prototype, 'clipboard', { get: () => undefined, configurable: true });
                return null;",
                vec![],
            )
            .await?;
        Ok(())
    }

    /// The clipboard's text (`navigator.clipboard.readText()`; WebDriver awaits the promise).
    pub async fn text(&self) -> Result<String, Report> {
        self.page
            .low_level()
            .eval("return navigator.clipboard.readText();", vec![])
            .await
    }
}
