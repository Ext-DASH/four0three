#[derive(Clone)]
#[derive(Debug)]
pub struct ParsedRequest {
    pub method: String,
    pub url: String,
    pub proto: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}