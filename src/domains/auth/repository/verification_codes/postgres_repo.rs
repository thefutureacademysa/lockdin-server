use crate::domains::auth::models::OtpResponse;
use crate::domains::auth::repository::verification_codes::repo::OtpRepo;
use chrono::{DateTime, Local};
use sqlx::Error;

pub struct OtpPostgresRepo {
    pub pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl OtpRepo for OtpPostgresRepo {
    async fn store_otp(
        &self,
        // todo: update to user_id
        email: &String,
        code: &String,
        expires_at: &DateTime<Local>,
        now: &DateTime<Local>,
    ) -> sqlx::Result<OtpResponse, Error> {
        let otp = sqlx::query_as(
            "
        INSERT INTO otps (user_id, code, expires_at, created_at)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (user_id)
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
        user_id: &String,
    ) -> sqlx::Result<Option<OtpResponse>, Error> {
        let otp = sqlx::query_as(
            "
        SELECT * FROM otps WHERE code = $1 AND user_id = $2 AND expires_at > NOW()
        ",
        )
        .bind(code)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(otp)
    }

    async fn invalidate_otp(
        &self,
        code: &String,
        user_id: &String,
    ) -> sqlx::Result<u64, Error> {
        let otp = sqlx::query(
            "
        DELETE FROM otps WHERE code = $1 AND user_id = $2
        ",
        )
            .bind(code)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(otp.rows_affected())
    }
}
