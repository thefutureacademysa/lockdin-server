use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum NotificationType {
    EmailVerification(EmailVerificationPayload),
    PhoneNumberVerification(PhoneNumberVerificationPayload),
    ExamCountdown,
    StudyReminder,
    WelcomeMessage,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmailVerificationPayload {
    pub from: String,
    pub to: String, // or a user struct.
    pub code: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhoneNumberVerificationPayload {
    pub phone_number: String,
    pub code: String,
}