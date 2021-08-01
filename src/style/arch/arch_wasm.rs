use super::super::{ast::ToCss, Style};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlHeadElement};

#[derive(Clone, Debug)]
pub struct DomReference {
    node: Option<Element>,
    /// Usage count
    users: Rc<AtomicUsize>,
}

impl Default for DomReference {
    fn default() -> Self {
        Self {
            node: None,
            users: Rc::new(AtomicUsize::new(0)),
        }
    }
}

impl PartialEq for DomReference {
    fn eq(&self, rhs: &Self) -> bool {
        Rc::ptr_eq(&self.users, &rhs.users)
    }
}

pub type DomNode = DomReference;

/*#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (extends = ::js_sys::Object, js_name = Crypto, typescript_type = "Crypto")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Crypto;

    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}*/

pub fn classname_entropy() -> impl std::fmt::Display {
    let window = web_sys::window().expect("no global `window` exists");
    let crypto = window.crypto().expect("no crypto exists");
    let mut random_bits = [0u8; 8];
    let _ = crypto
        .get_random_values_with_u8_array(&mut random_bits[..])
        .expect("getRandomValues() succeeds");
    random_bits
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<String>()
    // random().to_bits()
}

fn find_head() -> HtmlHeadElement {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let head = document.head().expect("should have a head in document");
    head
}

impl Style {
    /// Mounts the styles to the document head web-sys style
    pub(crate) fn mount(&mut self) {
        if self.node.users.fetch_add(1, Ordering::Acquire) == 0 {
            self.node.node = self.generate_element().ok();
            if let Some(ref node) = self.node.node {
                find_head().append_child(node).expect("mounting failed");
            }
        }
    }

    /// Unmounts the style from the HTML head web-sys style
    pub(crate) fn unmount(&mut self) {
        let internal = &mut self.node;
        if internal.users.fetch_sub(1, Ordering::Release) == 1 {
            if let Some(ref node) = internal.node.take() {
                find_head().remove_child(node).expect("unmounting failed");
            }
        }
    }

    /// Takes all Scopes and lets them translate themselves into CSS.
    fn generate_css(&self) -> String {
        self.ast.to_css(&self.class_name)
    }

    /// Generates the `<style/>` tag web-sys style for inserting into the head of the
    /// HTML document.
    fn generate_element(&self) -> Result<Element, JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let style_element = document.create_element("style").unwrap();
        style_element
            .set_attribute("data-style", self.class_name.as_str())
            .expect("setting data-style failed");
        style_element.set_text_content(Some(self.generate_css().as_str()));
        Ok(style_element)
    }
}
