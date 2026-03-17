#![allow(non_snake_case)]
use clap::Parser;
use std::{fs::File, path::PathBuf, sync::Arc};
use regex::Regex;
use indicatif::{ProgressBar, ProgressStyle};

mod payloads;
mod args;
mod get_payload_list;
mod ResolvedPayloads;
mod handle_extra_payloads;
mod log_output_to_file;
mod build_and_send_request_packet;
mod mutate_request;
mod ParsedRequest;
mod print_init_and_status;

use mutate_request::mutate_request;
use args::Args;
use build_and_send_request_packet::build_and_send_request_packet;
use handle_extra_payloads::handle_extra_payloads;
use crate::get_payload_list::get_payload_list;


#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    let log_path_buf = match args.log_output.clone() {
        None => PathBuf::new(),
        Some(path) => path.to_path_buf()
    };

    if !log_path_buf.as_os_str().is_empty() {
        match log_path_buf.exists() {
            true => "file exists, skipping",
            false => {
                let _ = File::create(log_path_buf);
                "file did not exist, created"
            }
        };
    }
    let mut scheme = "https".to_string();
    if args.scheme_override.is_some() {
        scheme = match args.scheme_override.as_ref().unwrap().as_str() {
            "http" => "http".to_string(),
            "https" => "https".to_string(),
            _ => {
                scheme
            }
        }
    }

    let parsed = if let Some(file_path_buf) = args.request.as_ref() {
        //parsing logic for request file
        if file_path_buf.exists() {
            let raw = std::fs::read_to_string(file_path_buf).unwrap();
        
            let raw = raw.replace("\r\n", "\n").replace("\r", "\n");
            
            let mut parts = raw.splitn(2, "\r");

            let head = parts.next().unwrap();
            let body = parts.next().unwrap_or("");
            
            
            let mut head_split: Vec<&str> = head.split("\n").collect();
            let host = head_split.iter().filter(|h| h.contains("Host")).next().unwrap().replace("Host: ", "");
            
                

            let mut first_line_parts = head_split.remove(0).split_whitespace();
           
            let method = first_line_parts.next().unwrap();
            let url = first_line_parts.next().unwrap();
            
            

            let mut headers_vec: Vec<(String, String)> = Vec::new();


            for header in head_split.iter().filter(|h| !h.is_empty()) {
                let mut header_split = header.split(": ");
                let h = header_split.next().unwrap();
                let v = header_split.next().unwrap();
                
                headers_vec.push((h.to_string(), v.to_string()));
            }

            let finalized_url = format!("{}://{}{}", scheme, host, url);
            
            
            ParsedRequest::ParsedRequest{method: method.to_string(), url: finalized_url, proto: scheme.to_string(), headers: headers_vec, body: body.to_string()}
        } else {
            println!("Error: The specified file does not exist, please ensure the file exists.");
            std::process::exit(1);
        }
        

    } else {
        //parsing logic for "custom" request
        let url_arg = args.url.as_ref().unwrap();
        let method = args.method.as_ref().unwrap();
        
        let mut headers_vec: Vec<(String, String)> = vec![];
        match args.headers.is_empty() {
            false => {
                for header_arg in &args.headers {
                    if header_arg.contains(": ") {
                        let mut headers_split = header_arg.split(": ");
                        let h = headers_split.next().unwrap().to_string();
                        let v = headers_split.next().unwrap().to_string();
                        headers_vec.push((h, v))
                    }
                }

            },
            true => { /* do nothing, no headers */ }
        };
        
        let data_arg_resolved = match args.data.as_ref() {
            Some(args_data) => args_data.clone(),
            None => String::new()
        };
        
        //return a ParsedRequest
        
        ParsedRequest::ParsedRequest{method: method.to_string(), url: url_arg.to_string(), proto: scheme.to_string(), headers: headers_vec, body: data_arg_resolved.to_string()}
    };
    
    print_init_and_status::print_init_and_status(&parsed, &args).await;
    let finalized_payloads = Arc::new(
        handle_extra_payloads(
                &args.oob_payload, 
                &args.oob_domain_payload, 
                &args.extra_header_payloads, 
                &args.extra_ip_payloads, 
                &args.extra_header_payloads, 
                parsed.url.to_string(),
                &args.base64,
                &args.case_tamper,
                &args.url_encode,
                &args.skip_ip_payloads,
                &args.skip_url_payloads)).clone();

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
            //refactor to match
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
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    mutate_request(parsed, finalized_payloads, &args, pb).await;

} 


