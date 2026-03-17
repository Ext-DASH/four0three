use std::path::PathBuf;
use std::io::Write;

pub fn log_output_to_file(log_output: PathBuf, res_status: String, res_block: String, head: String, value: String) {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&log_output)
        .unwrap();

    writeln!(file, "[{}] {} | Header: {} | Payload: {}", res_status, res_block, head, value).unwrap();
}