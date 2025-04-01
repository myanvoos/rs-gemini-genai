use crate::models::Model;

mod api_client;
mod models;

#[derive(Debug)]
pub struct GeminiClient {
    client: reqwest::Client,
    api_key: String
}

// Implement the `new` method for the Gemini client
// Usage: let client = GeminiClient::new(api_key)
// Staying true to the API design of the official SDKS, have a `client.models` module
// which exposes model inferencing and model getters
impl GeminiClient {
    pub fn new(api_client: String) -> Self {
        Self { client: reqwest::Client::new(), api_key: api_client }
    }

    // Return the 'model' service explicitly
    // Usage: GeminiClient::new(apiKey).models()
    pub fn models(&self) -> Model<'_> {
        Model { client: self }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_client() {
        let client = GeminiClient::new("hello".to_string());
        assert_eq!(client.api_key, "hello");
    }

    #[test]
    fn init_models() {
        let client = GeminiClient::new("hello".to_string());
        let models = client.models();
        let response = models.generate_content("hi");
        assert_eq!(response, "This is an example AI response")
    }
}