use std::io::{Error, ErrorKind};
use crate::domains::auth::models::{LoginRequest, OtpResponse, SignupRequest, VerifyOtpRequest};
use crate::domains::auth::repo::OtpRepo;
use chrono::{DateTime, Duration, Local};
use rand::Rng;
use std::sync::Arc;
use actix_web::web::Data;
use crate::domains::notifications::models::{EmailVerificationPayload, NotificationType};
use crate::domains::notifications::service::NotificationService;
use crate::domains::users::models::User;
use crate::domains::users::service::UserService;

pub struct AuthService {
    pub repo: Arc<dyn OtpRepo + Send + Sync>,
}

impl AuthService {
    pub async fn signup(&self, user_service: Data<UserService>, payload: SignupRequest) -> Result<OtpResponse, Error> {
        // Insert new user into database
        let user = User::new(payload.clone().full_name, payload.school_name, payload.grade, payload.email);
        match user_service.create_user(user).await {
            Ok(user) => {
                match self.generate_otp(&user.email).await {
                    Ok(otp) => {
                        Ok(otp)
                    },
                    Err(e) => Err(Error::from(e))
                }
            },
            Err(e) => {
                log::error!("Error: {}", e);
                Err(Error::other(e.to_string()))
            }
        }




    }
    pub async fn login(&self, user_service: Data<UserService>, payload: LoginRequest) -> Result<OtpResponse, Error> {
        match user_service.get_user(&payload.email).await {
            Ok(user) => {
                match self.generate_otp(&user.email).await {
                    Ok(otp) => {
                        Ok(otp)
                    },
                    Err(e) => Err(Error::from(e))
                }
            }
            Err(e) => {
                log::error!("Error: {}", e);
                Err(Error::from(e))
            }
        }
    }

    /** Generate Otp and store it or not*/
    async fn generate_otp(&self, contact: &String) -> Result<OtpResponse, Error> {
        // Generate secure 6-digit OTP
        let mut rng = rand::rng();
        let otp_code: String = format!("{:06}", rng.random_range(0..1_000_000));
        let now =  Local::now();
        let expires_at = now + Duration::minutes(10);
        match self.repo.store_otp(contact, &otp_code, &expires_at, &now).await {
            Ok(otp) => {
                log::info!("Expiry time converted back to DateTime object {}", DateTime::<Local>::from(expires_at));
                log::info!("OTP stored successfully: {:?}", otp);
                let email = std::env::var("EMAIL_FROM").expect("EMAIL_FROM environment variable is required");
                let payload = EmailVerificationPayload {
                    from: email,
                    to: otp.email.clone(),
                    code: otp.code.clone(),
                };
                let notification = NotificationType::EmailVerification(payload);
                NotificationService::send_notification(notification).await.unwrap();
                Ok(otp)
            },
            Err(e) => {
                log::error!("AuthService: Error: {}", e);
                Err(Error::other(e.to_string()))
            }
        }
    }
    pub async fn verify_otp(&self, user_service: Data<UserService>, payload: VerifyOtpRequest) -> Result<String, Error> {
        match self.repo.get_otp(&payload.code, &payload.email).await {
            Ok(otp) => {
                match otp {
                    Some(otp) => {
                        log::info!("OTP verified successfully: {:?}", otp);
                        // flip the is_verified status to true
                         let _updated = user_service.update_verify_status(&otp.email).await;
                        // invalidate here...
                        match self.invalidate_otp(payload).await {
                            Ok(rows_affected) => {
                               log::info!("Invalidating OTP: {:?} rows affected.", rows_affected);
                            },
                            Err(e) => {
                                log::error!("Error: {}", e);
                            }
                        }
                        Ok(String::from("Verified"))
                    },
                    None => {
                        log::error!("OTP not found");
                        Err(Error::new(ErrorKind::NotFound, "OTP not found or has expired."))
                    }
                }
            },
            Err(e) => {
                log::error!("Error: {}", e);
                Err(Error::other(e.to_string()))
            }
        }
    }

    pub async fn invalidate_otp(&self, payload: VerifyOtpRequest) -> Result<u64, Error> {
        match self.repo.invalidate_otp(&payload.code, &payload.email).await {
            Ok(rows_affected) => {
                log::info!("Invalidating OTP: {:?} rows affected.", rows_affected);
                Ok(rows_affected)
            },
            Err(e) => {
                log::error!("Error: {}", e);
                Err(Error::other(e.to_string()))
            }
        }
    }
}
