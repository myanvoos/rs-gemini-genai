use dotenv::dotenv;
use serde_json::{json, to_string_pretty};
use rs_gemini_genai::{Content, GeminiClient, GeminiContents, GeminiModels, GenerateContentConfig, GenerateContentParameters, GenerateContentParametersBuilder, HttpRequestBody, HttpRequestBuilder, Part};

#[tokio::main]
async fn main() {
    // Load the environment variables
    dotenv().ok();

    let client = GeminiClient::new(
        std::env::var("GEMINI_API_KEY")
            .unwrap_or("No API key found!".to_string()).as_str()
    );
    let models = client.models();
    let params = GenerateContentParameters::new(
        GeminiModels::Gemini20Flash,
        GeminiContents::Single("Hello there Gemini. How are you doing?".to_string()),
        GenerateContentConfig::new("Be nice to me")
    );

    let params_builder = GenerateContentParametersBuilder::new()
        .model(GeminiModels::Gemini20Flash)
        .contents(GeminiContents::Single("Hello there Gemini. How are you doing?".to_string()))
        .config(GenerateContentConfig::new("Be nice to me"))
        .build();

    let response = models.generate_content(params).await.unwrap_or("Failed to generate response".to_string());
    println!("{:?}", response);
}


