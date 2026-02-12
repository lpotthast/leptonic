use leptos_use::use_window;
use web_sys::Navigator;

/// Tests for device types.
pub mod device {
    use crate::utils::platform::{navigator, platform, user_agent};

    /// Returns `true` if the current platform is macOS.
    pub fn is_mac() -> bool {
        platform().as_deref().is_some_and(_is_mac)
    }

    fn _is_mac(platform: &str) -> bool {
        platform.contains("Mac")
    }

    /// Returns `true` if the current device is an iOS device (currently detect iPhone and iPad).
    pub fn is_ios() -> bool {
        navigator().is_some_and(|nav| {
            let platform = nav.platform().unwrap_or_default();
            _is_iphone(&platform)
                || _is_ipad(
                    &platform,
                    &nav.user_agent().unwrap_or_default(),
                    nav.max_touch_points(),
                )
        })
    }

    /// Returns `true` if the current device is an iPhone.
    pub fn is_iphone() -> bool {
        platform().as_deref().is_some_and(_is_iphone)
    }

    fn _is_iphone(platform: &str) -> bool {
        platform.contains("iPhone")
    }

    /// Returns `true` if the current device is an iPad.
    ///
    /// Modern iPads report as macOS in their user agent string, so we also check
    /// for touch support via `max_touch_points`.
    pub fn is_ipad() -> bool {
        navigator().is_some_and(|nav| {
            let platform = nav.platform().unwrap_or_default();
            let ua = nav.user_agent().unwrap_or_default();
            _is_ipad(&platform, &ua, nav.max_touch_points())
        })
    }

    fn _is_ipad(platform: &str, user_agent: &str, max_touch_points: i32) -> bool {
        // iPadOS 13+ identifies as Mac but has touch support.
        (platform.contains("iPad") || (_is_mac(&platform) && max_touch_points > 2))
            && !user_agent.contains("CriOS")
    }

    pub fn is_android() -> bool {
        user_agent().as_deref().is_some_and(_is_android)
    }

    fn _is_android(user_agent: &str) -> bool {
        user_agent.contains("Android")
    }
}

/// Tests for browser types.
pub mod browser {
    use crate::utils::platform::user_agent;

    /// Returns `true` if the current browser is Firefox.
    pub fn is_chrome() -> bool {
        user_agent().as_deref().is_some_and(_is_chrome)
    }

    fn _is_chrome(user_agent: &str) -> bool {
        user_agent.contains("Chrome")
    }

    /// Returns `true` if the current browser is Firefox.
    pub fn is_firefox() -> bool {
        user_agent().as_deref().is_some_and(_is_firefox)
    }

    fn _is_firefox(user_agent: &str) -> bool {
        user_agent.contains("Firefox")
    }

    /// Returns `true` if the current browser is apple `WebKit`-based (Safari) but not Chrome.
    pub fn is_webkit() -> bool {
        user_agent().as_deref().is_some_and(_is_webkit)
    }

    fn _is_webkit(user_agent: &str) -> bool {
        user_agent.contains("AppleWebKit") && !user_agent.contains("Chrome")
    }
}

fn platform() -> Option<String> {
    navigator().and_then(|it| it.platform().ok())
}

fn user_agent() -> Option<String> {
    navigator().and_then(|it| it.user_agent().ok())
}

fn navigator() -> Option<Navigator> {
    use_window().as_ref().map(|w| w.navigator())
}
