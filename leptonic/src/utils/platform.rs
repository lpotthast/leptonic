use leptos_use::use_window;

fn test_user_agent<F: Fn(&str) -> bool>(test: F) -> bool {
    match use_window().as_ref() {
        Some(window) => {
            let nav = window.navigator();
            match nav.user_agent() {
                Ok(user_agent) => test(&user_agent),
                Err(_err) => {
                    tracing::warn!("Could not detect user_agent information.");
                    false
                }
            }
        }
        None => false,
    }
}

fn test_platform<F: Fn(&str) -> bool>(test: F) -> bool {
    match use_window().as_ref() {
        Some(window) => {
            let nav = window.navigator();
            match nav.platform() {
                Ok(platform) => test(&platform),
                Err(_err) => {
                    tracing::warn!("Could not detect platform information.");
                    false
                }
            }
        }
        None => false,
    }
}

pub(crate) fn is_mac() -> bool {
    test_platform(|p| p.contains("Mac"))
}

pub(crate) fn is_iphone() -> bool {
    test_platform(|p| p.contains("iPhone"))
}

pub(crate) fn is_ipad() -> bool {
    test_platform(|p| p.contains("iPad")) ||
      // iPadOS 13 lies and says it's a Mac, but we can distinguish by detecting touch support.
      (is_mac() && use_window().as_ref().map(|w| w.navigator().max_touch_points()).unwrap_or_default() > 1)
}

pub(crate) fn is_ios() -> bool {
    is_iphone() || is_ipad()
}

pub(crate) fn is_apple_device() -> bool {
    is_mac() || is_ios()
}

pub(crate) fn is_web_kit() -> bool {
    test_user_agent(|a| a.contains("AppleWebKit")) && !is_chrome()
}

pub(crate) fn is_chrome() -> bool {
    test_user_agent(|a| a.contains("Chrome"))
}

pub(crate) fn is_android() -> bool {
    test_user_agent(|a| a.contains("Android"))
}

pub(crate) fn is_firefox() -> bool {
    test_user_agent(|a| a.contains("Firefox"))
}
