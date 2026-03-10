use clap::Parser;
use reqwest;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::alloc::System;
use std::sync::Arc;
use tokio::sync::Semaphore;
use regex::Regex;
use std::io;
use indicatif::{ProgressBar, ProgressStyle};
use colored::Colorize;

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
fn handle_extra_payloads(oob_payload: &Option<String>, oob_domain: &Option<String>, extra_header: &Option<String>, extra_ip: &Option<String>, extra_url: &Option<String>, url: String) -> ResolvedPayloads {
    let mut oob_payloads: Vec<String> = payloads::OOB_PAYLOADS.iter().map(|s| s.to_string()).collect();
    let mut oob_domains: Vec<String> = payloads::OOB_DOMAIN_PAYLOADS.iter().map(|s| s.to_string()).collect();
    let mut header_payloads: Vec<String> = payloads::HEADER_TEMPLATES.iter().map(|s| s.to_string()).collect();
    let mut ip_payloads: Vec<String> = payloads::IP_PAYLOADS.iter().map(|s| s.to_string()).collect();
    let mut url_payloads: Vec<String> = payloads::URL_PAYLOADS.iter().map(|s| s.to_string()).collect();
    let mut path_payload: Vec<String> = payloads::PATH_PAYLOAD.iter().map(|s| s.to_string()).collect();
    let mut whitespace_payloads: Vec<String> = payloads::WHITESPACE_PAYLOADS.iter().map(|s| s.to_string()).collect();
    
    path_payload.push(url);

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

async fn build_and_send_request_packet(req: &ParsedRequest, head: String, value: String, proxy: Option<String>, insecure: bool) -> Option<(reqwest::Response, String)> {
    
    let mut client = reqwest::Client::builder()
        .danger_accept_invalid_certs(insecure);
    if let Some(proxy_url) = proxy {
        client = client.proxy(reqwest::Proxy::all(&proxy_url).unwrap());
    } 
    let client = client.build().unwrap();

    let mut header_map = HeaderMap::new();
    
    for (key, value) in &req.headers {
        let header_name = match HeaderName::from_bytes(key.as_bytes()) {
            Ok(name) => name,
            Err(e) => {
                print!("Invalid Header: {}: {}", key, value);
                continue;
            }
        };
        
        let header_value = match HeaderValue::from_str(value) {
            Ok(val) => val,
            Err(e) => {
                print!("Invalid Header: {}: {}", key, value);
                continue;
            }
        };
        
        
        header_map.insert(header_name, header_value);
    }
    let header_name = match HeaderName::from_bytes(head.as_bytes()) {
        Ok(val) => val,
        Err(e) => {
            println!("Invalid header name: {}", head);
            return None;
            
        }
    };
    header_map.insert(HeaderName::from_bytes(head.as_bytes()).unwrap(), HeaderValue::from_str(&value).unwrap()); 
    // add all headers
    let host = req.headers.iter().find(|(key, _)| key == "Host")
                .map(|(_, value)| value.clone())
                .unwrap_or_default();
            
            //TODO: PROTO CHANGE
    let mut proto = "https";
    if insecure {
        proto  = "http";
    }
    let url = format!("{}://{}{}", proto, host, req.url);
    let mut request_builder = client
        .request(reqwest::Method::from_bytes(req.method.as_bytes()).unwrap(), &url)
        .headers(header_map);
    
    
    // add body if present
    let response = request_builder
        .body(req.body.clone())
        .send()
        .await
        .unwrap();
    
    Some((response, format!("{}://{}{}", proto, host, req.url)))
    
}

async fn mutate_request(req: ParsedRequest, resolved_payloads: Arc<ResolvedPayloads>, args: &Args, pb: Arc<ProgressBar>) {
    //rate_limit implementation
    let semaphore = Arc::new(Semaphore::new(args.rate_limit as usize));
    
    //tokio channel with queue
    //TODO: TEST QUEUE_SIZE AND DETERMINE MAX
    //TODO: IMPLEMENT MAX CHECK
    let (tx, rx) = tokio::sync::mpsc::channel::<String>(args.queue_size as usize);
    let arc_req = Arc::new(req);
    let rx = Arc::new(tokio::sync::Mutex::new(rx));
    let mut handles = vec![];
    let proxy = args.burp.clone();
    let insecure = args.insecure;
    let mut errors = 0;

    for _ in 0..args.threads {
        let rx = rx.clone();
        let sem = semaphore.clone();
        let mut request = arc_req.clone();
        
        let pb = pb.clone();
        let proxy = proxy.clone();
        let insecure = insecure.clone();
        
        let handle = tokio::spawn(async move {
            let mut join_set = tokio::task::JoinSet::new();
            loop {
                let item = rx.lock().await.recv().await;
                match item {
                    Some(payload) => {

                        let sem = sem.clone();
                        let request = request.clone();
                        let pb = pb.clone();
                        let proxy = proxy.clone();

                        join_set.spawn(async move {

                            let _permit = sem.acquire().await.unwrap();
                            let mut payload_split = payload.split("|||");
                            let head = payload_split.next().unwrap().to_string();
                            let value = payload_split.next().unwrap().to_string();
                            if let Some(response) = build_and_send_request_packet(&request, head.clone(), value.clone(), proxy.clone(), insecure).await {
                                pb.inc(1);
                                let res_status = response.0.status().to_string();
                                //res_status.match
                                if res_status.eq("200 OK") {
                                    pb.println(format!("{} {} ({}: {})", "[200]".green(), response.1.cyan(), head.yellow(), value.yellow()));
                                } 
                            }
                        });
                    }
                    None => break,
                }
            }
            while let Some(result) = join_set.join_next().await {
                if let Err(e) = result {
                    eprintln!("Error: {}", e);
                }
            }
        });
        handles.push(handle);
        
    }

    let re = Regex::new(r"\{[^}]+\}").unwrap();
    for header in payloads::HEADER_TEMPLATES {
        //parse header for {PAYLOAD TYPE}
        
        let has_type = re.is_match(header);
        if has_type {
            //parse and match type.
            if let Some((key, value)) = header.split_once(": ") {
                if let Some(payload_list) = get_payload_list(header, &resolved_payloads) {
                    
                    for item in payload_list {
                        tx.send(format!("{}|||{}", key, item)).await.unwrap();
                        
                    }
                } else {
                    tx.send(header.to_string()).await.unwrap();
                }
            } else {
                print!("Header payload has no matching list: {header}");
            }
        } else {
            if let Some((key, value)) = header.split_once(": ") {
                tx.send(format!("{}|||{}", key, value)).await.unwrap();
            } else {
                print!("Header has no value: {header}");
            }
        }
    }
    drop(tx);
    for handle in handles {
        match handle.await {
            Ok(_) => {},
            Err(e) => println!("{} {}", "Tash error:".red(), e)
        }
    } 

}

//TODO: needs alot
async fn print_init_and_status(req: &ParsedRequest, args: &Args) {
    //initialization:
    let threads = args.threads;
    let insecure = args.insecure.to_string();
    let queue = args.queue_size;
    let mut proto = "https";
    if args.insecure {
        proto = "http";
    }
    let mut proxy = !args.burp.is_none();
    let mut proxy = proxy.to_string();
    let host = req.headers.iter().find(|(key, _)| key == "Host")
                .map(|(_, value)| value.clone())
                .unwrap_or_default();
            
            //TODO: PROTO CHANGE
    let target = format!("{}://{}{}", proto, host, req.url);
    let rate = args.rate_limit;
    // let filter = &args.status_codes;

    println!(r#"
  __                        ___   _    _                    
 / _|  ___   _   _  _ ___  / _ \ | |_ | |__   _ ___   ___  ___
| |_  / _ \ | | | || '_  || | | || __|| '_ \ | '_  | / _ \/ _ \
|  _|| | | || | | || | |_|| ||| || |  | | | || | |_|| ___| ___/
| |  | |_| || |_| || |    | |_| || |_ | | | || |    | \__| \__
|_|   \___/  \__,_||_|     \___/  \__||_| |_||_|     \___\\___\
ver 1.0        ExtDASH https://github.com/Ext-DASH/four0three/
Based on BypassFuzzer.py but written in rust.
Payload credits:        intrudier https://github.com/intrudir/
──────────────────────────────────────────────────────────────
Target:              │ {target}
Threads:             │ {threads}
Insecure:            │ {insecure}
Max Queue Size:      │ {queue}
Rate Limit:          | {rate}
Proxy:               │ {proxy}
Response Filer:      | 200 OK
──────────────────────────────────────────────────────────────
"#);
    println!("Checking status of host...");
    let res = match reqwest::get(target).await {
        Ok(r) => println!("{}", "[+] Host is active.".green()),
        Err(e) => {
            println!("{}", "[-] Host is not active, exiting...".red());
            std::process::exit(1);
        }
    };
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let file_path_buf = &args.request;

    //check if file exists
    if file_path_buf.exists() {
        //get request file contents and process
        let raw = std::fs::read_to_string(file_path_buf).unwrap();
        
        let raw = raw.replace("\r\n", "\n").replace("\r", "\n");
        
        let mut parts = raw.splitn(2, "\r");

        let head = parts.next().unwrap();
        let body = parts.next().unwrap_or("");
        
        
        let mut head_split: Vec<&str> = head.split("\n").collect();
        let mut first_line_parts = head_split.remove(0).split_whitespace();

        let method = first_line_parts.next().unwrap();
        let url = first_line_parts.next().unwrap();
        
        let proto = first_line_parts.next().unwrap();
        let mut headers_vec: Vec<(String, String)> = Vec::new();


        for header in head_split.iter().filter(|h| !h.is_empty()) {
            let mut header_split = header.split(": ");
            let h = header_split.next().unwrap();
            let v = header_split.next().unwrap();
            
            headers_vec.push((h.to_string(), v.to_string()));
        }

        let parsed = ParsedRequest{method: method.to_string(), url: url.to_string(), proto: proto.to_string(), headers: headers_vec, body: body.to_string()};
        //print status and initialization and test connection
        print_init_and_status(&parsed, &args).await;
        let finalized_payloads = Arc::new(handle_extra_payloads(&args.oob_payload, &args.oob_domain_payload, &args.extra_header_payloads, &args.extra_ip_payloads, &args.extra_header_payloads, url.to_string())).clone();
        
        //calc number of requests
        //TODO: tamper counts
        let re = Regex::new(r"\{[^}]+\}").unwrap();
        let mut total = 0;
        let mut ip_template_count = 0;
        let mut oob_domain_template_count = 0;
        let mut oob_template_count = 0;
        let mut url_template_count = 0;
        let mut whitespace_template_count = 0;
        let mut path_template_count = 0;
        let mut no_template_count = 0;
        for h in &finalized_payloads.headers {
            if let Some(matched) = re.find(&h) {
                let template = matched.as_str();
                if template.contains("URL") {
                    url_template_count += 1;
                    
                } else if template.contains("IP") {
                    ip_template_count += 1;
                    
                } else if template.contains("OOB PAYLOAD") {
                    oob_template_count += 1;
                    
                } else if template.contains("OOB DOMAIN PAYLOAD") {
                    oob_domain_template_count += 1;
                    
                } else if template.contains("PATH") {
                    path_template_count += 1;
                    
                } else if template.contains("WHITESPACE") {
                    whitespace_template_count += 1;
                    
                }
            } else {
                no_template_count += 1;
            }
        }
        url_template_count = url_template_count * &finalized_payloads.url_payloads.len();
        ip_template_count = ip_template_count * &finalized_payloads.ip_payloads.len();
        oob_template_count = oob_template_count * &finalized_payloads.oob_payloads.len();
        oob_domain_template_count = oob_domain_template_count * &finalized_payloads.oob_domain_payloads.len();
        path_template_count = path_template_count * &finalized_payloads.path_payload.len();
        whitespace_template_count = whitespace_template_count * &finalized_payloads.whitespace_payloads.len();

           
        total = url_template_count + ip_template_count + oob_domain_template_count + oob_template_count + path_template_count + whitespace_template_count + no_template_count;
        
        let pb = Arc::new(ProgressBar::new(total as u64));
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"));

        mutate_request(parsed, finalized_payloads, &args, pb).await;

    } else {
        println!("Error: The specified file does not exist, please ensure the file exists.");
        std::process::exit(1);
    } 
}

