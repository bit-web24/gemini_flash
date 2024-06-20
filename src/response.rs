use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub candidates: Vec<Candidate>,
    pub usageMetadata: UsageMetadata,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Candidate {
    pub content: Content,
    pub finishReason: String,
    pub index: u32,
    pub safetyRatings: Vec<SafetyRating>,
    pub citationMetadata: CitationMetadata,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Part {
    pub text: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SafetyRating {
    pub category: String,
    pub probability: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CitationMetadata {
    pub citationSources: Vec<CitationSource>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CitationSource {
    pub startIndex: u32,
    pub endIndex: u32,
    pub uri: String,
    pub license: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct UsageMetadata {
    pub promptTokenCount: u32,
    pub candidatesTokenCount: u32,
    pub totalTokenCount: u32,
}

// impl Response {
//     pub fn text(&self) -> String {
//         utils::markdown_to_text(self.candidates[0].content.parts[0].text.as_str())
//     }

//     pub fn prompt_feedback(&self) -> &Vec<SafetyRating> {
//         self.candidates[0].safetyRatings.as_ref()
//     }

//     pub fn candidates(&self) -> &Vec<Candidate> {
//         self.candidates.as_ref()
//     }
// }