use serde::Serialize;
use std::collections::HashMap;

use super::vscode;

pub type Theme = HashMap<String, Scope>;

impl From<&vscode::Theme> for Theme {
    fn from(vs: &vscode::Theme) -> Self {
        let mut hx = Self::new();

        vs.token_colors.iter().for_each(|token_color| {
            let scopes: Vec<String> = match &token_color.scope {
                vscode::Scope::String(s) => vec![s.clone()],
                vscode::Scope::StringArray(s) => s.clone(),
            };

            scopes.iter().for_each(|scope_name| {
                let scope_name = scope_name.trim_start_matches("entity.name.");

                if scope_name == "entity.name" {
                    let scope = hx.entry("ui.text".to_string()).or_default();
                    token_color.to_helix_scope(scope);

                    return;
                }

                let scope = hx.entry(scope_name.to_string()).or_default();
                token_color.to_helix_scope(scope);

                if scope.is_empty() {
                    hx.remove(scope_name);
                }
            });
        });

        enum Mapping {
            Fg(String),
            Bg(String),
            FgBg(String, String),
        }

        let mappings = vec![
            (
                "ui.background".to_string(),
                Mapping::Bg("editor.background".to_string()),
            ),
            (
                "ui.cursor".to_string(),
                Mapping::FgBg(
                    "editorCursor.background".to_string(),
                    "editorCursor.foreground".to_string(),
                ),
            ),
            (
                "ui.cursor.match".to_string(),
                Mapping::Bg("editorBracketMatch.background".to_string()),
            ),
            (
                "ui.linenr".to_string(),
                Mapping::Fg("editorLineNumber.foreground".to_string()),
            ),
            (
                "ui.linenr.selected".to_string(),
                Mapping::Fg("editorLineNumber.activeForeground".to_string()),
            ),
            (
                "ui.statusline".to_string(),
                Mapping::FgBg(
                    "statusBar.foreground".to_string(),
                    "statusBar.background".to_string(),
                ),
            ),
            (
                "ui.popup".to_string(),
                Mapping::FgBg(
                    "editorSuggestWidget.foreground".to_string(),
                    "editorSuggestWidget.background".to_string(),
                ),
            ),
            (
                "ui.window".to_string(),
                Mapping::Bg("window.activeBorder".to_string()),
            ),
            (
                "ui.help".to_string(),
                Mapping::FgBg("foreground".to_string(), "editor.background".to_string()),
            ),
            (
                "ui.menu".to_string(),
                Mapping::FgBg(
                    "editorHoverWidget.foreground".to_string(),
                    "editorHoverWidget.background".to_string(),
                ),
            ),
            (
                "ui.selection".to_string(),
                Mapping::FgBg(
                    "editor.selectionBackground".to_string(),
                    "editor.selectionForeground".to_string(),
                ),
            ),
            (
                "warning".to_string(),
                Mapping::FgBg(
                    "editorWarning.foreground".to_string(),
                    "editorWarning.background".to_string(),
                ),
            ),
            (
                "error".to_string(),
                Mapping::FgBg(
                    "editorError.foreground".to_string(),
                    "editorError.background".to_string(),
                ),
            ),
            (
                "info".to_string(),
                Mapping::FgBg(
                    "editorInfo.foreground".to_string(),
                    "editorInfo.background".to_string(),
                ),
            ),
            (
                "hint".to_string(),
                Mapping::FgBg(
                    "editorHint.foreground".to_string(),
                    "editorHint.background".to_string(),
                ),
            ),
        ];

        mappings.iter().for_each(|(scope_name, mapping)| {
            let scope = hx.entry(scope_name.clone()).or_default();

            match mapping {
                Mapping::Fg(fg) => {
                    scope.fg = vs.colors.get(fg).map(|c| c.to_string());
                }
                Mapping::Bg(bg) => {
                    scope.bg = vs.colors.get(bg).map(|c| c.to_string());
                }
                Mapping::FgBg(fg, bg) => {
                    scope.fg = vs.colors.get(fg).map(|c| c.to_string());
                    scope.bg = vs.colors.get(bg).map(|c| c.to_string());
                }
            }

            if scope.is_empty() {
                hx.remove(scope_name);
            }
        });

        hx
    }
}

#[derive(Serialize, Debug)]
pub struct Scope {
    #[serde(rename = "fg")]
    pub fg: Option<String>,

    #[serde(rename = "bg")]
    pub bg: Option<String>,

    #[serde(rename = "modifiers")]
    pub modifiers: Option<Vec<String>>,
}

impl Scope {
    pub fn new(fg: Option<String>, bg: Option<String>, modifiers: Option<Vec<String>>) -> Self {
        Scope { fg, bg, modifiers }
    }

    pub fn fg(fg: &str) -> Self {
        Scope::new(Some(fg.to_string()), None, None)
    }

    pub fn is_empty(&self) -> bool {
        self.fg.is_none() && self.bg.is_none() && self.modifiers.is_none()
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new(None, None, None)
    }
}
