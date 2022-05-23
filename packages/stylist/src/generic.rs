//! This module contains framework independent macros and code.
//!
//! You should only need its contents if you're writing bindings for a web framework.

/// A procedural macro that parses a string literal or an inline stylesheet into a
/// [`StyleSource`](crate::StyleSource).
///
/// Please consult the documentation of the [`macros`](crate::macros) module for the supported
/// syntax of this macro.
///
/// # Example
///
/// ```
/// use stylist::generic::css;
///
/// let style_source = css!("color: red;");
/// ```
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use stylist_macros::css;
/// A procedural macro that parses a string literal or an inline stylesheet into a
/// [`GlobalStyle`](crate::GlobalStyle).
///
/// Please consult the documentation of the [`macros`](crate::macros) module for the supported
/// syntax of this macro.
///
/// # Example
///
/// ```
/// use stylist::generic::global_style;
///
/// // Returns a GlobalStyle instance.
/// let style = global_style!("color: red;");
/// ```
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use stylist_macros::global_style;
/// A procedural macro that parses a string literal or an inline stylesheet into a
/// [`Style`](crate::Style).
///
/// Please consult the documentation of the [`macros`](crate::macros) module for the supported
/// syntax of this macro.
///
/// # Example
///
/// ```
/// use stylist::generic::style;
///
/// // Returns a Style instance.
/// let style = style!("color: red;");
/// ```
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use stylist_macros::style;
