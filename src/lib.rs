
mod api_client;

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


}