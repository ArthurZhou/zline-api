use std::collections::HashMap;

use reqwest::Client;
use serde_json::json;

use crate::api::{default_headers, get_params};

pub async fn exam_list(req: worker::Request) -> serde_json::Value {
    fn processor(source: String, id: String) -> serde_json::Value {
        let dom = tl::parse(&source, tl::ParserOptions::default()).unwrap();
        let parser = dom.parser();
        let element = dom
            .get_element_by_id(&*id)
            .expect("Failed to find element")
            .get(parser)
            .unwrap();
        let binding = element.inner_html(parser).to_string();
        let t = binding.split("</option>");

        let mut map: HashMap<&str, &str> = HashMap::new();
        for child in t {
            if child.contains("value") {
                let p1: Vec<&str> = child.split("\">").collect();
                let label = p1[1];
                let p2: Vec<&str> = p1[0].split("=\"").collect();
                let id = p2[1];
                map.insert(label, id);
            }
        }

        json!(map)
    }

    let params = get_params(req.url().unwrap());

    let client = Client::new();
    let resp = match client
        .get("https://www.jincai.sh.cn/cjcx/student/ajax/cjhztable.php")
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

    if resp.url().to_string() == "https://www.jincai.sh.cn/cjcx/student/ajax/cjhztable.php" {
        let text = resp.text().await.unwrap();
        let time = processor(text.clone(), "selektime".to_string());
        let subject = processor(text.clone(), "selekid".to_string());

        json!({ "code": 200, "message": "ok", "data": {
            "time": time,
            "subject": subject
        }})
    } else {
        json!({ "code": 401, "message": "invalid cookie", "data": { "valid": false } })
    }
}
