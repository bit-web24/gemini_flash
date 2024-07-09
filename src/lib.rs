use curl::easy::{Easy, List};
use neon::prelude::*;
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use serde_json::to_string;
mod image_search;

mod request;
mod response;
use request::*;
use image_search::image_search;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("generate_content", generate_content)?;
    cx.export_function("image_search", image_search)?;
    Ok(())
}

pub fn generate_content(mut cx: FunctionContext) -> JsResult<JsObject> {
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
    let response: response::Response = serde_json::from_str(json_data.as_str()).unwrap();
    let js_res_obj = to_jsobj(cx, response);

    Ok(js_res_obj.unwrap())
}

pub fn markdown_to_text(markdown: &str) -> String {
    let options = Options::empty();
    let parser = Parser::new_ext(&markdown, options);
    let mut plain_text = String::new();

    for event in parser {
        match event {
            Event::Text(text) | Event::Code(text) => plain_text.push_str(&text),
            Event::Start(tag) => match tag {
                Tag::Paragraph | Tag::Heading { .. } => plain_text.push_str("\n\n"),
                Tag::List(_) => plain_text.push('\n'),
                Tag::Item => plain_text.push_str("\n• "),
                _ => {}
            },
            Event::End(tag) => match tag {
                TagEnd::Paragraph | TagEnd::Heading { .. } => plain_text.push('\n'),
                _ => {}
            },
            _ => {}
        }
    }

    plain_text
}

fn to_jsobj(mut cx: FunctionContext, res: response::Response) -> JsResult<JsObject> {
    let obj = cx.empty_object();

    let text = cx.string(markdown_to_text(&res.candidates[0].content.parts[0].text));
    obj.set(&mut cx, "text", text)?;

    let finish_reason = cx.string(&res.candidates[0].finishReason);
    obj.set(&mut cx, "finish_reason", finish_reason)?;

    let usage_metadata = cx.string(to_string(&res.usageMetadata).unwrap());
    obj.set(&mut cx, "usage_metadata", usage_metadata)?;

    let role = cx.string(&res.candidates[0].content.role);
    obj.set(&mut cx, "role", role)?;

    let prompt_feedback = to_string(&res.candidates[0].safetyRatings).unwrap();
    let prompt_feedback_str = cx.string(prompt_feedback);
    obj.set(&mut cx, "prompt_feedback", prompt_feedback_str)?;

    Ok(obj)
}