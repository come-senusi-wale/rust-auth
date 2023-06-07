use actix_web::{Scope, web, HttpResponse};
use chrono::{Utc, Duration};
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
use serde::{Serialize, Deserialize};
use crate::ectraction::authetication_token::AuthenticationToken;

pub fn user_scope() -> Scope {
    web::scope("/user")
    .route("/encode-token/{id}", web::get().to(encode_token))
    .route("/decode-token", web::post().to(decode_token))
    .route("/protected", web::get().to(protected))
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,   
}

#[derive(Serialize, Deserialize)]
struct EncodeResponse {
    message: String,
    token: String
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: usize,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
struct DecodeBody {
    token: String,
}


#[derive(Serialize, Deserialize)]
struct DecodeResponse {
    message: String,
    id: usize
}

async fn encode_token(path: web::Path<usize>, secret: web::Data<String>)-> HttpResponse{
    let id: usize = path.into_inner();
    let exp: usize = (Utc::now() + Duration::days(365)).timestamp() as usize;
    let claims: Claims = Claims{id, exp};

    let token: String = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_str().as_ref())
    ).unwrap();

    HttpResponse::Ok().json(EncodeResponse{
        message: "success".to_owned(),
        token,
    })
}

async fn decode_token(body: web::Json<DecodeBody>, secret: web::Data<String>) -> HttpResponse{
    let decode: Result<TokenData<Claims>, JWTError> = decode::<Claims>(
        &body.token,
        &DecodingKey::from_secret(secret.as_str().as_ref()),
        &Validation::new(Algorithm::HS256)
    );

    match decode {
        Ok(token)=>HttpResponse::Ok().json( DecodeResponse{
            message: "Authorize".to_string(),
            id: token.claims.id,
        }),
        Err(e)=> HttpResponse::BadRequest().json(Response {message: e.to_string()})
    }
    //HttpResponse::Ok().json(Response{message: "decode_token".to_owned()})
}

async fn protected(auth_token: AuthenticationToken) -> HttpResponse{
    println!("{}", auth_token.id);

    //HttpResponse::Ok().json(Response{message: "protected".to_owned()})
    HttpResponse::Ok().json(Response{message: auth_token.id.to_string()})
} 