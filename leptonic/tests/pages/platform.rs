//! Emulated platforms (see `Page::emulate_platform`): what Chrome on Linux reports
//! instead, through CDP's user agent override.

use browser_test::thirtyfour::cdp::{CdpCommand, Empty};
use serde::Serialize;

/// A platform whose code paths leptonic's platform checks select.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Mac,
    IPhone,
    /// Chrome on an Android phone (TalkBack's virtual pointer events).
    Android,
}

/// Chrome's typed override, including user-agent metadata not exposed by thirtyfour's convenience type.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UserAgentOverride {
    user_agent: &'static str,
    platform: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_agent_metadata: Option<Metadata>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Metadata {
    platform: &'static str,
    platform_version: &'static str,
    architecture: &'static str,
    model: &'static str,
    mobile: bool,
    brands: [Brand; 1],
}
#[derive(Serialize)]
struct Brand {
    brand: &'static str,
    version: &'static str,
}
impl CdpCommand for UserAgentOverride {
    const METHOD: &'static str = "Emulation.setUserAgentOverride";
    type Returns = Empty;
}
impl Platform {
    pub(super) fn user_agent_override(self) -> UserAgentOverride {
        match self {
            Self::Mac => UserAgentOverride {
                user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36",
                platform: "MacIntel",
                user_agent_metadata: Some(Metadata {
                    platform: "macOS",
                    platform_version: "15.0.0",
                    architecture: "arm",
                    model: "",
                    mobile: false,
                    brands: [Brand {
                        brand: "Chromium",
                        version: "140",
                    }],
                }),
            },
            Self::Android => UserAgentOverride {
                user_agent: "Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Mobile Safari/537.36",
                platform: "Linux armv8l",
                user_agent_metadata: Some(Metadata {
                    platform: "Android",
                    platform_version: "14.0.0",
                    architecture: "",
                    model: "Pixel 8",
                    mobile: true,
                    brands: [Brand {
                        brand: "Chromium",
                        version: "140",
                    }],
                }),
            },
            Self::IPhone => UserAgentOverride {
                user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 17_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4 Mobile/15E148 Safari/604.1",
                platform: "iPhone",
                user_agent_metadata: None,
            },
        }
    }
}
