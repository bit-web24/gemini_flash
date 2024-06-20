use curl::easy::{Easy, List};
use neon::prelude::*;
use serde_json::to_string;
use pulldown_cmark::{Event, Parser, Tag, TagEnd, Options};

mod request;
mod response;
use request::*;
// use response::Response;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("generate_content", generate_content)?;
    cx.export_function("markdown_to_text", markdown_to_text)?;
    Ok(())
}

// @args api_key: String, prompt: &str
pub fn generate_content(mut cx: FunctionContext) -> JsResult<JsString> {
    let api_key = cx.argument::<JsString>(0)?.value(&mut cx);
    let prompt = cx.argument::<JsString>(1)?.value(&mut cx);

    let mut easy = Easy::new();

    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}", api_key);
    easy.url(&url).unwrap();

    easy.post(true).unwrap();

    let mut headers = List::new();
    headers.append("Content-Type: application/json").unwrap();
    easy.http_headers(headers).unwrap();

    let request: Request = Request {
        contents: vec![Content {
            parts: vec![Part { text: prompt }],
        }],
    };

    let data_string = to_string(&request).unwrap();
    easy.post_fields_copy(data_string.as_bytes()).unwrap();

    let mut response_data = Vec::new();

    {
        let mut transfer = easy.transfer();

        transfer
            .write_function(|data| {
                response_data.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();

        transfer.perform().unwrap();
    }

    let json_data = String::from_utf8(response_data).unwrap();
    // let response: Response = serde_json::from_str(json_data.as_str()).unwrap();
    // response
    Ok(cx.string(json_data))
}

pub fn markdown_to_text(mut cx: FunctionContext) -> JsResult<JsString> {
    let markdown = cx.argument::<JsString>(0)?.value(&mut cx);
    let options = Options::empty();
    let parser = Parser::new_ext(&markdown, options);
    let mut plain_text = String::new();

    for event in parser {
        match event {
            Event::Text(text) | Event::Code(text) => plain_text.push_str(&text),
            Event::Start(tag) => match tag {
                Tag::Paragraph | Tag::Heading{..} => plain_text.push_str("\n\n"),
                Tag::List(_) => plain_text.push('\n'),
                Tag::Item => plain_text.push_str("\n• "),
                _ => {},
            },
            Event::End(tag) => match tag {
                TagEnd::Paragraph | TagEnd::Heading{..} => plain_text.push('\n'),
                _ => {},
            },
            _ => {}
        }
    }

    Ok(cx.string(plain_text))
}