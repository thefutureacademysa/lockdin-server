use resend_rs::Resend;
use resend_rs::types::CreateEmailBaseOptions;
use crate::domains::notifications::models::NotificationType;

pub struct NotificationService {}

impl NotificationService {
    /// Send notification.
    pub async fn send_notification(notification_type: NotificationType) -> Result<(), String> {
        match notification_type {
            // Email verification
            NotificationType::EmailVerification(payload) => {
                let to = [payload.to.clone()];
                let subject = format!("{} is your Lockdin code.", payload.code);
                let html = format!("<strong>Here is your Lockdin verification code: {} \nUse it to login.</strong>", payload.code);

                Self::send_email(payload.from, to.to_vec(), subject.to_string(), html).await
            }
            _ => {
                Err("Notification type not supported".to_string())
            }
        }
    }

    /// Send email.
    pub async fn send_email(from: String, to: Vec<String>, subject: String, body: String) -> Result<(), String> {
        log::info!("notification_service: sending email verification code");
        let resend_api_key = std::env::var("RESEND_API_KEY").expect("RESEND_API_KEY environment variable is required");
        let resend  = Resend::new(&resend_api_key);

        let email = CreateEmailBaseOptions::new(from, to, subject).with_html(body.as_str());

        match resend.emails.send(email).await {
            Ok(result) => {
                println!("{:?}", result);
                Ok(())
            }
            Err(e) => {
                log::error!("notification_service: failed to send email \n{}", e.to_string());
                Err(e.to_string())
            }
        }

    }


}