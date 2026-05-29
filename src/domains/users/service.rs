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

    pub async fn get_user_by_email(&self, email: &String) -> Result<User, Error> {
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

    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        match self.repo.get_user(id).await {
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

    pub async fn update_verify_status(&self, user_id: &String) -> Result <User, AppError> {
        match self.repo.verify_user(user_id).await {
            Ok(updated_user) => Ok(updated_user),
            Err(e) => {
                log::error!("UserService: Error: {}", e);
                Err(AppError::from(e))
            }
        }
    }
}