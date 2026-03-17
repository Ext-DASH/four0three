use crate::ParsedRequest::ParsedRequest;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderName;
use reqwest::header::HeaderValue;

pub async fn build_and_send_request_packet(req: &ParsedRequest, head: String, value: String, proxy: Option<String>, insecure: bool, follow_redirects: bool, scheme: Option<String>) -> Option<(reqwest::Response, String)> {
    let mut client = reqwest::Client::builder()
        .danger_accept_invalid_certs(insecure);
    if let Some(proxy_url) = proxy {
        client = client.proxy(reqwest::Proxy::all(&proxy_url).unwrap());
    }
    if follow_redirects {
        client = client.redirect(reqwest::redirect::Policy::limited(10));
    }
    let client = client.build().unwrap();

    let mut header_map = HeaderMap::new();
    
    for (key, value) in &req.headers {
        let header_name = match HeaderName::from_bytes(key.as_bytes()) {
            Ok(name) => name,
            Err(_) => {
                print!("Invalid Header: {}: {}", key, value);
                continue;
            }
        };
        
        let header_value = match HeaderValue::from_str(value) {
            Ok(val) => val,
            Err(_) => {
                print!("Invalid Header: {}: {}", key, value);
                continue;
            }
        };
        
        
        header_map.insert(header_name, header_value);
    }
    match HeaderName::from_bytes(head.as_bytes()) {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid header name: {}", head);
            return None;
            
        }
    };
    header_map.insert(HeaderName::from_bytes(head.as_bytes()).unwrap(), HeaderValue::from_str(&value).unwrap()); 
    // add all headers
    let host = req.headers.iter().find(|(key, _)| key == "Host")
                .map(|(_, value)| value.clone())
                .unwrap_or_default();
    
    let proto = String::new();
    let mut overriden = String::new();
    if !scheme.is_none() {
        overriden = match scheme.unwrap().as_str() {
            "http" => "http".to_string(),
            "https" => "https".to_string(),
            _ => req.proto.to_string()
        };
    }
    
    let request_builder = client
        .request(reqwest::Method::from_bytes(req.method.as_bytes()).unwrap(), req.url.clone())
        .headers(header_map);
     
    // add body if present
    let response = request_builder
        .body(req.body.clone())
        .send()
        .await
        .unwrap();
    
    //if verbose here?

    Some((response, format!("{}://{}{}", proto, host, req.url)))
    
    
}