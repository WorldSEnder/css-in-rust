#[cfg(target_arch = "wasm32")]
#[path = "arch_wasm.rs"]
mod detail;
#[cfg(not(target_arch = "wasm32"))]
#[path = "arch_no_wasm.rs"]
mod detail;
// detail also implements Style::mount

pub use detail::{DomNode, classname_entropy};
