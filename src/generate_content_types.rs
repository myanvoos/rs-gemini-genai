use strum_macros::{Display, EnumString};

// Use a basic GenerateContentConfig struct for now
#[derive(Debug, Clone, PartialEq)]
pub struct GenerateContentConfig {
    system_instruction: String,
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

impl GenerateContentConfig {
    pub fn new(system_instruction: &str) -> Self {
        Self { system_instruction: system_instruction.to_string() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenerateContentParameters {
    pub model: GeminiModels,
    pub contents: GeminiContents,
    pub config: GenerateContentConfig
}

#[derive(Debug, Default, Clone)]
pub struct GenerateContentParametersBuilder {
    model: Option<GeminiModels>,
    contents: Option<GeminiContents>,
    config: Option<GenerateContentConfig>
}

impl GenerateContentParameters {
    pub fn new(model: GeminiModels, contents: GeminiContents, config: &GenerateContentConfig) -> Self {
        Self {
            model,
            contents,
            config: config.clone(),
        }
    }
}

impl GenerateContentParametersBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn model(mut self, model: GeminiModels) -> Self {
        self.model = Some(model);
        self
    }
    pub fn contents(mut self, contents: GeminiContents) -> Self {
        self.contents = Some(contents);
        self
    }
    pub fn config(mut self, config: GenerateContentConfig) -> Self {
        self.config = Some(config);
        self
    }
    pub fn build(self) -> GenerateContentParameters {
        GenerateContentParameters {
            model: self.model.unwrap(),
            contents: self.contents.unwrap(),
            config: self.config.unwrap(),
        }
    }
}
