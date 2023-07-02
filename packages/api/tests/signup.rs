mod spawn_app;

use api::routes::SignUpInput;
use spawn_app::spawn_app;

#[tokio::test]
async fn signup_should_create_a_new_user() {
    let app = spawn_app().await;

    let body = SignUpInput {
        email: "john.doe@example.com".to_owned(),
        password: "helloworld".to_owned(),
    };

    let response = app.post("signup", &body).await;

    assert_eq!(201, response.status());
}
