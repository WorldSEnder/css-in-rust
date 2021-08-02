// Copyright © 2020 Lukas Wagner

//! Yew integration module.
//! The user doesn't need to do anything but to put a style into the class of a
//! yew component.

use super::super::style::Style;
use crate::style::ast::Scopes;
use yew::prelude::Classes;

impl From<&Style> for Classes {
    fn from(style: &Style) -> Self {
        let mut classes = Self::new();
        classes.push(style.get_class_name().to_string());
        classes
    }
}

pub fn gen_unique_name(suggestion: impl Into<String>) -> String {
    format!(
        "{}-{}",
        suggestion.into(),
        crate::style::arch::classname_entropy()
    )
}

pub fn use_scopes<I1: Into<String>>(class_name: I1, scopes: Scopes) -> Style {
    struct HookState {
        in_use: Option<Style>,
    }

    yew::functional::use_hook(
        || HookState { in_use: None },
        |state, _upd| {
            let mut style = Style::from_scopes(class_name, scopes);
            let new_style = style.clone();
            if let Some(ref mut in_use) = state.in_use {
                // One some archs, node is just a unit, so the comparison is trivial
                #[allow(clippy::unit_cmp)]
                if in_use.node != style.node {
                    style.mount();
                    let mut released = std::mem::replace(in_use, style);
                    released.unmount();
                }
            } else {
                style.mount();
                state.in_use = Some(style);
            }
            new_style
        },
        |state| {
            if let Some(mut in_use) = std::mem::replace(&mut state.in_use, None) {
                in_use.unmount();
            }
        },
    )
}
