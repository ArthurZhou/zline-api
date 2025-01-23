use std::fmt;

use reqwest::header;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug)]
pub struct Response {
    pub code: u16,
    message: String,
    data: String,
}

impl Default for Response {
    fn default() -> Self {
        Response {
            code: 200,
            message: "ok".to_string(),
            data: "".to_string(),
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{\"code\":\"{}\",\"message\":\"{}\",\"data\":\"{}\"}}",
            self.code, self.message, self.data
        )
    }
}

fn default_headers() -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert("User-Agent", header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"));
    headers.insert(
        "Content-Type",
        header::HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers
}

pub async fn get_xtoken() -> serde_json::Value {
    let client = Client::new();

    let resp = match client
        .get("https://www.jincai.sh.cn/zlineauthrize/xlogin")
        .headers(default_headers())
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            return json!({ "code": 502, "message": format!("unable to perform request: {}", e.to_string()).to_string() })
        }
    };
    if !resp.status().is_success() {
        return json!({ "code": resp.status().as_u16(), "message": resp.status().canonical_reason().unwrap_or("unknown").to_string() });
    }

    let text: String = match resp.text().await {
        Ok(text) => text,
        Err(_) => return json!({ "code": 500, "message": "failed to parse response".to_string() }),
    };

    let mut dom = tl::parse(&text, tl::ParserOptions::default()).unwrap();
    let anchor = dom
        .query_selector("#XToken[value]")
        .expect("Failed to parse query selector")
        .next()
        .expect("Failed to find anchor tag");
    let parser_mut = dom.parser_mut();

    let anchor = anchor
        .get_mut(parser_mut)
        .expect("Failed to resolve node")
        .as_tag_mut()
        .expect("Failed to cast Node to HTMLTag");

    let attributes = anchor.attributes_mut();

    let xtoken = attributes
        .get_mut("value")
        .flatten()
        .expect("Attribute not found or malformed");

    json!({
        "code": 200,
        "message": "ok",
        "data": {
            "xtoken": xtoken.as_utf8_str().to_string()
        }
    })
}
