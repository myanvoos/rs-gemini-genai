use rs_gemini_genai::{GeminiClient, GeminiContents, GeminiModels, GenerateContentConfig, GenerateContentParameters, GenerateContentParametersBuilder};

fn main() {
    let client = GeminiClient::new("GEMINI_API_KEY");
    let models = client.models();
    let config = GenerateContentConfig::new("Be nice to me");
    let params = GenerateContentParameters::new(
        GeminiModels::Gemini20Flash,
        GeminiContents::Single("Hello there Gemini".to_string()),
        &config
    );

    let params_builder = GenerateContentParametersBuilder::new()
        .model(GeminiModels::Gemini20Flash)
        .contents(GeminiContents::Single("Hello there Gemini".to_string()))
        .config(config)
        .build();

    assert_eq!(params, params_builder);

    let response = models.generate_content(params);
}


