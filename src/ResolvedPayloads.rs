#[derive(Debug)]
pub struct ResolvedPayloads {
    pub headers: Vec<String>,
    pub ip_payloads: Vec<String>,
    pub url_payloads: Vec<String>,
    pub oob_payloads: Vec<String>,
    pub oob_domain_payloads: Vec<String>,
    pub whitespace_payloads: Vec<String>,
    pub path_payload: Vec<String>
}