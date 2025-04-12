use std::collections::HashMap;
use reqwest::Error;
use serde_json::{json, Value};
use crate::api_client::GeminiClient;
use crate::{GeminiContents, GenerateContentParameters};

// Defines the 'model' module equivalent. It will have a lifetime that borrows from
// the Gemini client (has temporary access to data) and won't live longer than GeminiClient
// i.e. the reference to `GeminiClient` must outlive `Model`
pub struct Model<'a> {
    pub(crate) client: &'a GeminiClient
}

const GEMINI_API_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models/";

impl<'a> Model<'a> {
    pub async fn generate_content(&self, generate_content_parameters: GenerateContentParameters) -> Result<String, Error> {
        println!("Received generate content parameters: {:?}", &generate_content_parameters);

        let model = generate_content_parameters.model;

        let parts_json: Vec<Value> = match  generate_content_parameters.contents {
            GeminiContents::Single(text) => {
                vec![json!({ "text": text })]
            }
            GeminiContents::Multiple(texts) => {
                texts.into_iter()
                    .map(|text| json!({ "text": text }))
                    .collect()
            }
        };

        let contents_json = vec![json!({ "parts": parts_json })];

        let mut request_body_map = serde_json::Map::new();
        request_body_map.insert("contents".to_string(), Value::Array(contents_json));

        let client = reqwest::Client::new();
        let res = client.post(GEMINI_API_URL.to_owned() + &*model.to_string() + ":generateContent?key=" + self.client.api_key())
            .json(&Value::Object(request_body_map))
            .send()
            .await?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let response_json: Value = res.json().await?;
                Ok(response_json["candidates"][0]["content"]["parts"][0]["text"].as_str().unwrap_or("").to_string())
            },
            _ => {
                let response: Value = res.json().await?;
                println!("Response: {}", response);
                Ok("Something went wrong".to_string())
            }
        }
    }
}
