use rs_gemini_genai::{GeminiClient, GeminiContents, GeminiModels, GenerateContentConfig, GenerateContentParameters};

fn main() {
    let client = GeminiClient::new("GEMINI_API_KEY");
    let models = client.models();
    let config = GenerateContentConfig::new("Be nice to me");
    let params = GenerateContentParameters::new(
        GeminiModels::Gemini20Flash,
        GeminiContents::Single("Hello there Gemini".to_string()),
        &config
    );
    let response = models.generate_content(params);
}
