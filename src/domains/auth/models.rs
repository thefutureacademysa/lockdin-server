use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignupRequest {
    pub full_name: String,
    pub school_name: String,
    pub grade: String,
    pub phone_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub phone_number: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyOtpRequest {
    pub otp: String
}