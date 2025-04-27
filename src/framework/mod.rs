//! Framework-specific integrations

#[cfg(feature = "leptos")]
pub mod leptos;

/// A wrapper for framework-specific quirks needed to make dom-test-lib work
pub trait Framework {
    /// The kind of context needed to keep a test wrapper working
    ///
    /// Must be cloneable since the wrappers are usually cloned a lot
    type Context: Clone;
}
