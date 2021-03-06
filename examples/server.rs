use std::fs::create_dir_all;
use std::path::PathBuf;

use actix::prelude::Actor;
use actix_web::{middleware, web, App, HttpServer};

use infotainer::{data_log::DataLogger, pubsub::PubSubService, websocket::websocket_handler};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let data_path = PathBuf::from("/tmp/infotainer-server-example");
    create_dir_all(&data_path)?;
    let data_logger_addr = DataLogger::new(&data_path)
        .expect("Could not initiate DataLogger")
        .start();
    let pubsub_server_addr = PubSubService::new(&data_logger_addr).start();
    HttpServer::new(move || {
        App::new()
            .data(pubsub_server_addr.clone())
            .data(data_logger_addr.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/ws/{session_id}").route(web::get().to(websocket_handler)))
    })
    .bind("127.0.0.1:1312")?
    .run()
    .await
}
