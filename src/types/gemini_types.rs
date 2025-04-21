use serde::Serialize;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct Part {
    pub text: String,
}

impl Part {
    pub fn new(text_value: String) -> Self {
        Self { text: text_value }
    }
}


#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct Content {
    pub parts: Vec<Part>
}

impl Content {
    pub fn new(parts: Vec<Part>) -> Self {
        Content { parts }
    }
}

#[derive(Debug, Clone, Display, EnumString, PartialEq)]
pub enum GeminiModels {
    #[strum(serialize = "gemini-1.5-pro")]
    Gemini15Pro,
    #[strum(serialize = "gemini-2.0-flash-001")]
    Gemini20Flash,
    #[strum(serialize = "gemini-2.0-flash-thinking-exp")]
    Gemini20FlashThinkingExperimental,
    #[strum(serialize = "gemini-2.5-pro-exp")]
    Gemini25ProExperimental,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GeminiContents {
    Single(String),
    Multiple(Vec<String>)
}