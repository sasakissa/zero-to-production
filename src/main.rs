use std::net::TcpListener;

use zero2prod::startup::run;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Faild to bind");
    let port = listener.local_addr().unwrap().port();
    println!("start subscribe 127.0.0.1:8000");
    run(listener)?.await
}
