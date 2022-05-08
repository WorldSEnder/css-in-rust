#![deny(missing_debug_implementations)]
#![deny(unsafe_code)]
#![deny(non_snake_case)]
#![deny(clippy::all)]
#![deny(clippy::cognitive_complexity)]
#![cfg_attr(documenting, feature(doc_cfg))]
#![cfg_attr(any(releasing, not(debug_assertions)), deny(dead_code, unused_imports))]

//! Stylist is a CSS-in-Rust styling solution for WebAssembly Applications.
//!
//! ## Usage
//!
//! ### Yew Integration
//!
//! Enable the `yew_integration` feature in your `Cargo.toml`.
//!
//! You can create a styled function component and use it with Yew like this:
//!
//! ```rust
//! use stylist::yew::styled_component;
//! use yew::prelude::*;
//!
//! #[styled_component(MyStyledComponent)]
//! fn my_styled_component() -> Html {
//!     html! {<div class={css!("color: red;")}>{"Hello World!"}</div>}
//! }
//! ```
//!
//! ### Style API
//!
//! If you want to parse a string into a style at runtime, the `parser` feature must be enabled.
//! Note that you do not need to do this if you use the provided macros to create the css.
//! You can then use [`Style::new`]:
//!
//! ```rust
//! use stylist::Style;
//!
//! let style = Style::new(
//!     r#"
//!         background-color: red;
//!
//!         .nested {
//!             background-color: blue;
//!             width: 100px
//!         }
//!     "#,
//! )
//! .expect("Failed to create style");
//! ```
//!
//! ### Syntax
//!
//! Everything that is not in a conditioned block will be applied to the Component
//! the class of this style is applied to.
//!
//! You may also use Current Selector (`&`) in CSS selectors to denote the container element:
//!
//! ```css
//! &:hover {
//!   background-color: #d0d0d9;
//! }
//! ```
//!
//! You can also use other CSS rules(such as: keyframes, supports and media):
//!
//! ```css
//! @keyframes mymove {
//!   from {
//!     top: 0px;
//!   }
//!   to {
//!     top: 200px;
//!   }
//! }
//! ```
//!
//! ```css
//! @media only screen and (max-width: 600px) {
//!   background-color: #303040;
//!
//!   .nested {
//!     background-color: lightblue;
//!   }
//!
//!   &:hover {
//!     background-color: #606072;
//!   }
//! }
//! ```
//!
//! ```css
//! @supports (backdrop-filter: blur(5px)) {
//!   backdrop-filter: blur(5px);
//! }
//! ```
//!
//! ### Theming
//!
//! There's theming example using
//! [Yew Context API](https://github.com/futursolo/stylist-rs/tree/master/examples/yew-theme-context).
//!
//! ## Features Flags
//!
//! - `macros`: Enabled by default, this flag enables procedural macro support.
//! - `parser`: Enabled by default, this flag enables runtime parsing of strings into Styles.
//! - `random`: Enabled by default, this flag uses `fastrand` crate to generate a random class name.
//!   Disabling this flag will opt for a class name that is counter-based.
//! - `yew_integration`: This flag enables yew integration, which implements
//!   [`Classes`](::yew::html::Classes) for [`Style`] and provides a [`Global`](yew::Global)
//!   component for applying global styles.
//! - `debug_style_locations`: Enabled by default, this flag adds additional class names when a
//!   style is used that identify the source location of your css. Not relevant when compiling in
//!   release mode.

#[cfg(any(feature = "yew_use_media_query", target_arch = "wasm32"))]
mod arch;
pub mod ast;
pub mod generic;
mod global_style;
#[cfg_attr(documenting, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub mod macros;
pub mod manager;
mod registry;
mod style;
mod style_src;
mod utils;
#[cfg_attr(documenting, doc(cfg(feature = "yew")))]
#[cfg(feature = "yew")]
pub mod yew;

pub use global_style::GlobalStyle;
pub use style::Style;
pub use style_src::StyleSource;
#[doc(inline)]
pub use stylist_core::{Error, Result};
