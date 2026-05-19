use crate::domains::auth::models::OtpResponse;
use crate::domains::auth::repo::OtpRepo;
use chrono::{DateTime, Local, Utc};
use sqlx::Error;

pub struct OtpPostgresRepo {
    pub pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl OtpRepo for OtpPostgresRepo {
    async fn store_otp(
        &self,
        phone_number: &String,
        code: &String,
        expires_at: &DateTime<Local>,
        now: &DateTime<Local>,
    ) -> sqlx::Result<OtpResponse, Error> {
        let otp = sqlx::query_as(
            "
        INSERT INTO otps (phone_number, code, expires_at, created_at)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (phone_number)
        DO UPDATE SET code = $2, expires_at = $3, created_at = $4
        RETURNING *
        ",
        )
        .bind(phone_number)
        .bind(&code)
        .bind(expires_at)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(otp)
    }

    async fn get_otp(
        &self,
        code: &String,
        phone_number: &String,
    ) -> sqlx::Result<Option<OtpResponse>, Error> {
        let otp = sqlx::query_as(
            "
        SELECT * FROM otps WHERE code = $1 AND phone_number = $2 AND expires_at > NOW()
        ",
        )
        .bind(code)
        .bind(phone_number)
        .fetch_optional(&self.pool)
        .await?;

        Ok(otp)
    }

    async fn invalidate_otp(
        &self,
        code: &String,
        phone_number: &String,
    ) -> sqlx::Result<u64, Error> {
        let otp = sqlx::query(
            "
        DELETE FROM otps WHERE code = $1 AND phone_number = $2
        ",
        )
            .bind(code)
            .bind(phone_number)
            .execute(&self.pool)
            .await?;

        Ok(otp.rows_affected())
    }
}
