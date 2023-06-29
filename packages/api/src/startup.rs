use actix_web::{dev::Server, web, App, HttpServer};
use sea_orm_migration::sea_orm::DatabaseConnection;
use std::net::TcpListener;

use crate::routes::health_check;

#[derive(Clone, Debug)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

pub fn run(listener: TcpListener, conn: DatabaseConnection) -> Result<Server, std::io::Error> {
    let state = web::Data::new(AppState { conn });
    let server = HttpServer::new(move || App::new().service(health_check).app_data(state.clone()))
        .listen(listener)?
        .run();

    Ok(server)
}
