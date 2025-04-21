use std::error::Error;
use std::io::Write;
use std::time::Duration;
use dotenv::dotenv;
use tokio::time::sleep;
use tokio_stream::StreamExt;
use rs_gemini_genai::GeminiClient;
use rs_gemini_genai::types::{GeminiContents, GeminiModels, GenerateContentConfig, GenerateContentParameters};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the environment variables
    dotenv().ok();

    let client = GeminiClient::new(
        std::env::var("GEMINI_API_KEY")
            .unwrap_or("No API key found!".to_string()).as_str()
    );
    let models = client.models();
    let params = GenerateContentParameters::new(
        GeminiModels::Gemini20Flash,
        GeminiContents::Single("Write me a 200 word story".to_string()),
        GenerateContentConfig::new("Talk like a pirate.")
    );

    // let response = models.generate_content(params).await.unwrap_or("Failed to generate response".to_string());
    // println!("{:?}", response);

    let stream = models.generate_content_stream(params).await?;
    tokio::pin!(stream);
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(text) => {
                sleep(Duration::from_millis(250)).await;
                print!("{}", text);
                std::io::stdout().flush().unwrap();
            }
            Err(error) => {
                eprintln!("Streaming error: {}", error);
            }
        }
    }

    Ok(())
}


