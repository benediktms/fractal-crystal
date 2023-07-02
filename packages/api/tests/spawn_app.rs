use std::net::TcpListener;

use api::{
    configuration::{get_configuration, DatabaseSettings},
    startup::run,
};
use migration::ConnectionTrait;
use sea_orm_migration::{
    sea_orm::{Database, DatabaseBackend, DatabaseConnection, Statement},
    MigratorTrait,
};

pub struct TestApp {
    pub address: String,
    pub api_client: reqwest::Client,
    conn: DatabaseConnection,
}

impl TestApp {
    pub async fn post<Body>(&self, path: &str, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(&format!("{}/{}", &self.address, path))
            .form(&body)
            .send()
            .await
            .expect("Failed to execute reequest")
    }
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

pub async fn spawn_app() -> TestApp {
    // let subscriber = get_subscriber("test".into(), "debug".into());
    // init_subscriber(subscriber);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    let port = configuration.application_port;
    let address = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(&address).expect("Failed to bind to random port");

    let id = nanoid::nanoid!();
    let date = chrono::Local::now().format("%d-%m-%Y").to_string();
    configuration.database.db_name = format!("{}-test-{}", date, id);

    let connection = get_test_db(&configuration.database).await;

    let server = run(listener, connection.clone()).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://{}", address),
        api_client: reqwest::Client::new(),
        conn: connection,
    }
}
