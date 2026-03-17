use crate::{ParsedRequest::ParsedRequest, args::Args};
use colored::Colorize;

pub async fn print_init_and_status(req: &ParsedRequest, args: &Args) {
    let threads = args.threads;
    let insecure = args.insecure.to_string();
    let queue = args.queue_size;
    let proto = req.proto.clone();
    

    let proxy = !args.burp.is_none();
    let proxy = proxy.to_string();

    let mut target = String::new();

    if args.scheme_override.is_none() {
        target = format!("{}", req.url);
    } else {
        let no_proto = req.url.split("://").last().unwrap();
        target = format!("{}://{}", proto, no_proto.to_string());
    }
    
    let rate = args.rate_limit;


    println!(r#"
  __                        ___   _    _                    
 / _|  ___   _   _  _ ___  / _ \ | |_ | |__   _ ___   ___  ___
| |_  / _ \ | | | || '_  || | | || __|| '_ \ | '_  | / _ \/ _ \
|  _|| | | || | | || | |_|| ||| || |  | | | || | |_|| ___| ___/
| |  | |_| || |_| || |    | |_| || |_ | | | || |    | \__| \__
|_|   \___/  \__,_||_|     \___/  \__||_| |_||_|     \___\\___\
ver 1.2         ExtDASH https://github.com/Ext-DASH/four0three/
Based on BypassFuzzer.py but written in rust.
Payload credits:         intrudier https://github.com/intrudir/
───────────────────────────────────────────────────────────────
Target:              │ {target}
Threads:             │ {threads}
Insecure:            │ {insecure}
Max Queue Size:      │ {queue}
Rate Limit:          | {rate}
Proxy:               │ {proxy}
Response Filer:      | 200, 201, 202
───────────────────────────────────────────────────────────────
"#);
    println!("Checking status of host...");
    match reqwest::get(target).await {
        Ok(_) => println!("{}", "[+] Host is active.".green()),
        Err(_) => {
            println!("{}", "[-] Host is not active, exiting...".red());
            println!("{}", "[-] Tip: If the host is not using https, use --scheme-override. See help for more details.".yellow());
            std::process::exit(1);
        }
    };
}