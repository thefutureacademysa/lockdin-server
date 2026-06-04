use crate::domains::auth::models::RefreshToken;
use crate::domains::auth::repository::refresh_tokens::repo::RefreshTokenRepo;
use sqlx::Error;

pub struct RefreshTokenPostgresRepo {
    pub pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl RefreshTokenRepo for RefreshTokenPostgresRepo {
    async fn store_refresh_token(&self, rt: &RefreshToken) -> sqlx::Result<RefreshToken, Error> {
        let token = sqlx::query_as(
            "
                INSERT INTO refresh_tokens (id, user_id, token, expires_at, created_at, revoked) 
                VALUES ($1, $2, $3, $4, $5, $6) RETURNING *
                ",
        )
        .bind(&rt.id)
        .bind(&rt.user_id)
        .bind(&rt.token)
        .bind(&rt.expires_at)
        .bind(&rt.created_at)
        .bind(&rt.revoked)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(token)
    }

    async fn get_refresh_token(&self, rt: &String) -> sqlx::Result<Option<RefreshToken>, Error> {
        log::info!("auth_repo: getting refresh token for user");
        let token = sqlx::query_as("SELECT *  FROM refresh_tokens WHERE token = $1")
            .bind(rt)
            .fetch_optional(&self.pool)
            .await?;

        Ok(token)
    }

    async fn revoke_refresh_token(&self, rt: &String) -> sqlx::Result<u64, Error> {
        let result = sqlx::query("UPDATE refresh_tokens SET revoked = true WHERE token = $1")
            .bind(rt)
            .execute(&self.pool)
            .await?;
        
        Ok(result.rows_affected())
    }

    async fn invalidate_refresh_token(&self, rt: &String) -> sqlx::Result<u64, Error> {
        let result = sqlx::query("DELETE FROM refresh_tokens WHERE token = $1")
            .bind(rt)
            .execute(&self.pool)
            .await?;
        
        Ok(result.rows_affected())
    }
}
