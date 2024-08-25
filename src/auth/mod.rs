use jsonwebtoken::{decode, DecodingKey, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

use crate::jwt::Claims;

pub struct AuthUser {
    pub id: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get the "Authorization" header
        let auth_header = request.headers().get_one("Authorization");

        let secret_key = std::env::var("AUTH_SECRET_KEY")
            .ok()
            .expect("Can not find auth secret key");
        if let Some(auth_header) = auth_header {
            // Check if the header is in the format "Bearer <token>"
            if auth_header.starts_with("Bearer ") {
                let token = &auth_header[7..]; // Extract the token

                // Decode the token
                let decoded = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(secret_key.as_ref()),
                    &Validation::default(),
                );

                // If decoding was successful, return the AuthUser
                if let Ok(decoded) = decoded {
                    return Outcome::Success(AuthUser {
                        id: decoded.claims.sub,
                    });
                }
            }
        }

        // If any step fails, return an Unauthorized status
        Outcome::Error((Status::Unauthorized, ()))
    }
}
