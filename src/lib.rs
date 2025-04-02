pub use crate::api_client::GeminiClient;
pub use crate::models::{GenerateContentConfig, GenerateContentParameters};

mod api_client;
mod models;

#[cfg(test)]
mod tests {
    use crate::models::{GenerateContentConfig, GenerateContentParameters};
    use super::*;

    #[test]
    fn init_client() {
        let client = GeminiClient::new("hello");
        assert_eq!(client.api_key, "hello");
    }

    #[test]
    fn init_models() {
        let client = GeminiClient::new("hello");
        let models = client.models();
        let config = GenerateContentConfig::new("Be nice to me");
        let params = GenerateContentParameters::new(
            "gemini-2.5-pro",
            "Hello there Gemini",
            &config
        );
        let response = models.generate_content(params);
        // TODO: Update this, this is outdated
        assert_eq!(response, "This is an example AI response")
    }
}