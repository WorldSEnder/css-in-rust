use stylist::{yew::Global, StyleSource, YieldStyle};
use yew::{html, Component, Context, Html, Properties};
use yewdux::prelude::*;

use log::Level;

mod store;

use store::{theme::ThemeKind, Action, AppDispatch, AppStore};

#[derive(PartialEq, Clone, Default, Properties)]
pub(crate) struct BaseProps {
    #[prop_or_default]
    dispatch: AppDispatch,
}

impl Dispatched for BaseProps {
    type Store = AppStore;

    fn dispatch(&self) -> &DispatchProps<Self::Store> {
        self.dispatch.dispatch()
    }
}

pub(crate) struct BaseInside;

impl Component for BaseInside {
    type Message = ();
    type Properties = BaseProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, _: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let dispatch = ctx.props().dispatch();
        let theme_str = match dispatch.state().theme.kind {
            ThemeKind::Light => "Dark Theme",
            ThemeKind::Dark => "Light Theme",
        };

        let other_theme = match dispatch.state().theme.kind {
            ThemeKind::Light => ThemeKind::Dark,
            ThemeKind::Dark => ThemeKind::Light,
        };

        let switch_theme =
            dispatch.callback(move |_: yew::MouseEvent| Action::SetTheme(other_theme.clone()));

        html! {
            <div class={self.style()}>
                <button onclick={move |c| switch_theme.emit(c)} id="yew-sample-button">{"Switch to "}{theme_str}</button>
            </div>
        }
    }
}

impl YieldStyle for BaseInside {
    fn style_from(&self) -> StyleSource<'static> {
        r#"
            button {
                color: white;
                height: 50px;
                width: 300px;
                font-size: 20px;
                background-color: rgb(88, 164, 255);
                border-radius: 5px;
                border: none;
            }
        "#
        .into()
    }
}

pub(crate) type Inside = WithDispatch<BaseInside>;

pub(crate) struct App {
    dispatch: AppDispatch,
}

impl Component for App {
    type Message = ();
    type Properties = BaseProps;

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = ctx.props().dispatch().clone();
        Self { dispatch }
    }

    fn update(&mut self, ctx: &Context<Self>, _msg: Self::Message) -> bool {
        self.dispatch = ctx.props().dispatch().clone();
        false
    }

    fn changed(&mut self, _: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let dispatch = ctx.props().dispatch();
        let theme = dispatch.state().theme.clone();

        let theme_str = match theme.kind {
            ThemeKind::Light => "light theme",
            ThemeKind::Dark => "dark theme",
        };

        html! {
            <>
                // Global Styles can be applied with <Global /> component.
                <Global css={format!(
                    r#"
                        html, body {{
                            font-family: sans-serif;

                            padding: 0;
                            margin: 0;

                            display: flex;
                            justify-content: center;
                            align-items: center;
                            min-height: 100vh;
                            flex-direction: column;

                            background-color: {bg};
                            color: {ft_color};
                        }}
                    "#,
                    bg = theme.current().background_color,
                    ft_color = theme.current().font_color,
                )} />
                <h1>{"Yew Theming w/ Yewdux"}</h1>
                <div class={self.style()} id="yew-sample-content">
                    {"You are now using the "}{theme_str}{"!"}
                    <Inside />
                </div>
            </>
        }
    }
}

impl YieldStyle for App {
    fn style_from(&self) -> StyleSource<'static> {
        let theme = self.dispatch.state().theme.current();

        format!(
            r#"
                box-shadow: 0 0 5px 1px rgba(0, 0, 0, 0.7);
                height: 500px;
                width: 500px;
                border-radius: 5px;

                display: flex;
                justify-content: space-around;
                align-items: center;

                padding: 15px;
                box-sizing: border-box;

                flex-direction: column;
                background-color: {bg};
            "#,
            bg = theme.paper_color
        )
        .into()
    }
}

fn main() {
    console_log::init_with_level(Level::Trace).expect("Failed to initialise Log!");
    yew::start_app::<WithDispatch<App>>();
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    use web_sys::window;

    #[wasm_bindgen_test]
    fn test_simple() {
        yew::start_app_in_element::<WithDispatch<App>>(
            yew::utils::document().get_element_by_id("output").unwrap(),
        );
        let window = window().unwrap();
        let doc = window.document().unwrap();
        let body = window.document().unwrap().body().unwrap();

        let content = doc.query_selector("#yew-sample-content").unwrap().unwrap();

        let body_style = window.get_computed_style(&body).unwrap().unwrap();
        let content_style = window.get_computed_style(&content).unwrap().unwrap();

        let bg_color = body_style.get_property_value("background-color").unwrap();
        assert_eq!(bg_color, "rgb(237, 244, 255)");

        let content_display = content_style.get_property_value("display").unwrap();
        assert_eq!(content_display, "flex");

        let button = doc
            .query_selector("#yew-sample-button")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        button.click();

        let bg_color = body_style.get_property_value("background-color").unwrap();
        assert_eq!(bg_color, "rgb(0, 0, 0)");
    }
}
