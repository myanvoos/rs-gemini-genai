pub use crate::api_client::GeminiClient;
pub use crate::generate_content_types::*;
pub use crate::request::*;

mod api_client;
mod models;
mod generate_content_types;
mod request;

#[cfg(test)]
mod tests {
    use crate::generate_content_types::{GenerateContentConfig, GenerateContentParameters};
    use super::*;

    #[test]
    fn init_client() {
        let client = GeminiClient::new("hello");
        assert_eq!(client.api_key, "hello");
    }

    // #[test]
    // fn init_models() {
    //     let client = GeminiClient::new("hello");
    //     let models = client.models();
    //     let config = GenerateContentConfig::new("Be nice to me");
    //     let params = GenerateContentParameters::new(
    //         "gemini-2.5-pro",
    //         "Hello there Gemini",
    //         &config
    //     );
    //     let response = models.generate_content(params);
    //     // TODO: Update this, this is outdated
    //     assert_eq!(response, "This is an example AI response")
    // }
}