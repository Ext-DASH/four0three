use clap::Parser;
use std::path::PathBuf;
#[derive(Parser)]
#[derive(Clone)]
pub struct Args {
    
    #[arg(short, long, required_unless_present = "url", help = "Path to request file.")]
    pub request: Option<PathBuf>,

    #[arg(long, help = "URL to fuzz.")]
    pub scheme_override: Option<String>,

    #[arg(short, long, required_unless_present = "request", help = "URL scheme to use, 'http', 'https'.")]
    pub url: Option<String>,
    #[arg(short = 'X', long, required_unless_present = "request", help = "Method to use with the request URL.")]
    pub method: Option<String>,
    #[arg(short, long, help = "Post body data for the request.")]
    pub data: Option<String>,
    #[arg(short, long, help = "Extra Cookies to add to the request.")]
    pub cookies: Vec<String>,
    #[arg(short = 'H', long, help = "Extra header and value to add to the request.")]
    pub headers: Vec<String>,

    #[arg(help_heading = "Proxy Settings")]
    #[arg(long, help = "Burp proxy address. (Format: http://host:port)")]
    pub burp: Option<String>,
    
    #[arg(help_heading = "Performance Settings")]
    #[arg(short, long, default_value_t = 50, help = "Number of threads to use. This is the amount of workers that are able to modify and send requests at a time.")]
    pub threads: u8,

    #[arg(help_heading = "Performance Settings")]
    #[arg(long, default_value_t = 25, help = "Limit the max number of requests waiting to be processed by a thread. Higher numbers increase memory consumption for better performance. [ Low: 5, High: 50 ]")]
    pub queue_size: u16,

    #[arg(help_heading = "Performance Settings")]
    #[arg(short = 'R', long, default_value_t = 20, help = "Max number of requests that can be sent at once.")]
    pub rate_limit: u8,
    

    #[arg(help_heading = "Client Settings")]
    #[arg(short = 'k', long, help = "Skip SSL/TLS certificate validation.")]
    pub insecure: bool,

    #[arg(help_heading = "Client Settings")]
    #[arg(short = 'A', long, help = "Replace the Agent header value with a random agent.")]
    pub random_agent: bool,

    #[arg(help_heading = "Response Settings")]
    #[arg(short, long, help = "Follow redirects.")]
    pub follow_redirects: bool,

    //skip ip, skip url, skip regex
    #[arg(help_heading = "Payload Settings")]
    #[arg(long, help = "Skip all IP payloads.")]
    pub skip_ip_payloads: bool,
    #[arg(help_heading = "Payload Settings")]
    #[arg(long, help = "Skip all URL payloads.")]
    pub skip_url_payloads: bool,
    // #[arg(help_heading = "Payload Settings")]
    // #[arg(long, help = "Skip that match a regex pattern.")]
    // pub skip_regex: Option<String>,
    

    #[arg(help_heading = "Payload Settings")]
    #[arg(short = 'D', long, help = "Out-of-band domain. Adds more tests. Can be the same value as -P. (Format: -D '192.168.0.1' or -D 'some.domain.com')")]
    pub oob_domain_payload: Option<String>,

    #[arg(help_heading = "Payload Settings")]
    #[arg(short = 'P', long, help = "Out-of-band payload. Adds more tests. Can be the same value as -D. (Format: -P '192.168.0.1' or -P 'some.domain.com')")]
    pub oob_payload: Option<String>,

    #[arg(help_heading = "Payload Settings")]
    #[arg(short, long, help = "Try header payload values as is and with base64 encoding header values. Because try harder.")]
    pub base64: bool,

    #[arg(help_heading = "Payload Settings")]
    #[arg(long, help = "Try header payload values as is and as URL encoded payloads. Because try harder. This will URL encode payloads that are already URL encoded.")]
    pub url_encode: bool,

    #[arg(help_heading = "Payload Settings")]
    #[arg(long, help = "Try header payload values as is and with upper case. Because try harder.")]
    pub case_tamper: bool,

    #[arg(help_heading = "Payload Settings")]
    #[arg(long, help = "Extra header payloads to be used in the payload template. Can be a string list seperated by commas. For a list of header payloads, use --list-header-payloads. (Format: 'Header: Value' or 'Header: Value, Header: Value,...')")]
    pub extra_header_payloads: Option<String>,

    #[arg(help_heading = "Payload Settings")]
    #[arg(long, help = "Extra IP payloads to be used in the ip payload template. Can be a string list seperated by commas. For a list of IP payloads, use --list-ip-payloads. (Format: '192.168.0.1' or '192.168.0.1, 127.0.0.1,...')")]
    pub extra_ip_payloads: Option<String>,

    #[arg(help_heading = "Payload Settings")]
    #[arg(long, help = "Extra URL payloads to be used in the URL payload template. Can be a string list seperated by commas. For a list of URL payloads, use --list-url-payloads. (Format ';/../' or ';/../, ;/../.;/../,...')")]
    pub extra_url_payloads: Option<String>,

    #[arg(help_heading = "General Settings")]
    #[arg(short, long, help = "Enable verbose output.")]
    pub verbose: bool,

    #[arg(help_heading = "General Settings")]
    #[arg(long, help = "Log any output to file.")]
    pub log_output: Option<PathBuf>,
}
