use actix_web::{web, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;

// Define the structure for form data
#[derive(Deserialize, Serialize)]
pub struct ContactForm {
    full_name: String,
    email: String,
    phone: Option<String>,
    subject: String,
    credit_card: Option<String>,
    message: String,
}

// Handler for the GET request to display the contact form
pub async fn contact_page() -> impl Responder {
    let html_content = include_str!("../templates/contact.html");
    let response_body = html_content
        .replace("{{message_type}}", "")
        .replace("{{status_message}}", "");
    
    HttpResponse::Ok().content_type("text/html").body(response_body)
}

// Handler for the POST request to process the form submission
pub async fn handle_contact_form(form: web::Form<ContactForm>) -> impl Responder {
    let contact_info = form.into_inner();
    
    // Log the sensitive information - intentional security vulnerability
    log_sensitive_information(&contact_info);
    
    // Display confirmation page with the user's message details
    let html_content = include_str!("../templates/contact_confirmation.html");
    
    // Replace placeholders with actual values from the form
    let response_body = html_content
        .replace("{{sender_name}}", &contact_info.full_name)
        .replace("{{sender_email}}", &contact_info.email)
        .replace("{{subject}}", &contact_info.subject)
        .replace("{{message}}", &contact_info.message.replace("\n", "<br>"));
    
    HttpResponse::Ok().content_type("text/html").body(response_body)
}

// Function to log sensitive information (intentional vulnerability)
fn log_sensitive_information(contact_info: &ContactForm) {
    // Open log file in append mode
    let mut file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt") {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to open log file: {}", e);
                return;
            }
        };
    
    // Format log message with all sensitive information in clear text
    let log_message = format!(
        "[{}] CONTACT FORM SUBMISSION\nName: {}\nEmail: {}\nPhone: {}\nSubject: {}\nCredit Card: {}\nMessage: {}\n\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        contact_info.full_name,
        contact_info.email,
        contact_info.phone.clone().unwrap_or_else(|| "Not provided".to_string()),
        contact_info.subject,
        contact_info.credit_card.clone().unwrap_or_else(|| "Not provided".to_string()),
        contact_info.message
    );
    
    // Write sensitive data to log file
    if let Err(e) = file.write_all(log_message.as_bytes()) {
        eprintln!("Failed to write to log file: {}", e);
    }
}