use anyhow::{Result, anyhow};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use tracing::{info, warn};

/// Send email notification
pub async fn send_email(subject: &str, body: &str) -> Result<()> {
    let smtp_server = std::env::var("SMTP_SERVER").unwrap_or_else(|_| "smtp.gmail.com".to_string());
    let username = std::env::var("SMTP_USERNAME")
        .map_err(|_| anyhow!("SMTP_USERNAME environment variable not set"))?;
    let password = std::env::var("SMTP_PASSWORD")
        .map_err(|_| anyhow!("SMTP_PASSWORD environment variable not set"))?;
    let from_email = std::env::var("FROM_EMAIL")
        .map_err(|_| anyhow!("FROM_EMAIL environment variable not set"))?;
    let to_emails_str = std::env::var("TO_EMAILS")
        .map_err(|_| anyhow!("TO_EMAILS environment variable not set"))?;

    let to_emails: Vec<&str> = to_emails_str.split(',').map(|s| s.trim()).collect();

    if to_emails.is_empty() {
        return Err(anyhow!("No recipient email addresses configured"));
    }

    info!("Preparing to send email to {} recipients", to_emails.len());

    // Create email message
    let email = Message::builder()
        .from(
            from_email
                .parse()
                .map_err(|e| anyhow!("Invalid sender email format: {}", e))?,
        )
        .to(to_emails[0]
            .parse()
            .map_err(|e| anyhow!("Invalid recipient email format: {}", e))?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_string())?;

    // Create SMTP transport
    let creds = Credentials::new(username.clone(), password.clone());

    let mailer = SmtpTransport::relay(&smtp_server)?
        .credentials(creds)
        .build();

    // Send email
    match mailer.send(&email) {
        Ok(_) => {
            info!("Email sent successfully: {}", subject);
            Ok(())
        }
        Err(e) => {
            warn!("Email sending failed: {:?}", e);
            Err(anyhow!("Email sending failed: {}", e))
        }
    }
}

/// Send HTML formatted email
#[allow(dead_code)]
pub async fn send_html_email(subject: &str, html_body: &str, text_body: &str) -> Result<()> {
    let smtp_server = std::env::var("SMTP_SERVER").unwrap_or_else(|_| "smtp.gmail.com".to_string());
    let username = std::env::var("SMTP_USERNAME")
        .map_err(|_| anyhow!("SMTP_USERNAME environment variable not set"))?;
    let password = std::env::var("SMTP_PASSWORD")
        .map_err(|_| anyhow!("SMTP_PASSWORD environment variable not set"))?;
    let from_email = std::env::var("FROM_EMAIL")
        .map_err(|_| anyhow!("FROM_EMAIL environment variable not set"))?;
    let to_emails_str = std::env::var("TO_EMAILS")
        .map_err(|_| anyhow!("TO_EMAILS environment variable not set"))?;

    let to_emails: Vec<&str> = to_emails_str.split(',').map(|s| s.trim()).collect();

    if to_emails.is_empty() {
        return Err(anyhow!("No recipient email addresses configured"));
    }

    info!(
        "Preparing to send HTML email to {} recipients",
        to_emails.len()
    );

    // Create multipart email (HTML + plain text)
    let email = Message::builder()
        .from(
            from_email
                .parse()
                .map_err(|e| anyhow!("Invalid sender email format: {}", e))?,
        )
        .to(to_emails[0]
            .parse()
            .map_err(|e| anyhow!("Invalid recipient email format: {}", e))?)
        .subject(subject)
        .multipart(
            lettre::message::MultiPart::alternative()
                .singlepart(
                    lettre::message::SinglePart::builder()
                        .header(ContentType::TEXT_PLAIN)
                        .body(text_body.to_string()),
                )
                .singlepart(
                    lettre::message::SinglePart::builder()
                        .header(ContentType::TEXT_HTML)
                        .body(html_body.to_string()),
                ),
        )?;

    // Create SMTP transport
    let creds = Credentials::new(username.clone(), password.clone());

    let mailer = SmtpTransport::relay(&smtp_server)?
        .credentials(creds)
        .build();

    // Send email
    match mailer.send(&email) {
        Ok(_) => {
            info!("HTML email sent successfully: {}", subject);
            Ok(())
        }
        Err(e) => {
            warn!("HTML email sending failed: {:?}", e);
            Err(anyhow!("Email sending failed: {}", e))
        }
    }
}

/// Send bulk emails
#[allow(dead_code)]
pub async fn send_bulk_email(subject: &str, body: &str, recipients: &[String]) -> Result<()> {
    for recipient in recipients {
        let smtp_server =
            std::env::var("SMTP_SERVER").unwrap_or_else(|_| "smtp.gmail.com".to_string());
        let _smtp_port = std::env::var("SMTP_PORT")
            .unwrap_or_else(|_| "587".to_string())
            .parse::<u16>()
            .unwrap_or(587);
        let username = std::env::var("SMTP_USERNAME")
            .map_err(|_| anyhow!("SMTP_USERNAME environment variable not set"))?;
        let password = std::env::var("SMTP_PASSWORD")
            .map_err(|_| anyhow!("SMTP_PASSWORD environment variable not set"))?;
        let from_email = std::env::var("FROM_EMAIL")
            .map_err(|_| anyhow!("FROM_EMAIL environment variable not set"))?;

        let email = Message::builder()
            .from(
                from_email
                    .parse()
                    .map_err(|e| anyhow!("发件人邮箱格式错误: {}", e))?,
            )
            .to(recipient
                .parse()
                .map_err(|e| anyhow!("收件人邮箱格式错误: {}", e))?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_string())?;

        let creds = Credentials::new(username.clone(), password.clone());

        let mailer = SmtpTransport::relay(&smtp_server)?
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => info!("Email sent successfully to: {}", recipient),
            Err(e) => warn!("Email sending failed to {}: {:?}", recipient, e),
        }

        // Avoid sending too quickly
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    Ok(())
}

/// Validate email configuration
#[allow(dead_code)]
pub async fn validate_email_config() -> Result<bool> {
    let smtp_server = std::env::var("SMTP_SERVER").unwrap_or_else(|_| "smtp.gmail.com".to_string());
    let username = std::env::var("SMTP_USERNAME")
        .map_err(|_| anyhow!("SMTP_USERNAME environment variable not set"))?;
    let password = std::env::var("SMTP_PASSWORD")
        .map_err(|_| anyhow!("SMTP_PASSWORD environment variable not set"))?;

    let creds = Credentials::new(username, password);

    let mailer = SmtpTransport::relay(&smtp_server)?
        .credentials(creds)
        .build();

    // Try to connect to SMTP server
    match mailer.test_connection() {
        Ok(_) => {
            info!("Email configuration validation successful");
            Ok(true)
        }
        Err(e) => {
            warn!("Email configuration validation failed: {:?}", e);
            Ok(false)
        }
    }
}
