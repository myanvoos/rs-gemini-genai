use crate::GeminiClient;

// Defines the 'model' module equivalent. It will have a lifetime that borrows from
// the Gemini client (has temporary access to data) and won't live longer than GeminiClient
pub struct Model<'a> {
    pub(crate) client: &'a GeminiClient
}

impl<'a> Model<'a> {
    pub fn generate_content(&self, query: &str) -> &str {
        "This is an example AI response"
    }
}
