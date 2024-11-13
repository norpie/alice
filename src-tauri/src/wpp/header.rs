use anyhow::Result;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderItem {
    name: String,
    full: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    template: String,
    system: Option<String>,
    characters: Vec<HeaderItem>,
    user: Option<HeaderItem>,
    scenario: Option<String>,
}

impl Header {
    pub fn evaluate(&self) -> Result<String> {
        let mut handlebars = Handlebars::new();
        handlebars.register_escape_fn(handlebars::no_escape);
        Ok(handlebars
            .render_template(TEMPLATE, &self)
            .map_err(|e| anyhow::anyhow!("Failed to render template: {}", e))?
            .trim()
            .into())
    }
}

static TEMPLATE: &str = r#"
{{#if system}}
{{system}}
{{/if}}

{{#each characters as |char|}}
{{char.name}}:
{{char.full}}
{{/each}}

{{#if user}}
{{user.name}}:
{{user.full}}
{{/if}}

{{#if scenario}}
Scenario:
{{scenario}}
{{/if}}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header() {
        let header = Header {
            template: "template".to_string(),
            system: Some("This is a roleplay.".to_string()),
            scenario: Some("User is chatting with characters.".to_string()),
            user: Some(HeaderItem {
                name: "user".to_string(),
                full: "[THIS IS USER WPP]".to_string(),
            }),
            characters: vec![
                HeaderItem {
                    name: "character 1".to_string(),
                    full: "[THIS IS CHARACTER 1 WPP]".to_string(),
                },
                HeaderItem {
                    name: "character 2".to_string(),
                    full: "[THIS IS CHARACTER 2 WPP]".to_string(),
                },
            ],
        };
        println!("{}", header.evaluate().unwrap());
    }
}
