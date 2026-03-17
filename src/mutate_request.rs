use crate::{ParsedRequest::ParsedRequest, args::Args, build_and_send_request_packet, ResolvedPayloads::ResolvedPayloads, log_output_to_file::log_output_to_file, payloads, get_payload_list};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use indicatif::ProgressBar;
use regex::Regex;
use tokio::sync::Semaphore;
use std::path::PathBuf;
use colored::Colorize;

pub fn build_status_message(
    verbose: bool,
    one: &AtomicUsize, two_oh_two: &AtomicUsize, other_two: &AtomicUsize,
    three_oh_one: &AtomicUsize, other_three: &AtomicUsize,
    four_oh_one: &AtomicUsize, four_oh_three: &AtomicUsize, four_oh_four: &AtomicUsize, other_four: &AtomicUsize,
    five_two_four: &AtomicUsize, other_five: &AtomicUsize,
) -> String {
    if verbose {
        format!(
            "1xx: {} | 200-202: {} | 203-226: {} | 301-302: {} | 300,303-308: {} | 400,402,405-499: {} | 401: {} | 403: {} | 404: {} | 500,501,505-511: {} | 502-504: {}",
            one.load(Ordering::Relaxed),
            two_oh_two.load(Ordering::Relaxed).to_string().bright_green(),
            other_two.load(Ordering::Relaxed).to_string().green(),
            three_oh_one.load(Ordering::Relaxed).to_string().bright_yellow(),
            other_three.load(Ordering::Relaxed).to_string().yellow(),
            other_four.load(Ordering::Relaxed).to_string().red(),
            four_oh_one.load(Ordering::Relaxed).to_string().red(),
            four_oh_three.load(Ordering::Relaxed).to_string().red(),
            four_oh_four.load(Ordering::Relaxed).to_string().red(),
            other_five.load(Ordering::Relaxed).to_string().bright_red(),
            five_two_four.load(Ordering::Relaxed).to_string().blink().bright_red()
        )
    } else {
        let fivexx = other_five.load(Ordering::Relaxed) + five_two_four.load(Ordering::Relaxed);
        let other = other_two.load(Ordering::Relaxed) + one.load(Ordering::Relaxed) 
                  + other_three.load(Ordering::Relaxed) + other_four.load(Ordering::Relaxed);
        format!(
            "200-202: {} | 301-302: {} | 403: {} | 404: {} | 5xx: {} | Other: {}",
            two_oh_two.load(Ordering::Relaxed).to_string().bright_green(),
            three_oh_one.load(Ordering::Relaxed).to_string().bright_yellow(),
            four_oh_three.load(Ordering::Relaxed).to_string().red(),
            four_oh_four.load(Ordering::Relaxed).to_string().red(),
            fivexx.to_string().bright_red(),
            other.to_string()
        )
    }
}

