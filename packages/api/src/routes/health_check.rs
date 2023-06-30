use actix_web::{get, web, HttpResponse, Responder};
use entity::user::{ActiveModel, Entity as User, Role};
use sea_orm_migration::sea_orm::{ActiveValue, EntityTrait};

use crate::startup::AppState;

#[get("/health_check")]
pub async fn health_check(data: web::Data<AppState>) -> impl Responder {
    let db = &data.conn;

    let user = User::insert(ActiveModel {
        email: ActiveValue::Set("john.doe@example.com".to_owned()),
        role: ActiveValue::Set(Role::User),
        ..Default::default()
    })
    .exec_with_returning(db)
    .await
    .expect("Failed to insert user");

    println!("New user: {:?}", user);

    HttpResponse::Ok().json(user)
}
