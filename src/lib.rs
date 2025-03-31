
mod api_client;

#[derive(Debug)]
pub struct GeminiClient {
    client: reqwest::Client,
    api_key: String
}

// Defines the 'model' module equivalent. It will have a lifetime that borrows from
// the Gemini client (has temporary access to data) and won't live longer than GeminiClient
pub struct Model<'a> {
    client: &'a GeminiClient
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

impl<'a> Model<'a> {
    pub fn generate_content(&self, query: &str) -> &str {
        "This is an example AI response"
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