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

    pub async fn get_user(&self, phone_number: &String) -> Result<User, Error> {
        match self.repo.get_user_by_phone(phone_number).await {
            Ok(user) => {
                match user {
                    Some(user) => Ok(user),
                    None => Err(Error::new(ErrorKind::NotFound, "User not found"))
                }
            },
            Err(e) => Err(Error::other(e.to_string())),
        }
    }
}