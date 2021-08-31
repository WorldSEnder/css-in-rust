use yew::prelude::*;

use crate::{GlobalStyle, StyleSource};
use stylist_core::ResultDisplay;

/// The properties for [`Global`] Component, please see its documentation for usage.
#[derive(Properties, Clone, Debug, PartialEq)]
pub struct GlobalProps {
    pub css: StyleSource<'static>,
}

/// A Global Style that will be applied to `<html />` tag, inspired by [emotion](https://emotion.sh).
///
/// The `css` attribute accepts anything that implements
/// [`IntoPropValue<StyleSource>`](yew::html::IntoPropValue) and
/// panics if the string failed to be parsed into a stylesheet.
///
/// # Example:
///
/// ```
/// use yew::prelude::*;
/// use stylist::yew::Global;
///
/// struct App;
///
/// impl Component for App {
///     type Message = ();
///     type Properties = ();
///
///     fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
///         Self
///     }
///
///     fn update(&mut self, _msg: Self::Message) -> ShouldRender {
///         false
///     }
///
///     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
///         false
///     }
///
///     fn view(&self) -> Html {
///         html! {
///             <>
///                 <Global css="color: red;" />
///                 <div>{"Hello World!"}</div>
///             </>
///         }
///     }
/// }
/// ```
#[derive(Debug)]
pub struct Global {
    global_style: Option<GlobalStyle>,
}

impl Component for Global {
    type Message = ();
    type Properties = GlobalProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { global_style: None }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.update_global_style(ctx.props());
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.update_global_style(ctx.props());
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        Html::default()
    }
}

impl Global {
    fn update_global_style(&mut self, props: &GlobalProps) {
        if let Some(ref m) = self.global_style {
            m.unregister();
        }

        self.global_style =
            Some(GlobalStyle::new(props.css.clone()).expect_display("Failed to parse style."));
    }
}
