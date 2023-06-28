use std::net::TcpListener;

use api::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let address = format!("127.0.0.1:{}", 8000);
    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
