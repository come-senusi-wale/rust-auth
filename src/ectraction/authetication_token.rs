use std::future::{Ready, ready};
use actix_web::{
    FromRequest,
    Error as ActixWebError,
    error::ErrorUnauthorized,
    HttpRequest,
    dev::Payload,
    http,
    http::header::HeaderValue,
    web,
};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{
    encode,
    decode,
    Header,
    EncodingKey, 
    DecodingKey,
    Validation,
    Algorithm,
    TokenData,
    errors::Error as JWTError,
}; 

use crate::scopes::Claims;

#[derive(Serialize, Deserialize)]
pub struct AuthenticationToken {
    pub id: usize,
}

impl FromRequest for AuthenticationToken{
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload)-> Self::Future{

        //get auth token from the authorization header

        let auth_header: Option<&HeaderValue> = req.headers().get(http::header::AUTHORIZATION);

        let auth_token: String = auth_header.unwrap().to_str().unwrap_or("").to_string();

        if auth_token.is_empty(){
            return ready(Err(ErrorUnauthorized("invalid auth token")));
        }

        let secret: String = req.app_data::<web::Data<String>>().unwrap().to_string();

        //decode token with the secret

        let decode: Result<TokenData<Claims>, JWTError> = decode::<Claims>(
            &auth_token,
            &DecodingKey::from_secret(secret.as_str().as_ref()),
            &Validation::new(Algorithm::HS256)
        );

        //reture authentication
        match decode{
            Ok(token) => ready(Ok(AuthenticationToken {id: token.claims.id})),
            Err(_) => ready(Err(ErrorUnauthorized("Unauthorized"))),
        }


    }
}