use serde::Deserialize;
use std::collections::HashMap;

use crate::converter::helix;

#[derive(Deserialize, Debug)]
pub struct Theme {
    #[serde(rename = "colors")]
    pub colors: HashMap<String, String>,

    #[serde(rename = "tokenColors")]
    pub token_colors: Vec<TokenColor>,
}

#[derive(Deserialize, Debug)]
pub struct TokenColor {
    #[serde(rename = "scope")]
    pub scope: Scope,

    #[serde(rename = "settings")]
    pub settings: Settings,
}

impl TokenColor {
    pub fn to_helix_scope(&self, scope: &mut helix::Scope) {
        if let Some(fg) = &self.settings.foreground {
            scope.fg = Some(fg.clone());
        }

        if let Some(bg) = &self.settings.background {
            scope.bg = Some(bg.clone());
        }

        if let Some(font_style) = &self.settings.font_style {
            let mut new_modifiers: Vec<String> = font_style
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            if let Some(modifiers) = &scope.modifiers {
                new_modifiers.append(&mut modifiers.clone());
            }

            new_modifiers.sort();
            new_modifiers.dedup();

            scope.modifiers = Some(new_modifiers);
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    #[serde(rename = "fontStyle")]
    pub font_style: Option<String>,

    #[serde(rename = "foreground")]
    pub foreground: Option<String>,

    #[serde(rename = "background")]
    pub background: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Scope {
    String(String),
    StringArray(Vec<String>),
}
