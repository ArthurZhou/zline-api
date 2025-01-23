use reqwest::{Client, Error};
use serde_json::json;
use std::collections::HashMap;

use crate::api::{default_headers, get_params, get_tag_attribute};

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
    let xtoken = get_tag_attribute(&text, "#XToken[value]", "value");

    json!({
        "code": 200,
        "message": "ok",
        "data": {
            "xtoken": xtoken
        }
    })
}

pub async fn login(req: worker::Request) -> serde_json::Value {
    let params: HashMap<String, String> = get_params(req.url().unwrap());

    let mut form = HashMap::new();
    form.insert(
        "XToken",
        params
            .get("xtoken")
            .map_or("".to_string(), |v| v.to_string()),
    );
    form.insert(
        "pzlusername",
        params
            .get("username")
            .map_or("".to_string(), |v| v.to_string()),
    );
    form.insert(
        "pzlpassword",
        params
            .get("password")
            .map_or("".to_string(), |v| v.to_string()),
    );

    let client = Client::new();
    let resp = match client
        .post("https://www.jincai.sh.cn/zlineauthrize/xlogin/sysxlogin")
        .headers(default_headers())
        .form(&form)
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
        let php_cookie = match data_login(&cookie).await {
                    Ok(t) => t.clone().split(" ").next().unwrap_or("").to_string(),
                    Err(e) => {
                    return json!({ "code": 500, "message": format!("failed to decode response(maybe a remote server issue): {}", e) })
                }};
        json!({ "code": 200, "message": "ok", "data": {
            "cookie": cookie.split(" ").next().unwrap_or("").to_string() + &php_cookie
        } })
    } else {
        json!({ "code": 401, "message": json["errorMsg"] })
    }
}

async fn data_login(cookie: &str) -> Result<String, Error> {
    let client = Client::new();
    let resp = match client
        .get("https://www.jincai.sh.cn/zlinesystem/xsso/gotox/JCAPW1002?pzlsid=pz6CE59B351CA97621C593D89ADBDB57E8&ticket=pz6CE59B351CA97621C593D89ADBDB57E8")
        .headers(default_headers())
        .header("Cookie", cookie)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => return Err(e)
    };

    let text: String = match resp.text().await {
        Ok(text) => text,
        Err(e) => return Err(e)
    };
    let xuid = get_tag_attribute(&text, "input[name=\"xuid\"]", "value");
    let xuxm = get_tag_attribute(&text, "input[name=\"xuxm\"]", "value");
    let xright = get_tag_attribute(&text, "input[name=\"xright\"]", "value");
    let xtimestamp = get_tag_attribute(&text, "input[name=\"xtimestamp\"]", "value");
    let rebackpage = get_tag_attribute(&text, "input[name=\"rebackpage\"]", "value");
    let signstr = get_tag_attribute(&text, "input[name=\"signstr\"]", "value");

    let mut form = HashMap::new();
    form.insert("xuid", xuid);
    form.insert("xuxm", xuxm);
    form.insert("xright", xright);
    form.insert("xtimestamp", xtimestamp);
    form.insert("rebackpage", rebackpage);
    form.insert("signstr", signstr);

    let client = Client::new();
    let resp = match client
        .post("https://www.jincai.sh.cn/cjcx/student/ajax/login0.php")
        .headers(default_headers())
        .header("Cookie", cookie)
        .form(&form)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => return Err(e)
    };

    let new_cookie = match resp.headers().get("Set-Cookie") {
        Some(cookie) => cookie.to_str().unwrap().to_string(),
        None => "".to_string(),
    };
    Ok(new_cookie)
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
    let params = get_params(req.url().unwrap());

    let client = Client::new();
    let resp = match client
        .get("https://www.jincai.sh.cn/zlinesystem/hdesk")
        .headers(default_headers())
        .header("Cookie", params.get("cookie").map_or("", |v| v))
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
