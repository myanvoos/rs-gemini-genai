use dotenv::dotenv;
use rs_gemini_genai::{GeminiClient, GeminiContents, GeminiModels, GenerateContentConfig, GenerateContentParameters, GenerateContentParametersBuilder};

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

    assert_eq!(params, params_builder);

    let response = models.generate_content(params).await.unwrap_or("Failed to generate response".to_string());
    println!("{:?}", response);
}


