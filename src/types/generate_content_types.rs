use crate::types::{GeminiContents, GeminiModels};

#[derive(Debug, Clone, PartialEq)]
pub struct GenerateContentConfig {
    system_instruction: String,
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
    pub fn new(model: GeminiModels, contents: GeminiContents, config: GenerateContentConfig) -> Self {
        Self {
            model,
            contents,
            config,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_content_params_builder() {
        let params = GenerateContentParameters::new(
            GeminiModels::Gemini20Flash,
            GeminiContents::Single("Hello there Gemini. How are you doing?".to_string()),
            GenerateContentConfig::new("Be nice to me")
        );

        let params_builder = GenerateContentParametersBuilder::new()
            .model(GeminiModels::Gemini20Flash)
            .contents(GeminiContents::Single("Hello there Gemini. How are you doing?".to_string()))
            .config(GenerateContentConfig::new("Be nice to me"))
            .build();

        assert_eq!(params, params_builder);
    }
}