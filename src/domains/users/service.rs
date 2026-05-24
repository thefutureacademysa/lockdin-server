use std::io::{Error, ErrorKind};
use std::sync::Arc;
use crate::domains::auth::error::AppError;
use crate::domains::users::models::User;
use crate::domains::users::repo::UserRepository;

pub struct UserService {
    pub repo: Arc<dyn UserRepository + Send + Sync>,
}

impl UserService {
    pub async fn create_user(&self, user: User) -> Result<User, AppError> {
        match self.repo.create_user(user).await {
            Ok(user) => Ok(user),
            Err(e) => Err(e),
        }
    }

    pub async fn get_user(&self, email: &String) -> Result<User, Error> {
        match self.repo.get_user_by_email(email).await {
            Ok(user) => {
                match user {
                    Some(user) => Ok(user),
                    None => Err(Error::new(ErrorKind::NotFound, "User not found"))
                }
            },
            Err(e) => {
                log::error!("UserService: Error: {}", e);
                Err(Error::other(e.to_string()))
            },
        }
    }

    pub async fn update_verify_status(&self, email: &String) -> Result <bool, AppError> {
        match self.repo.verify_user(email).await {
            Ok(rows_affected) => Ok(rows_affected > 0),
            Err(e) => {
                log::error!("UserService: Error: {}", e);
                Err(AppError::from(e))
            }
        }
    }
}