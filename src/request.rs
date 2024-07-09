use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Part {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub contents: Vec<Content>,
}


// Structs for Text+Image prompt

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineData {
    pub mime_type: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImgPart {
    pub text: Option<String>,
    pub inline_data: Option<InlineData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImgContent {
    pub parts: Vec<ImgPart>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    pub contents: Vec<ImgContent>,
}
