use crate::domains::auth::error::AppError;
use crate::domains::users::models::User;
use crate::domains::users::repo::UserRepository;
use sqlx::Error;

pub struct UserPostgresRepo {
    pub pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl UserRepository for UserPostgresRepo {
    async fn create_user(&self, user: User) -> sqlx::Result<User, AppError> {
        let existing_user =
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE email = $1")
                .bind(&user.email)
                .fetch_one(&self.pool)
                .await?;

        if existing_user > 0 {
            return Err(AppError::UserAlreadyExists);
        }
        let user = sqlx::query_as("
            INSERT INTO users (id, full_name, school_name, grade, email, avatar_url, is_verified, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
        ").bind(&user.id)
        .bind(&user.full_name)
        .bind(&user.school_name)
        .bind(&user.grade)
        .bind(&user.email)
        .bind(&user.avatar_url)
        .bind(&user.is_verified)
        .bind(&user.created_at)
        .bind(&user.updated_at)
        .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn get_user(&self, id: &str) -> sqlx::Result<Option<User>, Error> {
        sqlx::query_as("SELECT * FROM users WHERE id=$1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn get_user_by_email(&self, email: &String) -> sqlx::Result<Option<User>, AppError> {
        let user = sqlx::query_as("SELECT * FROM users WHERE email=$1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    async fn verify_user(&self, email: &String) -> sqlx::Result<u64, Error> {
        let results = sqlx::query("UPDATE users SET is_verified=true WHERE email=$1 RETURNING *")
            .bind(email)
            .execute(&self.pool)
            .await?;

        Ok(results.rows_affected())
    }
}
