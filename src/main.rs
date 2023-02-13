use actix_web::*;

mod routers;
use routers::{ping};

#[actix_web::main]

async fn main() -> Result<(), std::io::Error> {
    let api = HttpServer::new( || {
        App::new()
        .route("/ping", web::get().to(ping))
    });

    let port:i32 = 9091;

    let api = api.bind(format!("127.0.0.1:{}", port))
    .expect(" Não conseguiu conectar... ");

    println!(" conectado, porta: http://localhost:{}/ping", port);

    api.run()
    .await
}
