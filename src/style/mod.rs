// Copyright © 2020 Lukas Wagner

pub mod ast;
mod arch;
mod utils;

use super::parser::Parser;
use ast::Scope;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref STYLE_REGISTRY: Arc<Mutex<StyleRegistry>> = Arc::new(Mutex::default());
}

/// The style registry is just a global struct that makes sure no style gets lost.
/// Every style automatically registers with the style registry.
#[derive(Debug, Clone)]
struct StyleRegistry {
    styles: HashMap<String, Style>,
}

impl Default for StyleRegistry {
    fn default() -> Self {
        Self {
            styles: HashMap::new(),
        }
    }
}

unsafe impl Send for StyleRegistry {}
unsafe impl Sync for StyleRegistry {}

#[derive(Debug, Clone)]
pub struct Style {
    /// The designated class name of this style
    class_name: String,
    /// The abstract syntax tree of the css
    ast: Option<Vec<Scope>>,
    /// Style DOM node the data in this struct is turned into.
    node: arch::DomNode,
}

impl Style {
    /// Creates a new style and, stores it into the registry and returns the
    /// newly created style.
    ///
    /// This function will already mount the style to the HTML head for the browser to use.
    pub fn create<I1: Into<String>, I2: Into<String>>(
        class_name: I1,
        css: I2,
    ) -> Result<Style, String> {
        let (class_name, css) = (class_name.into(), css.into());
        let ast = Parser::parse(css)?;
        let mut new_style = Self {
            class_name: format!("{}-{}", class_name, arch::classname_entropy()),
            ast: Some(ast),
            node: Default::default(),
        };
        let style_registry_mutex = Arc::clone(&STYLE_REGISTRY);
        let mut style_registry = match style_registry_mutex.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        (*style_registry)
            .styles
            .insert(new_style.class_name.clone(), new_style.clone());
        new_style = new_style.mount();
        Ok(new_style)
    }

    pub fn get_class_name(self) -> String {
        self.class_name
    }
}

impl ToString for Style {
    /// Just returns the classname
    fn to_string(&self) -> String {
        self.class_name.clone()
    }
}
