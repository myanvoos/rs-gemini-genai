pub use crate::api_client::GeminiClient;
pub use crate::types::*;
pub use crate::request::*;

mod api_client;
mod models;
pub mod types;
mod request;

#[cfg(test)]
mod tests {
    use crate::types::{GenerateContentConfig, GenerateContentParameters};
    use super::*;

    #[test]
    fn init_client() {
        let client = GeminiClient::new("hello");
        assert_eq!(client.api_key, "hello");
    }
}