use actix_web::dev::Payload;
use actix_web::{web, FromRequest, HttpRequest, HttpResponse};
use serde::Serialize;
use std::future::{ready, Ready};

use crate::config::jwt::JwtClaims;
use crate::config::JwtConfig;

/// Extracted from a valid JWT Bearer token. Available in handler parameters.
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: String,
    pub email: String,
    pub roles: Vec<String>,
}

impl AuthenticatedUser {
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    pub fn is_admin(&self) -> bool {
        self.has_role("admin") || self.has_role("superadmin")
    }

    pub fn is_super_admin(&self) -> bool {
        self.has_role("superadmin")
    }
}

impl From<JwtClaims> for AuthenticatedUser {
    fn from(claims: JwtClaims) -> Self {
        AuthenticatedUser {
            user_id: claims.sub,
            email: claims.email,
            roles: claims.roles,
        }
    }
}

#[derive(Debug, Serialize)]
struct AuthError {
    error: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let jwt_config = req.app_data::<web::Data<JwtConfig>>();

        let result = (|| {
            let config = jwt_config.ok_or_else(|| {
                actix_web::error::InternalError::from_response(
                    "JWT config not found",
                    HttpResponse::InternalServerError().json(AuthError {
                        error: "Internal server error".to_string(),
                    }),
                )
            })?;

            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|v| v.to_str().ok())
                .ok_or_else(|| {
                    actix_web::error::InternalError::from_response(
                        "Missing Authorization header",
                        HttpResponse::Unauthorized().json(AuthError {
                            error: "Missing Authorization header".to_string(),
                        }),
                    )
                })?;

            if !auth_header.starts_with("Bearer ") {
                return Err(actix_web::error::InternalError::from_response(
                    "Invalid Authorization header format",
                    HttpResponse::Unauthorized().json(AuthError {
                        error: "Invalid Authorization header format".to_string(),
                    }),
                ));
            }

            let token = &auth_header[7..];

            let claims = config.validate_token(token).map_err(|e| {
                tracing::debug!("JWT validation failed: {e}");
                actix_web::error::InternalError::from_response(
                    "Invalid or expired token",
                    HttpResponse::Unauthorized().json(AuthError {
                        error: "Invalid or expired token".to_string(),
                    }),
                )
            })?;

            if claims.token_type != "access" {
                return Err(actix_web::error::InternalError::from_response(
                    "Invalid token type",
                    HttpResponse::Unauthorized().json(AuthError {
                        error: "Invalid token type".to_string(),
                    }),
                ));
            }

            Ok(AuthenticatedUser::from(claims))
        })();

        ready(result.map_err(actix_web::Error::from))
    }
}

/// Role-based access control guard. Use in handlers via `RequireRole`.
pub struct RequireRole {
    pub user: AuthenticatedUser,
}

impl RequireRole {
    pub fn admin(user: AuthenticatedUser) -> Result<Self, HttpResponse> {
        if user.is_admin() {
            Ok(RequireRole { user })
        } else {
            Err(HttpResponse::Forbidden().json(AuthError {
                error: "Forbidden: Admin role required".to_string(),
            }))
        }
    }

    pub fn super_admin(user: AuthenticatedUser) -> Result<Self, HttpResponse> {
        if user.is_super_admin() {
            Ok(RequireRole { user })
        } else {
            Err(HttpResponse::Forbidden().json(AuthError {
                error: "Forbidden: SuperAdmin role required".to_string(),
            }))
        }
    }
}
