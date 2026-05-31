use std::io::{Error, ErrorKind};
use crate::domains::auth::error::AppError;
use crate::domains::auth::models::{AuthTokensResponse, Claims, LoginRequest, OtpResponse, RefreshToken, SignupRequest, VerifyOtpRequest};
use crate::domains::auth::repository::verification_codes::repo::OtpRepo;
use chrono::{DateTime, Duration, Local, Utc};
use jsonwebtoken::{encode, Header, EncodingKey, Algorithm};
use rand::Rng;
use std::sync::Arc;
use actix_web::web::Data;
use crate::domains::auth::repository::refresh_tokens::repo::RefreshTokenRepo;
use crate::domains::notifications::models::{EmailVerificationPayload, NotificationType};
use crate::domains::notifications::service::NotificationService;
use crate::domains::users::models::User;
use crate::domains::users::service::UserService;

pub struct AuthService {
    pub jwt_secret: String,
    pub rt_repo: Arc<dyn RefreshTokenRepo + Send + Sync>,
    pub otp_repo: Arc<dyn OtpRepo + Send + Sync>,
}

impl AuthService {
    // Authentication
    pub async fn signup(&self, user_service: Data<UserService>, payload: SignupRequest) -> Result<OtpResponse, Error> {
        // Insert new user into database
        let user = User::new(payload.clone().full_name, payload.school_name, payload.grade, payload.email);
        match user_service.create_user(user).await {
            Ok(user) => {
                match self.generate_otp(user_service, &user.id).await {
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
        match user_service.get_user_by_email(&payload.email).await {
            Ok(user) => {
                match self.generate_otp(user_service, &user.id).await {
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

    // Auth Tokens
    pub fn generate_access_token(&self, user_id: &str) -> Result<String, AppError> {
        let now = Utc::now().timestamp();
        let expires_at = now + 3600; // 60 minutes from now

        let claims = Claims {
            sub: user_id.to_string(),
            iat: now,
            exp: expires_at,
        };

        let header = Header::new(Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(self.jwt_secret.as_bytes());

        match encode(&header, &claims, &encoding_key) {
            Ok(token) => Ok(token),
            Err(e) => Err(AppError::TokenGenerationError(e.to_string())),
        }
    }
    async fn generate_refresh_token(&self, user_id: &String) -> Result<RefreshToken, Error> {
        let rng = rand::rng();
        let token: String = rng.sample_iter(&rand::distr::Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

        let id = uuid::Uuid::now_v7().to_string();
        let now = Utc::now();
        let expires_at = now + Duration::days(30);

        let token = RefreshToken {
            id,
            user_id: user_id.clone(),
            token,
            expires_at,
            created_at: now,
            revoked: false,
        };

        match self.rt_repo.store_refresh_token(&token).await {
            Ok(refresh_token) => {
                log::info!("Refresh token stored successfully: {:?}", refresh_token);
                Ok(refresh_token)
            },
            Err(e) => {
                log::error!("AuthService: Error storing refresh token: {}", e);
                Err(Error::other(e.to_string()))
            }
        }
    }

    // Verification Codes
    async fn generate_otp(&self, user_service: Data<UserService>,user_id: &String) -> Result<OtpResponse, Error> {
        // Generate secure 6-digit OTP
        let mut rng = rand::rng();
        let otp_code: String = format!("{:06}", rng.random_range(0..1_000_000));
        let now =  Local::now();
        let expires_at = now + Duration::minutes(10);
        match self.otp_repo.store_otp(user_id, &otp_code, &expires_at, &now).await {
            Ok(otp) => {
                log::info!("Expiry time converted back to DateTime object {}", DateTime::<Local>::from(expires_at));
                log::info!("OTP stored successfully: {:?}", otp);
                let email = std::env::var("EMAIL_FROM").expect("EMAIL_FROM environment variable is required");
                let user = user_service.get_user(user_id).await?;
                let payload = EmailVerificationPayload {
                    from: email,
                    to: user.email.clone(),
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

    pub async fn verify_otp(&self, user_service: Data<UserService>, payload: VerifyOtpRequest) -> Result<AuthTokensResponse, Error> {
        let user = user_service.get_user_by_email(&payload.email).await?;
        log::info!("User found: {:?}", user.id);
        match self.otp_repo.get_otp(&payload.code, &user.id).await {
            Ok(otp) => {
                match otp {
                    Some(otp) => {
                        log::info!("OTP verified successfully: {:?}", otp);

                        // Flip the is_verified status to true
                        let updated = user_service.update_verify_status(&otp.user_id).await;

                        match updated {
                            Ok(updated_user) => {
                                // Generate an access token
                                let access_token = self.generate_access_token(&otp.user_id)
                                    .map_err(|e| Error::other(e.to_string()))?;

                                // Generate refresh token
                                let refresh_token_data = self.generate_refresh_token(&otp.user_id).await?;

                                // Invalidate OTP
                                match self.invalidate_otp(payload).await {
                                    Ok(rows_affected) => {
                                        log::info!("Invalidating OTP: {:?} rows affected.", rows_affected);
                                    },
                                    Err(e) => {
                                        log::error!("Error: {}", e);
                                    }
                                }

                                Ok(AuthTokensResponse {
                                    access_token,
                                    refresh_token: refresh_token_data.token,
                                    user: updated_user
                                })
                            },
                            Err(e) => {
                                log::error!("Error: {}", e);
                                return Err(Error::other(e.to_string()));
                            }
                        }
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
        match self.otp_repo.invalidate_otp(&payload.code, &payload.email).await {
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
