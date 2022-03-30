use json_comments::StripComments;
use std::error::Error;

pub mod helix;
pub mod vscode;

pub fn vscode2helix(json: &str) -> Result<String, Box<dyn Error>> {
    let json = StripComments::new(json.as_bytes());
    let vs = serde_json::from_reader::<StripComments<&[u8]>, vscode::Theme>(json)?;
    let hx: helix::Theme = (&vs).into();
    let hx_text = toml::to_string_pretty(&hx)?;

    Ok(hx_text)
}
