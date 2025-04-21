use std::collections::HashMap;
use std::io::Write;
use std::time::Duration;
use reqwest::{Error, Response};
use serde_json::{json, to_string_pretty, Value};
use tokio::time::sleep;
use tokio_stream::{Stream, StreamExt};
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;
use crate::api_client::GeminiClient;
use crate::types::{Content, Part, GeminiContents, GenerateContentParameters};
use crate::{HttpRequestBody, HttpRequestBuilder, ModelMethod};

// Defines the 'model' module equivalent. It will have a lifetime that borrows from
// the Gemini client (has temporary access to data) and won't live longer than GeminiClient
// i.e. the reference to `GeminiClient` must outlive `Model`
pub struct Model<'a> {
    pub(crate) client: &'a GeminiClient
}

impl<'a> Model<'a> {
    fn extract_response(&self, response_json: Value) -> String {
        response_json["candidates"][0]["content"]["parts"][0]["text"].as_str().unwrap_or("").to_string()
    }
    fn construct_request_body(&self, generate_content_parameters: &GenerateContentParameters) -> HttpRequestBody {
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
        println!("DEV: Constructed request body: {}", to_string_pretty(&request_body).unwrap());
        request_body
    }

    async fn handle_response_return(&self, res: Response) -> Result<String, Error> {
        match res.status() {
            reqwest::StatusCode::OK => {
                let response_json: Value = res.json().await?;
                Ok(self.extract_response(response_json))
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

    pub async fn generate_content_stream(
        &self,
        generate_content_parameters: GenerateContentParameters,
    ) -> Result<impl Stream<Item = Result<String, Error>>, Error> {
        let request_body = self.construct_request_body(&generate_content_parameters);

        let http_request = HttpRequestBuilder::new()
            .model(generate_content_parameters.model.to_string())
            .api_key(self.client.api_key().parse().unwrap())
            .request_body(request_body)
            .method(ModelMethod::GenerateContentStream)
            .build();

        let res = http_request.post().await?;

        let mut stream =  res.bytes_stream();

        // set up a channel where:
        // - tx is used in the background task to send each chunk
        // - rx is used by the calling code to receive streamed output
        // - buffer is how many messages can be waiting in the queue
        let (tx, rx) = mpsc::channel(10);

        // run the future immediately on the Tokio runtime in the background, no need for async-await
        tokio::spawn(async move {
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(chunk) => {
                        let text = String::from_utf8_lossy(&chunk);
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let json_str = &line[6..]; // strip off "data: "
                                if let Ok(json_value) = serde_json::from_str::<Value>(json_str) {
                                    if let Some(text_part) = json_value
                                        .get("candidates")
                                        .and_then(|c| c.get(0))
                                        .and_then(|c| c.get("content"))
                                        .and_then(|content| content.get("parts"))
                                        .and_then(|parts| parts.get(0))
                                        .and_then(|part| part.get("text"))
                                        .and_then(|t| t.as_str())
                                    {
                                        // every time we parse a text_part, we send it to the rx stream
                                        if tx.send(Ok(text_part.to_string())).await.is_err() {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => {
                        let _ = tx.send(Err(err)).await;
                        break;
                    }
                }
            }
        });

        Ok(ReceiverStream::new(rx))

    }
}
