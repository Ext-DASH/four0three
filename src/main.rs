use clap::Parser;
use clap::Command;
use std::path::PathBuf;
use std::str::Split;
use httparse;
use reqwest;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::sync::Arc;
use tokio::sync::Semaphore;
use regex::Regex;
use std::io;

mod payloads;
mod args;

use args::Args;

#[derive(Clone)]
#[derive(Debug)]
struct ParsedRequest {
    method: String,
    url: String,
    proto: String,
    headers: Vec<(String, String)>,
    body: String,
}

#[derive(Debug)]
struct ResolvedPayloads {
    headers: Vec<String>,
    ip_payloads: Vec<String>,
    url_payloads: Vec<String>,
    oob_payloads: Vec<String>,
    oob_domain_payloads: Vec<String>,
    whitespace_payloads: Vec<String>,
    path_payload: Vec<String>
}

fn pause() {
    println!("Press Enter to continue...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

//TODO: FINISH FUNCTION THAT ADDS USER SUPPLIED PAYLOADS TO PAYLOAD LISTS.
fn handle_extra_payloads(oob_payload: Option<String>, oob_domain: Option<String>, extra_header: Option<String>, extra_ip: Option<String>, extra_url: Option<String>, req_path: String) -> ResolvedPayloads {
    let mut oob_payloads: Vec<String> = payloads::OOB_PAYLOADS.iter().map(|s| s.to_string()).collect();
    let mut oob_domains: Vec<String> = payloads::OOB_DOMAIN_PAYLOADS.iter().map(|s| s.to_string()).collect();
    let mut header_payloads: Vec<String> = payloads::HEADER_TEMPLATES.iter().map(|s| s.to_string()).collect();
    let mut ip_payloads: Vec<String> = payloads::IP_PAYLOADS.iter().map(|s| s.to_string()).collect();
    let mut url_payloads: Vec<String> = payloads::URL_PAYLOADS.iter().map(|s| s.to_string()).collect();
    let mut path_payload: Vec<String> = payloads::PATH_PAYLOAD.iter().map(|s| s.to_string()).collect();
    let mut whitespace_payloads: Vec<String> = payloads::WHITESPACE_PAYLOADS.iter().map(|s| s.to_string()).collect();

    path_payload.push(req_path);

    if let Some(extra) =  oob_payload {
        oob_payloads.push(extra.to_string());
    }

    if let Some(extra) =  oob_domain {
        oob_domains.push(extra.to_string());
    }

    if let Some(extra) =  extra_header {
        //TODO: PROCESS IF LIST
        // oob_payloads.push(extra.to_string());
    }

    if let Some(extra) =  extra_ip {
        //TODO: PROCESS IF LIST
        // oob_payloads.push(extra.to_string());
    }

    if let Some(extra) =  extra_url {
        //TODO: PROCESS IF LIST
        // oob_payloads.push(extra.to_string());
    }


    ResolvedPayloads { headers: header_payloads, ip_payloads: ip_payloads, url_payloads: url_payloads, oob_payloads: oob_payloads, oob_domain_payloads: oob_domains, whitespace_payloads: whitespace_payloads, path_payload: path_payload }

}

fn get_payload_list<'a>(header: &str, payloads: &'a ResolvedPayloads) -> Option<&'a Vec<String>> {
    
    let re = Regex::new(r"\{[^}]*\}").unwrap();
    let matched = re.find(header)?;

    match matched.as_str() {
        "{IP PAYLOAD}" => Some(&payloads.ip_payloads),
        "{URL PAYLOAD}" => Some(&payloads.url_payloads),
        "{OOB PAYLOAD}" => Some(&payloads.oob_payloads),
        "{OOB DOMAIN PAYLOAD}" => Some(&payloads.oob_payloads),
        "{WHITESPACE PAYLOAD}" => Some(&payloads.whitespace_payloads),
        "{PATH PAYLOAD}" => Some(&payloads.path_payload),
        _ => None, 
    }
}

async fn build_and_send_request_packet(req: &ParsedRequest) {
    
    let mut client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .proxy(reqwest::Proxy::all("http://127.0.0.1:8080").unwrap())
        .build()
        .unwrap();
    
    let mut header_map = HeaderMap::new();
    for (key, value) in &req.headers {
        let header_name = HeaderName::from_bytes(key.as_bytes()).unwrap();
        let header_value = HeaderValue::from_str(value).unwrap();
        header_map.insert(header_name, header_value);
    }
    // add all headers
    let mut request_builder = client
        .request(reqwest::Method::from_bytes(req.method.as_bytes()).unwrap(), &req.url)
        .headers(header_map);
    
    
    // add body if present
    let response = request_builder
        .body(req.body.clone())
        .send()
        .await
        .unwrap();
    
    println!("{} {} {} {:#?}", req.method, req.url, response.status(), req);
    pause();
    
    
}

