use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;
use std::time::Instant;

#[derive(Debug, Clone , Copy , PartialEq , Eq)]
pub enum HttpMethod{
    POST,
    GET,
    PUT,
    DELETE,
    PATCH,
}

impl HttpMethod {
    pub fn to_request(&self) -> reqwest::Method {
        match self {
            HttpMethod::POST => reqwest::Method::POST,
            HttpMethod::GET => reqwest::Method::GET,
            HttpMethod::PUT => reqwest::Method::PUT,
            HttpMethod::DELETE => reqwest::Method::DELETE,
            HttpMethod::PATCH => reqwest::Method::PATCH,
        }
    }
}

#[derive(Clone,Debug)]
pub struct ApiResponse {
    pub status: u16,
    pub status_text : String,
    pub headers: Vec<(String,String)>,
    pub body : String,
    pub duration_ms: u128,
}

pub struct Request{
    pub url : String ,
    pub method : HttpMethod,
    pub headers: Vec<(String , String)>,
    pub body: Option<String>,
}

pub async fn send_request(request : Request) -> Result<ApiResponse, String> {
    
    let start = Instant::now();

    let client = reqwest::Client::new();
    
    let mut request_builder = client.request(request.method.to_request(), &request.url);

    let mut header_map = HeaderMap::new();
    for (key, value) in request.headers {
        if let (Ok(k), Ok(v)) = (HeaderName::from_str(&key), HeaderValue::from_str(&value)) {
            header_map.insert(k, v);
        }
    }
    request_builder = request_builder.headers(header_map);

    if let Some(b) = request.body {
        request_builder = request_builder.body(b);
    }

    let response = request_builder.send().await.map_err(|e| e.to_string())?;

    let duration = start.elapsed().as_millis();

    let status = response.status().as_u16();
    let status_text = response.status().canonical_reason().unwrap_or("Unknown").to_string();
    
    let mut response_headers = Vec::new();
    for (k, v) in response.headers().iter() {
        response_headers.push((k.to_string(), v.to_str().unwrap_or("").to_string()));
    }

    let body_text = response.text().await.map_err(|e| e.to_string())?;

    Ok(ApiResponse {
        status,
        status_text,
        headers: response_headers,
        body: body_text,
        duration_ms: duration,
    })
}