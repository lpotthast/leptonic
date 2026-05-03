//! Animation lifecycle hooks for CSS enter/exit animations.
//!
//! These hooks use the Web Animations API (`Element.getAnimations()`) to detect
//! when CSS animations complete, enabling coordinated enter/exit transitions
//! for overlays, popovers, modals, and other animated elements.
//!
//! Based on: <https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/utils/src/animation.ts>

#[cfg(not(feature = "ssr"))]
mod use_animation;

mod use_enter_animation;
mod use_exit_animation;

pub use use_enter_animation::*;
pub use use_exit_animation::*;
