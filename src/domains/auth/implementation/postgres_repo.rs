use crate::domains::auth::models::OtpResponse;
use crate::domains::auth::repo::OtpRepo;
use chrono::{DateTime, Local};
use sqlx::Error;

pub struct OtpPostgresRepo {
    pub pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl OtpRepo for OtpPostgresRepo {
    async fn store_otp(
        &self,
        email: &String,
        code: &String,
        expires_at: &DateTime<Local>,
        now: &DateTime<Local>,
    ) -> sqlx::Result<OtpResponse, Error> {
        let otp = sqlx::query_as(
            "
        INSERT INTO otps (email, code, expires_at, created_at)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (email)
        DO UPDATE SET code = $2, expires_at = $3, created_at = $4
        RETURNING *
        ",
        )
        .bind(email)
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
        email: &String,
    ) -> sqlx::Result<Option<OtpResponse>, Error> {
        let otp = sqlx::query_as(
            "
        SELECT * FROM otps WHERE code = $1 AND email = $2 AND expires_at > NOW()
        ",
        )
        .bind(code)
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(otp)
    }

    async fn invalidate_otp(
        &self,
        code: &String,
        email: &String,
    ) -> sqlx::Result<u64, Error> {
        let otp = sqlx::query(
            "
        DELETE FROM otps WHERE code = $1 AND email = $2
        ",
        )
            .bind(code)
            .bind(email)
            .execute(&self.pool)
            .await?;

        Ok(otp.rows_affected())
    }
}
