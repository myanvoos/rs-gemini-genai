use rs_gemini_genai::{GeminiClient, GenerateContentConfig, GenerateContentParameters};

fn main() {
    let client = GeminiClient::new("hello");
    let models = client.models();
    let config = GenerateContentConfig::new("Be nice to me");
    let params = GenerateContentParameters::new(
        "gemini-2.5-pro",
        "Hello there Gemini",
        &config
    );
    let response = models.generate_content(params);
}
