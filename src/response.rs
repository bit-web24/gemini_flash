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
