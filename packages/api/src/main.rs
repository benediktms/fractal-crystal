use std::{net::TcpListener, time::Duration};

use api::{configuration::get_configuration, startup::run};
use sea_orm_migration::sea_orm::{ConnectOptions, Database};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    let mut opts = ConnectOptions::new(configuration.database.connection_string());
    opts.max_connections(100)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .sqlx_logging(true);
    let db = Database::connect(opts)
        .await
        .expect("Failed to connect to database");

    run(listener, db)?.await
}
