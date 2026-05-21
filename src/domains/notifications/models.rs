use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum NotificationType {
    OtpVerification(OtpVerificationPayload),
    ExamCountdown,
    StudyReminder,
    WelcomeMessage,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OtpVerificationPayload {
    pub email: String, // or a user struct.
    pub otp: String,
}