use std::collections::HashMap;
use reqwest::Error;
use serde_json::{json, Value};
use crate::api_client::GeminiClient;
use crate::{Content, GeminiContents, GenerateContentParameters, HttpRequestBody, HttpRequestBuilder, Part};

// Defines the 'model' module equivalent. It will have a lifetime that borrows from
// the Gemini client (has temporary access to data) and won't live longer than GeminiClient
// i.e. the reference to `GeminiClient` must outlive `Model`
pub struct Model<'a> {
    pub(crate) client: &'a GeminiClient
}

impl<'a> Model<'a> {
    pub async fn generate_content(&self, generate_content_parameters: GenerateContentParameters) -> Result<String, Error> {
        println!("Received generate content parameters: {:?}", &generate_content_parameters);

        let model = generate_content_parameters.model;

        let parts: Vec<Part> = match  generate_content_parameters.contents {
            GeminiContents::Single(text) => {
                [ Part::new(text) ].to_vec()
            }
            GeminiContents::Multiple(texts) => {
                texts.into_iter()
                    .map(|text| Part::new(text))
                    .collect()
            }
        };

        let contents = Content::new(parts);
        let request_body = HttpRequestBody::new([ contents ].to_vec());
        let http_request = HttpRequestBuilder::new()
            .model(model.to_string())
            .api_key(self.client.api_key().parse().unwrap())
            .request_body(request_body)
            .build();

        let res = http_request.post().await?;

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
