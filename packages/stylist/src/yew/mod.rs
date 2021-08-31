//! This module contains yew specific features.

use std::borrow::Cow;

use yew::{
    html::{Classes, IntoPropValue},
    use_context, use_hook,
};

use crate::{ast::Sheet, manager::StyleManager, Style, StyleSource};

mod global;

pub use global::{Global, GlobalProps};

impl From<Style> for Classes {
    fn from(style: Style) -> Self {
        let mut classes = Self::new();
        classes.push(style.get_class_name().to_string());
        classes
    }
}

impl<'a> From<&'a Style> for Classes {
    fn from(style: &'a Style) -> Self {
        let mut classes = Self::new();
        classes.push(style.get_class_name().to_string());
        classes
    }
}

impl From<StyleSource<'_>> for Classes {
    fn from(style_src: StyleSource<'_>) -> Self {
        let mut classes = Self::new();
        classes.push(style_src.to_style().get_class_name().to_string());
        classes
    }
}

impl IntoPropValue<StyleSource<'static>> for Sheet {
    fn into_prop_value(self) -> StyleSource<'static> {
        self.into()
    }
}

#[cfg_attr(documenting, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
mod feat_parser {
    use std::borrow::Cow;

    use super::*;

    impl IntoPropValue<StyleSource<'static>> for String {
        fn into_prop_value(self) -> StyleSource<'static> {
            self.into()
        }
    }

    impl<'a> IntoPropValue<StyleSource<'a>> for &'a str {
        fn into_prop_value(self) -> StyleSource<'a> {
            self.into()
        }
    }

    impl<'a> IntoPropValue<StyleSource<'a>> for Cow<'a, str> {
        fn into_prop_value(self) -> StyleSource<'a> {
            self.into()
        }
    }
}

fn use_sheet_impl(prefix: Cow<'static, str>, ctx: StyleManager, style: StyleSource) -> Style {
    #[derive(Default)]
    struct HookState {
        manager: StyleManager,
        style: Option<Style>,
    }
    impl HookState {
        fn unregister(&mut self) {
            if let Some(s) = self.style.take() {
                s.unregister()
            }
        }
    }
    // FIXME: mounting and unmounting should be an asynchronous effect to speed up the hook
    use_hook(
        HookState::default,
        |state, _updater| {
            if state.manager != ctx {
                state.unregister();
                state.manager = ctx.clone();
            }
            state
                .style
                .get_or_insert_with(|| {
                    // TODO: creation != mounting
                    Style::create_with_manager(prefix, style, ctx).expect("Style mounting failed")
                })
                .clone()
        },
        HookState::unregister,
    )
}

pub fn use_sheet<P, S>(prefix: P, style: S) -> Style
where
    P: Into<Cow<'static, str>>,
    S: Into<StyleSource<'static>>,
{
    let prefix = prefix.into();
    let style = style.into();
    let ctx = use_context::<StyleManager>().unwrap_or_default();
    use_sheet_impl(prefix, ctx, style)
}
