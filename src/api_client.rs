use crate::models::Model;

#[derive(Debug)]
pub struct GeminiClient {
    client: reqwest::Client,
    // Use String here because a String *owns* its data and stores it on the heap.
    // Which means the struct takes *ownership* of a copy of the string data
    pub(crate) api_key: String
}

// Implement the `new` method for the Gemini client
// Usage: let client = GeminiClient::new(api_key)
// Staying true to the API design of the official SDKS, have a `client.models` module
// which exposes model inferencing and model getters
impl GeminiClient {
    // The reason why we accept api_key as a &str instead of a String, then convert it to a
    // String to fit with the struct is because we want to allow more flexibility to the user
    pub fn new(api_key: &str) -> Self {
        Self { client: reqwest::Client::new(), api_key: api_key.to_string() }
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    // Return the 'model' service explicitly
    // Usage: GeminiClient::new(apiKey).models()
    pub fn models(&self) -> Model<'_> {
        Model { client: self }
    }
}