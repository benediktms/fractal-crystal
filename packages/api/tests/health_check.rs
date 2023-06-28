use std::net::TcpListener;

use api::run;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");

    tokio::spawn(async move {
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    });

    format!("http://127.0.0.1:{port}")
}