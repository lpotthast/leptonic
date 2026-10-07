// Upstream: react-aria/src/utils/platform.ts @ 99e6102368
//! Platform and browser detection (react-aria's `platform.ts`).
//!
//! The answers are computed once per page from the navigator (`userAgentData` brands and platform
//! where the browser provides them, else the user agent and `navigator.platform`) and cached, as
//! upstream does. During server-side rendering there is no navigator: every check is `false`.

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The checks are grouped into `device` (`is_mac`, `is_ios`, ...) and `browser` (`is_chrome`,
//   `is_webkit`, ...) modules: they read better at the call site.
// - The detection is a pure function over a `NavigatorInfo` snapshot (unit-testable); react-aria
//   tests regular expressions against `window.navigator` directly.
//
// =============================================================================

use std::cell::OnceCell;

/// Tests for device types.
pub mod device {
    use super::detected;

    /// Whether the platform is macOS (react-aria's `isMac`). iPads report themselves as Macs too.
    pub fn is_mac() -> bool {
        detected().is_mac
    }

    /// Whether the device is an iPhone or an iPad (react-aria's `isIOS`).
    pub fn is_ios() -> bool {
        detected().is_ios()
    }

    /// Whether the device is an iPhone (react-aria's `isIPhone`).
    pub fn is_iphone() -> bool {
        detected().is_iphone
    }

    /// Whether the device is an iPad (react-aria's `isIPad`). iPadOS 13+ reports itself as a Mac:
    /// a Mac with touch support counts as an iPad.
    pub fn is_ipad() -> bool {
        detected().is_ipad()
    }

    /// Whether the device is an Apple device, macOS or iOS (react-aria's `isAppleDevice`).
    pub fn is_apple_device() -> bool {
        detected().is_apple_device()
    }

    /// Whether the device runs Android (react-aria's `isAndroid`).
    pub fn is_android() -> bool {
        detected().is_android
    }
}

/// Tests for browser types.
pub mod browser {
    use super::detected;

    /// Whether the browser is Chrome or Chromium-based, including Chrome on iOS (react-aria's
    /// `isChrome`).
    pub fn is_chrome() -> bool {
        detected().is_chrome
    }

    /// Whether the browser is Firefox, including Firefox on iOS (react-aria's `isFirefox`).
    pub fn is_firefox() -> bool {
        detected().is_firefox
    }

    /// Whether the browser engine is Apple's WebKit (react-aria's `isWebKit`): Safari, and every
    /// browser on iOS (they all run on WebKit), but not Chrome elsewhere.
    pub fn is_webkit() -> bool {
        detected().is_webkit()
    }

    /// Whether the browser is Safari itself (react-aria's `isSafari`): WebKit, neither Chrome nor
    /// Firefox.
    pub fn is_safari() -> bool {
        detected().is_safari()
    }
}

/// What the navigator reports, the input of the detection.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct NavigatorInfo {
    /// `navigator.userAgent`.
    user_agent: String,
    /// `navigator.userAgentData.brands[].brand` (Chromium-based browsers only).
    brands: Vec<String>,
    /// `navigator.userAgentData.platform`, else `navigator.platform`.
    platform: String,
    /// `navigator.maxTouchPoints`.
    max_touch_points: i32,
}

/// The detected platform and browser.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
struct Platform {
    is_mac: bool,
    is_iphone: bool,
    /// The platform reports an iPad.
    reports_ipad: bool,
    /// A Mac with touch support (iPadOS 13+ reports itself as a Mac).
    is_touch_mac: bool,
    /// The user agent mentions AppleWebKit (Chrome does too).
    has_apple_webkit: bool,
    is_chrome: bool,
    is_firefox: bool,
    is_android: bool,
}

impl Platform {
    fn detect(info: &NavigatorInfo) -> Self {
        let test_user_agent = |needles: &[&str]| {
            needles.iter().any(|needle| {
                info.brands
                    .iter()
                    .any(|brand| contains_ignore_case(brand, needle))
                    || contains_ignore_case(&info.user_agent, needle)
            })
        };
        let test_platform = |prefix: &str| starts_with_ignore_case(&info.platform, prefix);

        let is_mac = test_platform("Mac");
        Self {
            is_mac,
            is_iphone: test_platform("iPhone"),
            reports_ipad: test_platform("iPad"),
            is_touch_mac: is_mac && info.max_touch_points > 1,
            has_apple_webkit: test_user_agent(&["AppleWebKit"]),
            is_chrome: test_user_agent(&["Chrome", "CriOS", "CrMo"]),
            is_firefox: test_user_agent(&["Firefox", "FxiOS"]),
            is_android: test_user_agent(&["Android"]),
        }
    }

    fn is_ipad(self) -> bool {
        self.reports_ipad || self.is_touch_mac
    }

