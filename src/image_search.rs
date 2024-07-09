use curl::easy::{Easy, List};
use neon::{context::FunctionContext, result::JsResult, types::{JsObject, JsString}};
use serde_json::to_string;
use crate::{response, to_jsobj};
use crate::request::{ImgContent, ImgPart, InlineData, Root};

pub fn image_search(mut cx: FunctionContext) -> JsResult<JsObject> {
    let api_key = cx.argument::<JsString>(0)?.value(&mut cx);
    let prompt = cx.argument::<JsString>(1)?.value(&mut cx);
    let base64_image = cx.argument::<JsString>(2)?.value(&mut cx);

    let mut easy = Easy::new();

    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}", api_key);
    easy.url(&url).unwrap();

    easy.post(true).unwrap();

    let mut headers = List::new();
    headers.append("Content-Type: application/json").unwrap();
    easy.http_headers(headers).unwrap();

    let request = Root {
        contents: vec![
            ImgContent {
                parts: vec![
                    ImgPart {
                        text: Some(prompt),
                        inline_data: None,
                    },
                    ImgPart {
                        text: None,
                        inline_data: Some(InlineData {
                            mime_type: "image/jpeg".to_string(),
                            data: base64_image,
                        }),
                    },
                ],
            },
        ],
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