async fn mutate_request(req: ParsedRequest, rate_limit: u8, queue_size: u16, threads: u8, resolved_payloads: Arc<ResolvedPayloads>) {
    //rate_limit implementation
    let semaphore = Arc::new(Semaphore::new(rate_limit as usize));
    
    //tokio channel with queue
    //TODO: TEST QUEUE_SIZE AND DETERMINE MAX
    //TODO: IMPLEMENT MAX CHECK
    let (tx, rx) = tokio::sync::mpsc::channel::<String>(queue_size as usize);
    let arc_req = Arc::new(req);
    let rx = Arc::new(tokio::sync::Mutex::new(rx));
    let mut handles = vec![];

    for _ in 0..threads {
        let rx = rx.clone();
        let sem = semaphore.clone();
        let request = arc_req.clone();
        
        let payloads = resolved_payloads.clone();

        let handle = tokio::spawn(async move {
            let mut req = (*request).clone();
            let host = req.headers.iter().find(|(key, _)| key == "Host")
                .map(|(_, value)| value.clone())
                .unwrap_or_default();

            req.url = format!("https://{}{}", host, req.url);

            let re = Regex::new(r"\{[^}]+\}").unwrap();
            for header in payloads::HEADER_TEMPLATES {
                //parse header for {PAYLOAD TYPE}
                
                let has_type = re.is_match(header);
                if has_type {
                    //parse and match type.
                    if let Some((key, value)) = header.split_once(": ") {
                        if let Some(payload_list) = get_payload_list(header, &payloads) {
                            
                            for item in payload_list {
                                req.headers.push((key.to_string(), item.to_string()));
                                build_and_send_request_packet(&req).await;
                                req.headers.pop();
                            }
                            
                            
                        }
                    } else {
                        print!("Header payload has no matching list: {header}");
                    }
                } else {
                    //if no payload template type then:
                        //header should have value, confirm header has value
                            //if not, just log this for now, all should have a value, but user supplied might not.
                            //if it does, modify a request with it
                    if let Some((key, value)) = header.split_once(": ") {
                        
                        req.headers.push((key.to_string(), value.to_string()));
                        //TODO: build request and send
                        // println!("req slice from no payload: {:#?}", &req);
                        // pause();
                        build_and_send_request_packet(&req).await; 
                        req.headers.pop();
                        
                    } else {
                        print!("Header has no value: {header}");
                    }
                    
                }
                   
            }
        });
        handles.push(handle);
        
    }
    for handle in handles {
        handle.await.unwrap();
    } 

}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let file_path_buf = args.request;

    //check if file exists
    if (file_path_buf.exists()) {
        //get request file contents and process
        let raw = std::fs::read_to_string(file_path_buf).unwrap();
        let raw = raw.replace("\r\n", "\n");
        let mut parts = raw.splitn(2, "\n\n");

        let head = parts.next().unwrap();
        let body = parts.next().unwrap_or("");

        let mut head_split: Vec<&str> = head.split("\n").collect();
        let mut first_line_parts = head_split.remove(0).split_whitespace();

        let method = first_line_parts.next().unwrap();
        let url = first_line_parts.next().unwrap();
        let proto = first_line_parts.next().unwrap();
        let mut headers_vec: Vec<(String, String)> = Vec::new();
        
        for header in head_split {
            let mut header_split = header.split(": ");
            let h = header_split.next().unwrap();
            let v = header_split.next().unwrap();
            
            headers_vec.push((h.to_string(), v.to_string()));
        }

        let parsed = ParsedRequest{method: method.to_string(), url: url.to_string(), proto: proto.to_string(), headers: headers_vec, body: body.to_string()};

        let finalized_payloads = Arc::new(handle_extra_payloads(args.oob_payload, args.oob_domain_payload, args.extra_header_payloads, args.extra_ip_payloads, args.extra_url_payloads, url.to_string()));
        // print!("{:#?}", finalized_payloads);
        mutate_request(parsed, args.rate_limit, args.queue_size, args.threads, finalized_payloads).await;

    } else {
        println!("Error: {}", "The specified file does not exist, please ensure the file exists.");
        std::process::exit(1);
    } 
}

