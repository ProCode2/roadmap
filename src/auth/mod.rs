use jsonwebtoken::{decode, DecodingKey, Validation};
use rocket::http::CookieJar;
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
        // Access the cookies from the request
        let cookies: &CookieJar = request.cookies();

        // Retrieve the cookie named "auth_cookie"
        if let Some(cookie) = cookies.get("auth_cookie") {
            let token = cookie.value();

            let secret_key = std::env::var("AUTH_SECRET_KEY")
                .ok()
                .expect("Cannot find auth secret key");

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

        // If any step fails, return an Unauthorized status
        Outcome::Error((Status::Unauthorized, ()))
    }
}