    fn is_ios(self) -> bool {
        self.is_iphone || self.is_ipad()
    }

    fn is_apple_device(self) -> bool {
        self.is_mac || self.is_ios()
    }

    fn is_webkit(self) -> bool {
        self.has_apple_webkit && (self.is_ios() || !self.is_chrome)
    }

    fn is_safari(self) -> bool {
        self.is_webkit() && !self.is_chrome && !self.is_firefox
    }
}

fn contains_ignore_case(haystack: &str, needle: &str) -> bool {
    haystack
        .to_ascii_lowercase()
        .contains(&needle.to_ascii_lowercase())
}

fn starts_with_ignore_case(haystack: &str, prefix: &str) -> bool {
    haystack
        .get(..prefix.len())
        .is_some_and(|start| start.eq_ignore_ascii_case(prefix))
}

thread_local! {
    /// The detection result, computed on first use (react-aria's `cached`).
    static DETECTED: OnceCell<Platform> = const { OnceCell::new() };
}

fn detected() -> Platform {
    DETECTED.with(|detected| *detected.get_or_init(|| Platform::detect(&navigator_info())))
}

/// Reads the navigator (nothing during server-side rendering).
fn navigator_info() -> NavigatorInfo {
    use js_sys::{Array, Reflect};
    use wasm_bindgen::JsValue;

    let Some(navigator) = leptos_use::use_window()
        .as_ref()
        .map(web_sys::Window::navigator)
    else {
        return NavigatorInfo::default();
    };
    // `userAgentData` (Chromium only) isn't in web-sys' stable surface: read it reflectively.
    let ua_data = Reflect::get(&navigator, &JsValue::from_str("userAgentData"))
        .ok()
        .filter(JsValue::is_object);
    let brands = ua_data
        .as_ref()
        .and_then(|data| Reflect::get(data, &JsValue::from_str("brands")).ok())
        .filter(Array::is_array)
        .map(|brands| {
            Array::from(&brands)
                .iter()
                .filter_map(|brand| Reflect::get(&brand, &JsValue::from_str("brand")).ok())
                .filter_map(|brand| brand.as_string())
                .collect()
        })
        .unwrap_or_default();
    // `userAgentData.platform || navigator.platform`: an empty platform falls through.
    let platform = ua_data
        .as_ref()
        .and_then(|data| Reflect::get(data, &JsValue::from_str("platform")).ok())
        .and_then(|platform| platform.as_string())
        .filter(|platform| !platform.is_empty())
        .or_else(|| navigator.platform().ok())
        .unwrap_or_default();
    NavigatorInfo {
        user_agent: navigator.user_agent().unwrap_or_default(),
        brands,
        platform,
        max_touch_points: navigator.max_touch_points(),
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::{NavigatorInfo, Platform};

    const MAC_SAFARI: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 \
                              (KHTML, like Gecko) Version/17.4 Safari/605.1.15";
    const MAC_CHROME: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 \
                              (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36";
    const MAC_FIREFOX: &str =
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 14.7; rv:131.0) Gecko/20100101 Firefox/131.0";
    const IPHONE_SAFARI: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_4 like Mac OS X) \
                                 AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4 \
                                 Mobile/15E148 Safari/604.1";
    const IPHONE_CHROME: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_4 like Mac OS X) \
                                 AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/129.0.6668.69 \
                                 Mobile/15E148 Safari/604.1";
    const IPHONE_FIREFOX: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_4 like Mac OS X) \
                                  AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/131.0 \
                                  Mobile/15E148 Safari/605.1.15";
    const ANDROID_CHROME: &str = "Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 \
                                  (KHTML, like Gecko) Chrome/129.0.0.0 Mobile Safari/537.36";
    const LINUX_FIREFOX: &str =
        "Mozilla/5.0 (X11; Linux x86_64; rv:131.0) Gecko/20100101 Firefox/131.0";
    const WINDOWS_CHROME: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
                                  (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36";

    fn detect(user_agent: &str, platform: &str, max_touch_points: i32) -> Platform {
        Platform::detect(&NavigatorInfo {
            user_agent: user_agent.to_owned(),
            brands: Vec::new(),
            platform: platform.to_owned(),
            max_touch_points,
        })
    }

    #[test]
    fn mac_safari() {
        let p = detect(MAC_SAFARI, "MacIntel", 0);
        assert_that!(p.is_mac).is_true();
        assert_that!(p.is_ios()).is_false();
        assert_that!(p.is_ipad()).is_false();
        assert_that!(p.is_apple_device()).is_true();
        assert_that!(p.is_webkit()).is_true();
        assert_that!(p.is_safari()).is_true();
        assert_that!(p.is_chrome).is_false();
        assert_that!(p.is_firefox).is_false();
    }

    #[test]
    fn mac_chrome_is_not_webkit() {
        let p = detect(MAC_CHROME, "MacIntel", 0);
        assert_that!(p.is_mac).is_true();
        assert_that!(p.is_chrome).is_true();
        assert_that!(p.is_webkit()).is_false();
        assert_that!(p.is_safari()).is_false();
    }

    #[test]
    fn mac_firefox() {
        let p = detect(MAC_FIREFOX, "MacIntel", 0);
        assert_that!(p.is_firefox).is_true();
        assert_that!(p.is_webkit()).is_false();
        assert_that!(p.is_safari()).is_false();
    }

    #[test]
    fn iphone_safari() {
        let p = detect(IPHONE_SAFARI, "iPhone", 5);
        assert_that!(p.is_iphone).is_true();
        assert_that!(p.is_ios()).is_true();
        assert_that!(p.is_ipad()).is_false();
        assert_that!(p.is_mac).is_false();
        assert_that!(p.is_apple_device()).is_true();
        assert_that!(p.is_webkit()).is_true();
        assert_that!(p.is_safari()).is_true();
    }

    /// Every browser on iOS runs on WebKit: Chrome (CriOS) and Firefox (FxiOS) are WebKit there,
    /// but not Safari.
    #[test]
    fn chrome_and_firefox_on_ios_are_webkit() {
        let chrome = detect(IPHONE_CHROME, "iPhone", 5);
        assert_that!(chrome.is_chrome).is_true();
        assert_that!(chrome.is_ios()).is_true();
        assert_that!(chrome.is_webkit()).is_true();
        assert_that!(chrome.is_safari()).is_false();

        let firefox = detect(IPHONE_FIREFOX, "iPhone", 5);
        assert_that!(firefox.is_firefox).is_true();
        assert_that!(firefox.is_webkit()).is_true();
        assert_that!(firefox.is_safari()).is_false();
    }

    /// iPadOS 13+ reports itself as a Mac; touch support (more than one touch point) tells it
    /// apart, also for Chrome on iPad.
    #[test]
    fn ipad_reporting_as_mac() {
        let safari = detect(MAC_SAFARI, "MacIntel", 5);
        assert_that!(safari.is_ipad()).is_true();
        assert_that!(safari.is_ios()).is_true();
        assert_that!(safari.is_mac).is_true();

        let chrome = detect(
            IPHONE_CHROME.replace("iPhone", "iPad").as_str(),
            "MacIntel",
            5,
        );
        assert_that!(chrome.is_ipad()).is_true();
        assert_that!(chrome.is_webkit()).is_true();

        let one_touch_point = detect(MAC_SAFARI, "MacIntel", 1);
        assert_that!(one_touch_point.is_ipad()).is_false();
    }

    #[test]
    fn ipad_platform() {
        let p = detect(IPHONE_SAFARI.replace("iPhone", "iPad").as_str(), "iPad", 5);
        assert_that!(p.is_ipad()).is_true();
        assert_that!(p.is_iphone).is_false();
        assert_that!(p.is_ios()).is_true();
    }

    #[test]
    fn android_chrome() {
        let p = detect(ANDROID_CHROME, "Linux armv8l", 5);
        assert_that!(p.is_android).is_true();
        assert_that!(p.is_chrome).is_true();
        assert_that!(p.is_webkit()).is_false();
        assert_that!(p.is_apple_device()).is_false();
    }

    #[test]
    fn desktop_firefox_and_chrome() {
        let firefox = detect(LINUX_FIREFOX, "Linux x86_64", 0);
        assert_that!(firefox.is_firefox).is_true();
        assert_that!(firefox.is_chrome).is_false();
        assert_that!(firefox.is_android).is_false();

        let chrome = detect(WINDOWS_CHROME, "Win32", 0);
        assert_that!(chrome.is_chrome).is_true();
        assert_that!(chrome.is_mac).is_false();
        assert_that!(chrome.is_webkit()).is_false();
    }

    /// `userAgentData` brands count like the user agent, and its platform ("macOS") matches
    /// case-insensitively.
    #[test]
    fn user_agent_data() {
        let p = Platform::detect(&NavigatorInfo {
            user_agent: String::new(),
            brands: vec!["Not)A;Brand".to_owned(), "Google Chrome".to_owned()],
            platform: "macOS".to_owned(),
            max_touch_points: 0,
        });
        assert_that!(p.is_chrome).is_true();
        assert_that!(p.is_mac).is_true();
    }

    /// The platform is matched at its start only.
    #[test]
    fn platform_is_a_prefix_match() {
        assert_that!(detect("", "NotMac", 0).is_mac).is_false();
        assert_that!(detect("", "", 0)).is_equal_to(Platform::default());
    }
}
