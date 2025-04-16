use crate::config::Config;
use std::fs::File;
use std::io::Write;

#[allow(dead_code)]
pub fn demonstrate_clear_text_logging(config: &Config) {
    let sensitive_data = "user_password=123456";
    
    // Log sensitive data to a file in clear text
    let mut file = File::create(&config.log_file).expect("Unable to create log file");
    writeln!(file, "Sensitive data: {}", sensitive_data).expect("Unable to write to log file");

    println!("Logged sensitive data: {}", sensitive_data);
}
