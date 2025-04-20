use std::collections::HashMap;
use reqwest::{Error, Response};
use serde_json::{json, Value};
use crate::api_client::GeminiClient;
use crate::{Content, GeminiContents, GenerateContentParameters, HttpRequestBody, HttpRequestBuilder, ModelMethod, Part};

// Defines the 'model' module equivalent. It will have a lifetime that borrows from
// the Gemini client (has temporary access to data) and won't live longer than GeminiClient
// i.e. the reference to `GeminiClient` must outlive `Model`
pub struct Model<'a> {
    pub(crate) client: &'a GeminiClient
}

impl<'a> Model<'a> {
    fn construct_request_body(&self, generate_content_parameters: &GenerateContentParameters) -> HttpRequestBody {
        println!("Received generate content parameters: {:?}", generate_content_parameters);

        let model = generate_content_parameters.clone().model;

        let parts: Vec<Part> = match  generate_content_parameters.clone().contents {
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
        request_body
    }

    async fn handle_response_return(&self, res: Response) -> Result<String, Error> {
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

    pub async fn generate_content(&self, generate_content_parameters: GenerateContentParameters) -> Result<String, Error> {
        let request_body = self.construct_request_body(&generate_content_parameters);

        let http_request = HttpRequestBuilder::new()
            .model(generate_content_parameters.model.to_string())
            .api_key(self.client.api_key().parse().unwrap())
            .request_body(request_body)
            .method(ModelMethod::GenerateContent)
            .build();

        let res = http_request.post().await?;

        self.handle_response_return(res).await
    }

    pub async fn generate_content_stream(&self, generate_content_parameters: GenerateContentParameters) -> Result<String, Error> {
        let request_body = self.construct_request_body(&generate_content_parameters);

        let http_request = HttpRequestBuilder::new()
            .model(generate_content_parameters.model.to_string())
            .api_key(self.client.api_key().parse().unwrap())
            .request_body(request_body)
            .method(ModelMethod::GenerateContentStream)
            .build();

        let res = http_request.post().await?;

        self.handle_response_return(res).await
    }
}
