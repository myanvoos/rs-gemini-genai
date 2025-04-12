use reqwest::{Error, Response};
use serde_json::{json, Map, Value};
use serde::Serialize;
use crate::models::Model;

const GEMINI_API_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models/";

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Part {
    pub text: String,
}

impl Part {
    pub fn new(text_value: String) -> Self {
        Self { text: text_value }
    }
}


#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Content {
    pub parts: Vec<Part>
}

impl Content {
    pub fn new(parts: Vec<Part>) -> Self {
        Content { parts }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct HttpRequestBody {
    pub contents: Vec<Content>
}

impl HttpRequestBody {
    pub fn new(contents: Vec<Content>) -> Self {
        Self { contents }
    }
}

struct HttpRequestClient {
    client: reqwest::Client,
    request_body: HttpRequestBody,
    model_id: String,
    api_key: String
}

impl HttpRequestClient {
    pub async fn post(self) -> Result<Response, Error> {
        let mut request_body_map = serde_json::Map::new();
        // request_body_map.insert("contents".to_string(), Value::Array(self.request_body.contents));

        self.client.post(GEMINI_API_URL.to_owned() + &*self.model_id.to_string() + ":generateContent?key=" + &*self.api_key)
            .json(&Value::Object(request_body_map))
            .send()
            .await
    }
}

pub struct HttpRequestBuilder {
    pub request_body: HttpRequestBody,
    pub model_id: String,
    api_key: String
}

impl HttpRequestBuilder {
    pub fn new(mut self, api_key: &str) -> Self {
        self.api_key = api_key.to_string();
        self
    }
    pub fn model(mut self, model_id: &str) -> Self {
        self.model_id = model_id.to_string();
        self
    }
    pub fn request_body(mut self, request_body: HttpRequestBody) -> Self {
        self.request_body = request_body;
        self
    }
    pub fn build(self) -> HttpRequestClient {
        let client = reqwest::Client::new();
        HttpRequestClient {
            client,
            request_body: self.request_body,
            model_id: self.model_id,
            api_key: self.api_key
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::to_string_pretty;
    use super::*;

    #[test]
    fn test_json_structure() {
        let part = Part::new("Write a story about a magic backpack.".to_string());
        let part_vec = [ part ].to_vec();
        let contents = Content::new(part_vec);
        let req_body = HttpRequestBody::new([ contents ].to_vec());

        let serialized_value = to_string_pretty(&req_body).unwrap();
        let expected_json = to_string_pretty(&json!(
        {
          "contents": [{
            "parts":[{
              "text": "Write a story about a magic backpack."
              }]
            }]
        })).unwrap();
        assert_eq!(serialized_value, expected_json);
    }
}