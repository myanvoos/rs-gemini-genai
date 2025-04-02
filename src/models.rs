use crate::api_client::GeminiClient;



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
