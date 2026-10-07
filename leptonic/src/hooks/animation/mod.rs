//! Animation lifecycle hooks for CSS enter/exit animations (react-aria's `utils/animation.ts`,
//! ported in the submodules).
//!
//! These hooks use the Web Animations API (`Element.getAnimations()`) to detect
//! when CSS animations complete, enabling coordinated enter/exit transitions
//! for overlays, popovers, modals, and other animated elements.

#[cfg(not(feature = "ssr"))]
mod use_animation;

mod use_enter_animation;
mod use_exit_animation;

pub use use_enter_animation::*;
pub use use_exit_animation::*;
