use crate::ResolvedPayloads::ResolvedPayloads;
use regex::Regex;

pub fn get_payload_list<'a>(header: &str, payloads: &'a ResolvedPayloads) -> Option<&'a Vec<String>> {
    
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