use std::{env};
extern crate dotenv;

use actix_web::{web::Json};
use dotenv::dotenv;
use crate::{models::{user_model::User, response::ErrResponse}, models::{login_user::LoginRequest, tokens::Token}, api::authorizaion::create_token::encode_token_and_refresh};
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, UpdateResult},
    Client, Collection,
};
use bcrypt::{hash, verify};
use crate::repository::{RegisterRequest, LoginRequestError};

use crate::constant::{
    JWT_EXPIRY, JWT_SECRETKEY, REFRESH_JWT_EXPIRY, REFRESH_JWT_SECRETKEY
};

use super::JwtToken;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting to database");
        let db = client.database("newDB");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<Json<Token>,ErrResponse> {

        let pwd_hash = hash(new_user.password, 4);

        let new_doc = User {
            id: None,
            name: new_user.name,
            mail: new_user.mail,
            password: pwd_hash.unwrap(),
        };

        let user = self.validate_registration(&new_doc.mail).await;

        match user.unwrap() {
            RegisterRequest::NewUser => {
                let _user = self
                .col
                .insert_one(&new_doc, None)
                .await
                .ok()
                .expect("Error creating user");

                let _token = self.create_jwt(new_doc.mail.clone()).await;

                match _token {
                    JwtToken::Ok(_token) => Ok(Json(_token)),
                    JwtToken::InvalidToken => Err(ErrResponse{
                        status: false,
                        message: "wrong password".to_string(),
                    })

                }
            },
            RegisterRequest::AlreadyCreated => Err(ErrResponse{
                status: false,
                message: "wrong password".to_string(),
            }),
        }
    }

    pub async fn login_user(&self,new_login: LoginRequest) -> Result<Token, ErrResponse> {
        let login_user = LoginRequest {
            login_id: new_login.login_id,
            password: new_login.password,
        };

        let _user = self.validate_login(login_user.login_id).await;
        
        match _user.unwrap() {
            LoginRequestError::Ok(_user) => {
                let _token = self.create_jwt(_user.mail.clone()).await;
                match verify(login_user.password, &_user.password) {
                        Ok(true) => match _token {
                            JwtToken::Ok(_token) => Ok(_token),
                            JwtToken::InvalidToken => Err(ErrResponse{
                                status: false,
                                message: "JWT not creation".to_string(),
                            })
        
                        } 
                        Ok(false) => Err(ErrResponse{
                            status: false,
                            message: "wrong password".to_string(),
                        }),
                        Err(_) => Err(ErrResponse{
                            status: false,
                            message: "wrong password".to_string(),
                        }),
                }
                  
            }
            LoginRequestError::UserNotFound => Err(ErrResponse{
                            status: false,
                            message: "User Not Found".to_string(),
                        })
        }

    }

    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");

        Ok(user_detail.unwrap())
    }

    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "name": new_user.name,
                    "mail": new_user.mail,
                    "password": new_user.password
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");

        Ok(user_detail)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of users");
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
    }

    pub async fn validate_registration(&self ,mail : &String) -> Result<RegisterRequest, Json<Error>> {
        // let obj_mail = ObjectId::parse_str(mail).unwrap();
        let filter = doc! {"mail": mail};
        let _user = self
                .col
                .find_one(filter, None)
                .await
                .ok()
                .expect("Already exist");

        match _user {
            None => Ok(RegisterRequest::NewUser) ,
            Some(_user) => Ok(RegisterRequest::AlreadyCreated)
        }

    }

    pub async fn validate_login(&self, mail: String) -> Result<LoginRequestError, Json<Error>> {
        let filter = doc! {"mail": mail};
        let _user = self
                .col
                .find_one(filter, None)
                .await
                .ok()
                .expect("Already exist");

        match _user {
            None => Ok(LoginRequestError::UserNotFound),
            Some(_user) => Ok(LoginRequestError::Ok(_user))
        }
    }

    pub async fn create_jwt(&self, mail: String) -> JwtToken {

        let filter = doc! {"mail": mail};
        let _user = self
                .col
                .find_one(filter, None)
                .await
                .ok()
                .expect("Already exist");

            let id  = _user.unwrap().id;

        match encode_token_and_refresh(
            id.unwrap(),
                    JWT_SECRETKEY, 
                    REFRESH_JWT_SECRETKEY, 
                    REFRESH_JWT_EXPIRY, 
                    JWT_EXPIRY) 
            {
                Ok(tokens) => JwtToken::Ok(tokens),
                Err(_) => JwtToken::InvalidToken,
            }

                // Ok(Json(token.unwrap()))
        // match _token {
        //     Some() => JwtToken::Ok(_token),
        // }
        
    }

}
