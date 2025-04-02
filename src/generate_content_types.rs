use strum_macros::{Display, EnumString};

// Use a basic GenerateContentConfig struct for now
#[derive(Debug, Clone)]
pub struct GenerateContentConfig {
    system_instruction: String,
}

#[derive(Debug, Clone, Display, EnumString)]
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

#[derive(Debug, Clone)]
pub enum GeminiContents {
    Single(String),
    Multiple(Vec<String>)
}

pub struct GenerateContentParameters {
    model: GeminiModels,
    contents: GeminiContents,
    config: GenerateContentConfig
}

impl GenerateContentConfig {
    pub fn new(system_instruction: &str) -> Self {
        Self { system_instruction: system_instruction.to_string() }
    }
}
impl GenerateContentParameters {
    pub fn new(model: GeminiModels, contents: GeminiContents, config: &GenerateContentConfig) -> Self {
        Self {
            model,
            contents,
            // What's happening here? I need to take a temporarily borrowed instance i.e. &GenerateContentConfig and convert it to an *owned* one i.e. GenerateContentConfig for the struct
            // Equivalent to *cloning* the borrowed instance. To do this I added the `Clone` trait to the struct allowing this
            config: config.clone(),
        }
    }
}