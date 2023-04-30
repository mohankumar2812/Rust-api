use futures::future::{err, ok, Ready};
use actix_web::error::ErrorBadRequest;
use crate::constant::JWT_SECRETKEY;
use crate::api::authorizaion::create_token::{decode_jwt, DecodeJwtHelper};
use actix_web::{dev, Error, FromRequest, HttpRequest};
use serde::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Deserialize, Debug, Serialize, Validate)]
pub struct AuthorizedUser {
    pub user_id: String,
}

impl FromRequest for AuthorizedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;


    fn from_request(request: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let auth_header =  request.headers().get("Authorization");
        let _split: Vec<&str> = auth_header.unwrap().to_str().unwrap().split("Bearer").collect();
        let token = _split[1].trim();
        match check_data_from_auth_header(Some(token)) {
            Ok(vec_header) => match decode_jwt(vec_header[0].to_string(), JWT_SECRETKEY) {

                DecodeJwtHelper::Ok(token_data) =>{
                    let auth_user = AuthorizedUser {
                        user_id: token_data.claims.user_id,
                    };

                    ok(auth_user)
                }
                DecodeJwtHelper::Err => err(ErrorBadRequest("Token expiry")),
            },
            Err(_) => err(ErrorBadRequest("un authorized user")),
        } 
    }

}

//check data from request auth
pub fn check_data_from_auth_header(auth_header: Option<&str>) -> Result<Vec<&str>, ()> {
    return if let Some(auth_string) = auth_header {
        let vec_header = auth_string.split_whitespace().collect::<Vec<_>>();
        if vec_header.len() != 2
            && vec_header[0] == "Bearer"
            && !vec_header[0].is_empty()
            && !vec_header[1].is_empty()
        {
            Err(())
        } else {
            Ok(vec_header)
        }
    } else {
        Err(())
    };
}
