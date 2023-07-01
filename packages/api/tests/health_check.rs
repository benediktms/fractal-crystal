use std::net::TcpListener;

use api::{
    configuration::{get_configuration, DatabaseSettings},
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use migration::ConnectionTrait;
use sea_orm_migration::{
    sea_orm::{Database, DatabaseBackend, DatabaseConnection, Statement},
    MigratorTrait,
};

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    println!("Address: {}", &app.address);

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

pub struct TestApp {
    address: String,
    // conn: DatabaseConnection,
}

pub async fn get_test_db(config: &DatabaseSettings) -> DatabaseConnection {
    let conn = Database::connect(config.connection_string_without_db().as_str())
        .await
        .expect("Failed to connect to database");

    conn.execute(Statement::from_string(
        DatabaseBackend::Postgres,
        format!(r#"CREATE DATABASE "{}";"#, config.db_name).to_string(),
    ))
    .await
    .expect("Failed to create database");

    migration::Migrator::up(&conn, None)
        .await
        .expect("Failed to migrate database");

    conn
}

async fn spawn_app() -> TestApp {
    // let subscriber = get_subscriber("test".into(), "debug".into());
    // init_subscriber(subscriber);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    let port = configuration.application_port;
    let address = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(&address).expect("Failed to bind to random port");
    println!("Address: {}", address);

    let id = nanoid::nanoid!();
    let date = chrono::Local::now().format("%d-%m-%Y").to_string();
    configuration.database.db_name = format!("{}-test-{}", date, id);

    let connection = get_test_db(&configuration.database).await;

    let server = run(listener, connection).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://{}", address),
        // conn: connection,
    }
}
