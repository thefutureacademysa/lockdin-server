use crate::domains::notifications::models::NotificationType;

pub struct NotificationService {}

impl NotificationService {
    pub async fn send_notification(notification_type: NotificationType) -> Result<(), String> {
        match notification_type {
            NotificationType::OtpVerification(payload) => {
                
            }
            NotificationType::ExamCountdown => {}
            NotificationType::StudyReminder => {}
            NotificationType::WelcomeMessage => {}
        }
        Ok(())
    }
}