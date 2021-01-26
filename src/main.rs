use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{configurations::get_configuration, startup::run};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to load configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect postgres. ");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))
        .expect("Faild to bind");
    let port = listener.local_addr().unwrap().port();
    println!("start subscribe 127.0.0.1:8000");
    run(listener, connection_pool)?.await
}
