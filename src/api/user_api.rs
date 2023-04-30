use crate::{models::user_model::User,models::login_user::LoginRequest, repository::mongodb_repo::MongoRepo};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json},
    HttpResponse,
};
use mongodb::bson::oid::ObjectId;
use crate::api::authorizaion::token_auth::AuthorizedUser;

#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        mail: new_user.mail.to_owned(),
        password: new_user.password.to_owned(),
    };

    let user_detail = db.create_user(data).await;

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::Ok().json(err),
    }
}

#[post("/user/login")]
pub async fn login_user(db: Data<MongoRepo>, new_login: Json<LoginRequest>) -> HttpResponse {

    let data = LoginRequest {
        login_id: new_login.login_id.to_owned(),
        password: new_login.password.to_owned(),
    };
    
    let user_details = db.login_user(data).await;

    match user_details {
        Ok(user)  => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::Ok().json(err),
    }
}

#[get("/user/{id}")]
pub async fn get_user(db: Data<MongoRepo>, _id: AuthorizedUser) -> HttpResponse {
    let id = _id.user_id;
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let user_detail = db.get_user(&id).await;

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/user/{id}")]
pub async fn update_user(
    db: Data<MongoRepo>,
    _id: AuthorizedUser,
    new_user: Json<User>,
) -> HttpResponse {
    let id = _id.user_id;
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_user.name.to_owned(),
        mail: new_user.mail.to_owned(),
        password: new_user.password.to_owned(),
    };

    let update_result = db.update_user(&id, data).await;

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id).await;

                return match updated_user_info {
                    Ok(user) => HttpResponse::Ok().json(user),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No user found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/user/{id}")]
pub async fn delete_user(db: Data<MongoRepo>, _id: AuthorizedUser) -> HttpResponse {
    let id = _id.user_id;
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let result = db.delete_user(&id).await;

    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("User successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("User with specified ID not found!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users")]
pub async fn get_all_users(db: Data<MongoRepo>) -> HttpResponse {
    let users = db.get_all_users().await;

    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
