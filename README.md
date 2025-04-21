## Rust API wrapper for Google Gemini APIs

rs-gemini-genai is a Rust wrapper around the Google Gemini API. It it heavily inspired by the [official Python SDK](https://github.com/googleapis/python-genai) and [this Postman workspace](https://www.postman.com/ai-on-postman/google-gemini-apis/overview).

### Example usage:
#### Initialize the client
```rust
// Initialize the Gemini API client with your API key
// Note: Make sure your API key is securely stored and is not publicly exposed
let client = GeminiClient::new("api_key");
```

#### Configure generation parameters
```rust
let models = client.models();

// Define the generate content parameters
let params = GenerateContentParameters::new(
    GeminiModels::Gemini20Flash,
    GeminiContents::Single("Hello there Gemini. How are you doing?".to_string()),
);

// Alternatively, use the builder pattern
let params = GenerateContentParametersBuilder::new()
    .model(GeminiModels::Gemini20Flash)
    .contents(
        GeminiContents::Multiple(
            vec![
                "Write me a 20 word poem".to_string(),
                "Then make the main character a turtle".to_string()
            ]
        ),
    )
    .build();
```
#### Text-only input
```rust
let response = models.generate_content(params)
    .await
    .unwrap_or("Failed to generate response".to_string());

println!("{:?}", response);
```
#### Generate a text stream
```rust
let stream = models.generate_content_stream(params).await?;

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
```
### Checklist:

- [x] Generate single part text content
- [x] Generate multi-parts text content
- [ ] Generate text content with system prompt
- [ ] Generate text content with generation configs
- [x] Generate text content with streaming