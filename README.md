## Rust API wrapper for the Google Gemini API
`rs-gemini-genai` is a Rust wrapper around the Google Gemini API. It's heavily inspired by the [official Python SDK](https://github.com/googleapis/python-genai).

### Example usage:
```rust
    // Initialize the Gemini API client with your API key
    // Note: Make sure your API key is securely stored and is not publicly exposed
    let client = GeminiClient::new(
        std::env::var("GEMINI_API_KEY")
            .unwrap_or("No API key found!".to_string()).as_str()
    );
    let models = client.models();

    // Define the generate content parameters
    let params = GenerateContentParameters::new(
        GeminiModels::Gemini20Flash,
        GeminiContents::Single("Hello there Gemini. How are you doing?".to_string()),
    );

    // Alternatively, use the builder pattern
    let params_builder = GenerateContentParametersBuilder::new()
        .model(GeminiModels::Gemini20Flash)
        .contents(GeminiContents::Single("Hello there Gemini. How are you doing?".to_string()))
        .build();

    // Generate a response
    let response = models.generate_content(params)
        .await
        .unwrap_or("Failed to generate response".to_string());

    println!("{:?}", response);
```

### Checklist:

- [ ] Generate single part text content
- [ ] Generate multi-parts text content
- [ ] Generate text content with system prompt
- [ ] Generate text content with generation configs
- [ ] Generate text content with streaming