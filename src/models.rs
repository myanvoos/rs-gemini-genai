use crate::api_client::GeminiClient;

// Use a basic GenerateContentConfig struct for now
#[derive(Debug, Clone)]
pub struct GenerateContentConfig {
    system_instruction: String,
}
pub struct GenerateContentParameters {
    model: String,
    contents: String,
    config: GenerateContentConfig
}

impl GenerateContentConfig {
    pub fn new(system_instruction: &str) -> Self {
        Self { system_instruction: system_instruction.to_string() }
    }
}
impl GenerateContentParameters {
    pub fn new(model: &str, contents: &str, config: &GenerateContentConfig) -> Self {
        Self {
            model: model.to_string(),
            contents: contents.to_string(),
            // What's happening here? I need to take a temporarily borrowed instance i.e. &GenerateContentConfig and convert it to an *owned* one i.e. GenerateContentConfig for the struct
            // Equivalent to *cloning* the borrowed instance. To do this I added the `Clone` trait to the struct allowing this
            config: config.clone(),
        }
    }
}

// Defines the 'model' module equivalent. It will have a lifetime that borrows from
// the Gemini client (has temporary access to data) and won't live longer than GeminiClient
// i.e. the reference to `GeminiClient` must outlive `Model`
pub struct Model<'a> {
    pub(crate) client: &'a GeminiClient
}

impl<'a> Model<'a> {
    pub fn generate_content(&self, generate_content_parameters: GenerateContentParameters) -> &str {
        println!("Received generate content parameters: {:?}", &generate_content_parameters);

        "This is an example AI response"
    }
}