pub async fn mutate_request(req: ParsedRequest, resolved_payloads: Arc<ResolvedPayloads>, args: &Args, pb: Arc<ProgressBar>) {
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
    let scheme = args.scheme_override.clone();
    let follow_redirects = args.follow_redirects;
    let log_path = match args.log_output.clone() {
        None => PathBuf::new(),
        Some(path) => path
    };
    
    let one = Arc::new(AtomicUsize::new(0));
    let two_oh_two = Arc::new(AtomicUsize::new(0));
    let other_two = Arc::new(AtomicUsize::new(0));
    let three_oh_one = Arc::new(AtomicUsize::new(0));
    let other_three = Arc::new(AtomicUsize::new(0));
    let four_oh_three = Arc::new(AtomicUsize::new(0));
    let four_oh_one = Arc::new(AtomicUsize::new(0));
    let other_four = Arc::new(AtomicUsize::new(0));
    let five_two_four = Arc::new(AtomicUsize::new(0));
    let other_five = Arc::new(AtomicUsize::new(0));
    let four_oh_four = Arc::new(AtomicUsize::new(0));

    for _ in 0..args.threads {
        let rx = rx.clone();
        let sem = semaphore.clone();
        let request = arc_req.clone();
        
        let pb = pb.clone();
        let proxy = proxy.clone();
        let insecure = insecure.clone();
        let scheme = scheme.clone();
        let follow_redirects = follow_redirects.clone();
        
        let log_path = log_path.clone();
        let verbose = args.verbose.clone();

        let one = one.clone();
        let two_oh_two = two_oh_two.clone();
        let other_two = other_two.clone();
        let three_oh_one = three_oh_one.clone();
        let other_three = other_three.clone();
        let four_oh_three = four_oh_three.clone();
        let four_oh_one = four_oh_one.clone();
        let other_four = other_four.clone();
        let five_two_four = five_two_four.clone();
        let other_five = other_five.clone();
        let four_oh_four = four_oh_four.clone();

        let handle = tokio::spawn(async move {
            
            let mut join_set = tokio::task::JoinSet::new();
            
            loop {
                let item = rx.lock().await.recv().await;
                match item {
                    Some(payload) => {
                        let one = one.clone();
                        let two_oh_two = two_oh_two.clone();
                        let other_two = other_two.clone();
                        let three_oh_one = three_oh_one.clone();
                        let other_three = other_three.clone();
                        let four_oh_three = four_oh_three.clone();
                        let four_oh_one = four_oh_one.clone();
                        let other_four = other_four.clone();
                        let five_two_four = five_two_four.clone();
                        let other_five = other_five.clone();
                        let four_oh_four = four_oh_four.clone();
                        let sem = sem.clone();
                        let request = request.clone();
                        let pb = pb.clone();
                        let proxy = proxy.clone();
                        let log_path= log_path.clone();
                        let scheme = scheme.clone();

                        join_set.spawn(async move {
                            
                            let _permit = sem.acquire().await.unwrap();
                            let mut payload_split = payload.split("|||");
                            let head = payload_split.next().unwrap().to_string();
                            let value = payload_split.next().unwrap().to_string();
                            
                            if let Some(response) = build_and_send_request_packet(&request, head.clone(), value.clone(), proxy.clone(), insecure, follow_redirects, scheme).await {
                                
                                pb.inc(1);
                                let res_status = response.0.status().to_string();
                                
                                
                                match response.0.status().as_u16() {
                                    100..=103 => {
                                        one.fetch_add(1, Ordering::Relaxed);
                                        
                                        if verbose { 
                                            pb.println(format!("[{}] {} ({}: {})", res_status.cyan(), response.1.cyan(), head.yellow(), value.yellow()));
                                        }
                                        
                                    },
                                    200..=202 => {
                                        pb.println(format!("[{}] {} ({}: {})", res_status.bold().blink().bright_green(), response.1.bold().blink().bright_green(), head.bold().blink().bright_green(), value.bold().blink().bright_green()));
                                        two_oh_two.fetch_add(1, Ordering::Relaxed);
                                        
                                    }
                                    203..=226 => {
                                        other_two.fetch_add(1, Ordering::Relaxed);
                                        if verbose { 
                                            pb.println(format!("[{}] {} ({}: {})", res_status.green(), response.1.cyan(), head.green(), value.green()));
                                        }
                                    },
                                    300 => {
                                        other_three.fetch_add(1, Ordering::Relaxed);
                                        if verbose { 
                                            pb.println(format!("[{}] {} ({}: {})", res_status, response.1, head, value));
                                        }
                                    }
                                    301..=302 => {
                                        three_oh_one.fetch_add(1, Ordering::Relaxed);
                                        if verbose { 
                                            pb.println(format!("[{}] {} ({}: {})", res_status.bright_yellow(), response.1.cyan(), head.bright_yellow(), value.bright_yellow()));
                                        }
                                    },
                                    303..=308 => {
                                        other_three.fetch_add(1, Ordering::Relaxed);
                                        if verbose { 
                                            pb.println(format!("[{}] {} ({}: {})", res_status.yellow(), response.1.cyan(), head.yellow(), value.yellow()));
                                        }
                                        
                                    },
                                    400 | 402 | 405..=499 => {
                                        other_four.fetch_add(1, Ordering::Relaxed);
                                        if verbose { 
                                            pb.println(format!("[{}] {} ({}: {})", res_status.red(), response.1.red(), head.red(), value.red()));
                                        }
                                        
                                    },
                                    401 => {
                                        four_oh_one.fetch_add(1, Ordering::Relaxed);
                                        if verbose { 
                                            pb.println(format!("[{}] {} ({}: {})", res_status.red(), response.1.red(), head.red(), value.red()));
                                        }
                                        
                                    },
                                    403 => {
                                        four_oh_three.fetch_add(1, Ordering::Relaxed);
                                        if verbose { 
                                            pb.println(format!("[{}] {} ({}: {})", res_status.red(), response.1.red(), head.red(), value.red()));
                                        }
                                        
                                    },
                                    404 => {
                                        four_oh_four.fetch_add(1, Ordering::Relaxed);
                                        if verbose { 
                                            pb.println(format!("[{}] {} ({}: {})", res_status.red(), response.1.red(), head.red(), value.red()));
                                        }
                                    },
                                    
                                    500..=501 => {
                                        other_five.fetch_add(1, Ordering::Relaxed);
                                        if verbose { 
                                            pb.println(format!("[{}] {} ({}: {})", res_status.bright_red(), response.1.bright_red(), head.bright_red(), value.bright_red()));
                                        }
                                        
                                    },
                                    502..=504 => {
                                        five_two_four.fetch_add(1, Ordering::Relaxed);
                                        if verbose { 
                                            pb.println(format!("[{}] {} ({}: {})", res_status.bold().blink().bright_red(), response.1.bold().blink().bright_red(), head.bold().blink().bright_red(), value.bold().blink().bright_red()));
                                        }
                                        
                                    },
                                    505..=511 => {
                                        other_five.fetch_add(1, Ordering::Relaxed);
                                        if verbose { 
                                            pb.println(format!("[{}] {} ({}: {})", res_status, response.1, head, value));
                                        }
                                        
                                    },
                                    _ => {
                                        if verbose { 
                                            pb.println(format!("[{}] {} ({}: {})", res_status, response.1, head, value))
                                        }
                                    }
                                };
                                pb.set_message(build_status_message(
                                    verbose, &one, &two_oh_two, &other_two,
                                    &three_oh_one, &other_three,
                                    &four_oh_one, &four_oh_three, &four_oh_four, &other_four,
                                    &five_two_four, &other_five,
                                ));

                                if !log_path.as_os_str().is_empty() {
                                    log_output_to_file(log_path, res_status, response.1, head, value);
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
            if let Some((key, _value)) = header.split_once(": ") {
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
    pb.finish();

}