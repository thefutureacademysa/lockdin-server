use crate::domains::auth::models::OtpResponse;
use chrono::{DateTime, Local};

#[async_trait::async_trait]
pub trait OtpRepo {
    async fn store_otp(
        &self,
        email: &String,
        code: &String,
        expires_at: &DateTime<Local>,
        now: &DateTime<Local>,
    ) -> sqlx::Result<OtpResponse, sqlx::Error>;

    async fn get_otp(
        &self,
        code: &String,
        email: &String,
    ) -> sqlx::Result<Option<OtpResponse>, sqlx::Error>;

    async fn invalidate_otp(
        &self,
        code: &String,
        email: &String,
    ) -> sqlx::Result<u64, sqlx::Error>;
}
