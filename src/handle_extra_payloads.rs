use base64::{Engine, engine::general_purpose};
use urlencoding::encode;

use crate::{ResolvedPayloads::ResolvedPayloads, payloads};

pub fn handle_extra_payloads(
    oob_payload: &Option<String>, 
    oob_domain: &Option<String>, 
    extra_header: &Option<String>, 
    extra_ip: &Option<String>, 
    extra_url: &Option<String>, 
    url: String, 
    base64: &bool, 
    case_tamper: &bool, 
    url_encode: &bool,
    skip_ip: &bool,
    skip_url: &bool) -> ResolvedPayloads {
    let mut oob_payloads: Vec<String> = payloads::OOB_PAYLOADS.iter().map(|s| s.to_string()).collect();
    let mut oob_domains: Vec<String> = payloads::OOB_DOMAIN_PAYLOADS.iter().map(|s| s.to_string()).collect();
    let mut header_payloads: Vec<String> = payloads::HEADER_TEMPLATES.iter().map(|s| s.to_string()).collect();
    let mut ip_payloads = vec![String::new()];
    if !skip_ip {
        ip_payloads.pop();
        ip_payloads = payloads::IP_PAYLOADS.iter().map(|s| s.to_string()).collect();
    }

    let mut url_payloads = vec![String::new()];
    if !skip_url {
        url_payloads = payloads::URL_PAYLOADS.iter().map(|s| s.to_string()).collect();
    }
    let mut path_payload: Vec<String> = payloads::PATH_PAYLOAD.iter().map(|s| s.to_string()).collect();
    let mut whitespace_payloads: Vec<String> = payloads::WHITESPACE_PAYLOADS.iter().map(|s| s.to_string()).collect();
    
    path_payload.push(url);

    
    fn push_extra(extra: &Option<String>, target: &mut Vec<String>) {
        if let Some(val) = extra {
            if val.contains(",") {
                for item in val.split(",") {
                    let item = item.trim().to_string();
                    if !target.contains(&item) {
                        target.push(item);
                    }
                }
            } else {
                target.push(val.to_string());
            }
        }
    }

    push_extra(oob_payload, &mut oob_payloads);
    push_extra(oob_domain, &mut oob_domains);
    push_extra(extra_header, &mut header_payloads);
    push_extra(extra_ip, &mut ip_payloads);
    push_extra(extra_url, &mut url_payloads);

    let ip_cloned = &ip_payloads.clone();
    let url_cloned = &url_payloads.clone();
    let whitespace_cloned = &whitespace_payloads.clone();
    let path_cloned = &path_payload.clone();

    fn handle_mutations (target: &mut Vec<String>, mutation_type: String, original: &Vec<String>) {
        match mutation_type.as_str() {
            "base64" => {
                for item in original {
                    if !general_purpose::STANDARD.encode(&item).eq(&item.to_string()) {
                        target.push(general_purpose::STANDARD.encode(item)); 
                    }
                }
            },
            "url" => {
                for item in original {
                    if encode(&item).eq(&item.to_string()) {
                        target.push(encode(item.as_str()).to_string()); 
                    }
                }

            },
            "case" => {
                for item in original {
                    if !item.to_uppercase().eq(item) {
                        target.push(item.to_uppercase()); 
                    }
                    if !item.to_lowercase().eq(item) {
                        target.push(item.to_lowercase());
                    }
                }
            }
            _ => {

            }
        };
    }
    
    if *url_encode {
        handle_mutations(&mut ip_payloads, "url".to_string(), ip_cloned);
        handle_mutations(&mut url_payloads, "url".to_string(), url_cloned);
        handle_mutations(&mut whitespace_payloads, "url".to_string(), whitespace_cloned);
        handle_mutations(&mut path_payload, "url".to_string(), path_cloned);
    }

    
    if *base64 {
        handle_mutations(&mut ip_payloads, "base64".to_string(), ip_cloned);
        handle_mutations(&mut url_payloads, "base64".to_string(), url_cloned);
        handle_mutations(&mut whitespace_payloads, "base64".to_string(), whitespace_cloned);
        handle_mutations(&mut path_payload, "base64".to_string(), path_cloned);
    }
    
    if *case_tamper {
        handle_mutations(&mut ip_payloads, "case".to_string(), ip_cloned);
        handle_mutations(&mut url_payloads, "case".to_string(), url_cloned);
        handle_mutations(&mut whitespace_payloads, "case".to_string(), whitespace_cloned);
        handle_mutations(&mut path_payload, "case".to_string(), path_cloned);
    }

    ResolvedPayloads { headers: header_payloads, ip_payloads: ip_payloads, url_payloads: url_payloads, oob_payloads: oob_payloads, oob_domain_payloads: oob_domains, whitespace_payloads: whitespace_payloads, path_payload: path_payload }

}