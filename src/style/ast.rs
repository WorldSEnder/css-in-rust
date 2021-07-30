// Copyright © 2020 Lukas Wagner
use super::utils::Itertools;
use std::convert::TryFrom;

/// Structs implementing this trait should be able to turn into
/// a part of a CSS style sheet.
pub trait ToCss {
    fn to_css(&self, class_name: &str) -> String;
}

/// A scope represents a media query or all content not in a media query.
///
/// As an example:
/// ```css
/// /* BEGIN Scope */
/// .wrapper {
///     width: 100vw;
/// }
/// /* END Scope */
/// /* BEGIN Scope */
/// @media only screen and (min-width: 1000px) {
///     .wrapper {
///         width: 1000px;
///     }
/// }
/// /* END Scope */
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Scope {
    pub(crate) condition: Option<String>,
    pub(crate) stylesets: Vec<ScopeContent>,
}

impl ToCss for Scope {
    fn to_css(&self, class_name: &str) -> String {
        let stylesets = self.stylesets.clone();
        let stylesets_css = stylesets
            .into_iter()
            .map(|styleset| match styleset {
                ScopeContent::Block(block) => block.to_css(class_name),
                ScopeContent::Rule(rule) => rule.to_css(class_name),
                // ScopeContent::Scope(scope) => scope.to_css(class_name.clone()),
            })
            .join("\n");
        match &self.condition {
            Some(condition) => format!("{} {{\n{}\n}}", condition, stylesets_css),
            None => stylesets_css.trim().to_string(),
        }
    }
}

impl Scopes {
    /// Combine the two scopes by appending more rules to it
    pub fn append(&mut self, other: Scopes) {
        let mut other = other;
        self.scopes.append(&mut other.scopes);
    }
}

#[derive(Debug, Clone, PartialEq, Default, Eq, Hash)]
pub struct Scopes {
    scopes: Vec<Scope>,
}

impl TryFrom<String> for Scopes {
    type Error = String;
    fn try_from(css: String) -> std::result::Result<Self, Self::Error> {
        let scopes = crate::parser::Parser::parse(css)?;
        Ok(Scopes { scopes })
    }
}

impl ToCss for Scopes {
    fn to_css(&self, class_name: &str) -> String {
        self.scopes
            .iter()
            .map(|scope| scope.to_css(class_name))
            .join("\n")
    }
}

/// Everything that can reside inside a scope.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ScopeContent {
    Block(Block),
    Rule(Rule),
    // e.g. media rules nested in support rules and vice versa
    // Scope(Scope),
}

/// A block is a set of css properties that apply to elements that
/// match the condition.
///
/// E.g.:
/// ```css
/// .inner {
///     color: red;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Block {
    pub(crate) condition: Option<String>,
    pub(crate) style_attributes: Vec<StyleAttribute>,
}

impl ToCss for Block {
    fn to_css(&self, class_name: &str) -> String {
        let condition = match &self.condition {
            Some(condition) => format!(" {}", condition),
            None => String::new(),
        };
        let style_property_css = self
            .style_attributes
            .clone()
            .into_iter()
            .map(|style_property| style_property.to_css(class_name))
            .join("\n");
        if condition.contains('&') {
            format!(
                "{} {{\n{}\n}}",
                condition.replace("&", format!(".{}", class_name).as_str()),
                style_property_css
            )
        } else {
            format!(".{}{} {{\n{}\n}}", class_name, condition, style_property_css)
        }
    }
}

/// A simple CSS proprerty in the form of a key value pair.
///
/// E.g.: `color: red`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct StyleAttribute {
    pub(crate) key: String,
    pub(crate) value: String,
}

impl ToCss for StyleAttribute {
    fn to_css(&self, _: &str) -> String {
        format!("{}: {};", self.key, self.value)
    }
}

/// A rule is everything that does not contain any properties.
///
/// An example would be `@keyframes`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Rule {
    pub(crate) condition: String,
    pub(crate) content: Vec<RuleContent>,
}

impl ToCss for Rule {
    fn to_css(&self, class_name: &str) -> String {
        format!(
            "{} {{\n{}\n}}",
            self.condition,
            self.content
                .iter()
                .map(|rc| rc.to_css(class_name))
                .collect::<Vec<String>>()
                .concat()
        )
    }
}

/// Everything that can be inside a rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum RuleContent {
    String(String),
    CurlyBraces(Vec<RuleContent>),
}

impl ToCss for RuleContent {
    fn to_css(&self, class_name: &str) -> String {
        match self {
            RuleContent::String(s) => s.to_string(),
            RuleContent::CurlyBraces(content) => format!(
                "{{{}}}",
                content
                    .iter()
                    .map(|rc| rc.to_css(class_name))
                    .collect::<Vec<String>>()
                    .concat()
            ),
        }
    }
}

impl Into<RuleContent> for String {
    fn into(self) -> RuleContent {
        RuleContent::String(self)
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use super::{Block, Rule, Scope, ScopeContent, StyleAttribute, ToCss};
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_scope_building_without_condition() {
        let test_block = Scope {
            condition: None,
            stylesets: vec![
                ScopeContent::Block(Block {
                    condition: None,
                    style_attributes: vec![StyleAttribute {
                        key: String::from("width"),
                        value: String::from("100vw"),
                    }],
                }),
                ScopeContent::Block(Block {
                    condition: Some(String::from(".inner")),
                    style_attributes: vec![StyleAttribute {
                        key: String::from("background-color"),
                        value: String::from("red"),
                    }],
                }),
                ScopeContent::Rule(Rule {
                    condition: String::from("@keyframes move"),
                    content: vec![String::from(
                        r#"from {
width: 100px;
}
to {
width: 200px;
}"#,
                    ).into()],
                }),
            ],
        };
        assert_eq!(
            test_block.to_css(String::from("test")),
r#".test {
width: 100vw;
}
.test .inner {
background-color: red;
}
@keyframes move {
from {
width: 100px;
}
to {
width: 200px;
}
}"#
        );
    }

    #[wasm_bindgen_test]
    fn test_scope_building_with_condition() {
        let test_block = Scope {
            condition: Some(String::from("@media only screen and (min-width: 1000px)")),
            stylesets: vec![
                ScopeContent::Block(Block {
                    condition: None,
                    style_attributes: vec![StyleAttribute {
                        key: String::from("width"),
                        value: String::from("100vw"),
                    }],
                }),
                ScopeContent::Block(Block {
                    condition: Some(String::from(".inner")),
                    style_attributes: vec![StyleAttribute {
                        key: String::from("background-color"),
                        value: String::from("red"),
                    }],
                }),
                ScopeContent::Rule(Rule {
                    condition: String::from("@keyframes move"),
                    content: vec![String::from(
                        r#"from {
width: 100px;
}
to {
width: 200px;
}"#,
                    ).into()],
                }),
            ],
        };
        assert_eq!(
            test_block.to_css(String::from("test")),
            r#"@media only screen and (min-width: 1000px) {
.test {
width: 100vw;
}
.test .inner {
background-color: red;
}
@keyframes move {
from {
width: 100px;
}
to {
width: 200px;
}
}
}"#
        );
    }
}
