use std::collections::HashMap;

use reqwest::header;
use worker::Url;

pub mod security;
pub mod data;

pub fn default_headers() -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert("User-Agent", header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"));
    headers.insert(
        "Content-Type",
        header::HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers
}

pub fn get_params(url: Url) -> HashMap<String, String> {
    let url = Url::try_from(url.as_str()).unwrap();
    let mut param_map: HashMap<String, String> = HashMap::new();
    for (key, val) in url.query_pairs() {
        param_map.insert(key.to_string(), val.to_string());
    }

    param_map
}

pub fn get_tag_attribute(text: &str, query: &str, key: &str) -> String {
    let mut dom = tl::parse(text, tl::ParserOptions::default()).unwrap();
    let anchor = dom
        .query_selector(query)
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

    let attribute = attributes
        .get_mut(key)
        .flatten()
        .expect("Attribute not found or malformed");

    attribute.as_utf8_str().to_string()
}