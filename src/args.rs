use clap::Parser;
use std::path::PathBuf;
#[derive(Parser)]
#[derive(Clone)]
pub struct Args {
    
    #[arg(short, long, help = "Path to request file.")]
    pub request: PathBuf,

    #[arg(help_heading = "Proxy Settings")]
    #[arg(long, help = "Burp proxy address. (Format: http://host:port)")]
    pub burp: Option<String>,
    
    #[arg(help_heading = "Performance Settings")]
    #[arg(short, long, default_value_t = 50, help = "Number of threads to use. This is the amount of workers that are able to modify and send requests at a time. (Default: 50)")]
    pub threads: u8,

    #[arg(help_heading = "Performance Settings")]
    #[arg(long, default_value_t = 100, help = "Limit the max number of requests waiting to be processed by a thread. Higher numbers increase memory consumption for better performance. (Default: 100, Low: 20, High: 500)")]
    pub queue_size: u16, //need to check limit, must be lower than something but not sure what. needs testing.

    #[arg(help_heading = "Performance Settings")]
    #[arg(short = 'R', long, default_value_t = 20, help = "Max number of requests that can be sent at once. (Default: 20)")]
    pub rate_limit: u8,

    // #[arg(help_heading = "Filter Settings")]
    // #[arg(short, long, default_value = "200", help = "List of status codes that indicate success. Format: '200'. For multiple: '200, 202,...'. (Default: 200)")]
    // pub status_codes: String,

    #[arg(help_heading = "Client Settings")]
    #[arg(short = 'k', long, help = "Skip SSL/TLS certificate validation.")]
    pub insecure: bool,

    // #[arg(help_heading = "Response Settings")]
    // #[arg(short, long, help = "Follow redirects.")]
    // follow_redirects: bool,

    #[arg(help_heading = "Payload Settings")]
    #[arg(short = 'D', long, help = "Out-of-band domain. Adds more tests. Can be the same value as -P. (Format: -D '192.168.0.1' or -D 'some.domain.com')")]
    pub oob_domain_payload: Option<String>,

    #[arg(help_heading = "Payload Settings")]
    #[arg(short = 'P', long, help = "Out-of-band payload. Adds more tests. Can be the same value as -D. (Format: -P '192.168.0.1' or -P 'some.domain.com')")]
    pub oob_payload: Option<String>,

    // #[arg(help_heading = "Payload Settings")]
    // #[arg(short, long, help = "Try header payload values as is and with base64 encoding header values. Because try harder.")]
    // pub base64: bool,

    #[arg(help_heading = "Payload Settings")]
    #[arg(short, long, help = "Try header payload values as is and with upper case. Because try harder.")]
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

    // #[arg(help_heading = "Payload Settings")]
    // #[arg(long, help = "List all header payloads used.")]
    // pub list_header_payloads: bool,

    // #[arg(help_heading = "Payload Settings")]
    // #[arg(long, help = "List all IP payloads used.")]
    // pub list_ip_payloads: bool,
    
    // #[arg(help_heading = "Payload Settings")]
    // #[arg(long, help = "List all URL payloads used.")]
    // pub list_url_payloads: bool,

    // #[arg(help_heading = "Payload Settings")]
    // #[arg(long, help = "List all URL payloads used. Spaces are represented with _'s.")]
    // pub list_whitespace_payloads: bool,

    // #[arg(help_heading = "Payload Settings")]
    // #[arg(long, help = "Append whitespace payloads to the end and begining of all header payloads. This adds a significant amount of tests. For a list of whitespace payloads, use --list-whitespace-payloads.")]
    // pub whitespace: bool,

    #[arg(help_heading = "General Settings")]
    #[arg(short, long, help = "Enable verbose output.")]
    pub verbose: bool,

    #[arg(help_heading = "General Settings")]
    #[arg(long, long, help = "Log output to file.")]
    pub log_output: Option<PathBuf>,
}