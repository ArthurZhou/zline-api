use std::collections::HashMap;
use std::fmt;

use reqwest::header;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use worker::Url;

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
            return json!({ "code": 502, "message": format!("unable to perform request: {}", e) })
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

pub async fn login(req: worker::Request) -> serde_json::Value {
    let url = Url::try_from(req.url().unwrap()).unwrap();
    let mut xtoken = "".to_string();
    let mut username = "".to_string();
    let mut password = "".to_string();
    for (key, val) in url.query_pairs() {
        match key {
            std::borrow::Cow::Borrowed("xtoken") => xtoken = val.to_string(),
            std::borrow::Cow::Borrowed("username") => username = val.to_string(),
            std::borrow::Cow::Borrowed("password") => password = val.to_string(),
            _ => {}
        }
    }

    let mut params = HashMap::new();
    params.insert("XToken", xtoken);
    params.insert("pzlusername", username);
    params.insert("pzlpassword", password);

    let client = Client::new();
    let resp = match client
        .post("https://www.jincai.sh.cn/zlineauthrize/xlogin/sysxlogin")
        .headers(default_headers())
        .form(&params)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            return json!({ "code": 502, "message": format!("unable to perform request: {}", e) })
        }
    };

    let cookie = match resp.headers().get("Set-Cookie") {
        Some(cookie) => cookie.to_str().unwrap().to_string(),
        None => "".to_string(),
    };
    let text = match resp.text().await {
        Ok(text) => text,
        Err(e) => {
            return json!({ "code": 500, "message": format!("failed to read response text: {}", e) })
        }
    };
    let json: serde_json::Value = match serde_json::from_str(&text) {
        Ok(json) => json,
        Err(e) => {
            return json!({ "code": 500, "message": format!("failed to decode response(maybe a remote server issue): {}", e) })
        }
    };

    if json["succeed"] == "1" {
        json!({ "code": 200, "message": "ok", "data": {
            "cookie": cookie.split(" ").next().unwrap_or("")
        } })
    } else {
        json!({ "code": 401, "message": json["errorMsg"] })
    }
}

pub async fn logout() -> serde_json::Value {
    let client = Client::new();
    let _ = match client
        .get("https://www.jincai.sh.cn/zlinesystem/xlogin/loginout")
        .headers(default_headers())
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            return json!({ "code": 502, "message": format!("unable to perform request: {}", e) })
        }
    };
    json!({ "code": 200, "message": "ok" })
}

pub async fn status(req: worker::Request) -> serde_json::Value {
    let url = Url::try_from(req.url().unwrap()).unwrap();
    let mut cookie = "".to_string();
    for (key, val) in url.query_pairs() {
        match key {
            std::borrow::Cow::Borrowed("cookie") => cookie = val.to_string(),
            _ => {}
        }
    }

    let client = Client::new();
    let resp = match client
        .get("https://www.jincai.sh.cn/zlinesystem/hdesk")
        .headers(default_headers())
        .header("Cookie", cookie)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            return json!({ "code": 502, "message": format!("unable to perform request: {}", e) })
        }
    };

    if resp.url().to_string() == "https://www.jincai.sh.cn/zlinesystem/hdesk" {
        json!({ "code": 200, "message": "ok", "data": { "valid": true } })
    } else {
        json!({ "code": 401, "message": "invalid cookie", "data": { "valid": false } })
    }
}
