use actix_web::{post, web, HttpResponse, Responder};
use entity::user::{ActiveModel, Entity as User, Role};
use sea_orm_migration::sea_orm::{ActiveValue, EntityTrait};
use serde::{Deserialize, Serialize};

use crate::startup::AppState;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignUpInput {
    pub email: String,
    pub password: String,
}

#[post("/signup")]
pub async fn signup(state: web::Data<AppState>, data: web::Json<SignUpInput>) -> impl Responder {
    let db = &state.conn;

    let user = User::insert(ActiveModel {
        email: ActiveValue::Set(data.email.clone()),
        role: ActiveValue::Set(Role::User),
        ..Default::default()
    })
    .exec_with_returning(db)
    .await
    .expect("Failed to insert user");

    tracing::trace!("New user: {:?}", user);

    HttpResponse::Created().json(user)
}
