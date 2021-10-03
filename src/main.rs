#![deny(
    //  missing_docs, // not compatible with big_array
      trivial_casts,
      trivial_numeric_casts,
      unsafe_code,
      unused_import_braces,
      unused_qualifications,
      warnings
  )]

#[cfg(test)]
mod tests;

mod services;
mod actix_actors;

use std::io::Read;
use std::fs::File;
use actix::prelude::*;
use lazy_static::lazy_static;
use actix_files as fs;
use actix_web_actors::ws;
// use actix_files::NamedFile;
// use std::path::PathBuf;
use actix_web::{web, App, HttpRequest, Error, HttpResponse, HttpServer, Result as ActixResult, middleware::Logger};
use actix_actors::{FeedActor, WsSession};
use services::data_processor::*;
use services::types::WebAppConfig;
use log::warn;
use std::sync::Once;

const LOG_CONFIG_PATH: &str = "log_config.yaml";
static INIT: Once = Once::new();
pub fn setup_log() -> () { 
    INIT.call_once(|| {    
        log4rs::init_file(LOG_CONFIG_PATH, Default::default()).unwrap();
    });
}

const CONFIG_PATH: &str = "config.json"; 

const PATH_SERVER: &str = "./www/dist/";

lazy_static! {
    static ref CONFIG: WebAppConfig = setup_config(CONFIG_PATH);
}
fn setup_config(path: &str) -> WebAppConfig {
    let task_name = "--setup_config--";

    let mut file = match File::open(path){
        Ok(file) => file,
        Err(err) => {
            log::error!("Error in {:?}\nFile open:\n{:?}", task_name, err);
            panic!("Error in {:?}\nFile open:\n{:?}", task_name, err)
        }
    };
    let mut buff = String::new(); 
    if let Err(err) = file.read_to_string(&mut buff){
        log::error!("Error in {:?}\nFile Read To String :\n{:?}", task_name, err);
        panic!("Error in {:?}\nFile Read To String :\n{:?}", task_name, err)

    };

    let config = match WebAppConfig::new(buff){
        Ok(config) => config,
        Err(err) => {
            log::error!("Error in {:?}\nDeserialize config :\n{:?}", task_name, err);
            panic!("Error in {:?}\nDeserialize config :\n{:?}", task_name, err);
        }
    };
    config
}


async fn ws_stream_rates(req: HttpRequest, stream: web::Payload) -> ActixResult<HttpResponse, Error> {
    warn!("Stream Called");
    let resp = ws::start(WsSession::new(), &req, stream);
    resp
}

// async fn index(_: HttpRequest) -> ActixResult<NamedFile> {
//     warn!("Just Entered index");

//     let my_path = PATH_SERVER.to_owned() + "index.html";

//     let path: PathBuf = my_path.parse().unwrap();
//     warn!("Path {:?}", path);
//     Ok(NamedFile::open(path)?)
// }

#[rustfmt::skip]
#[actix_web::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    setup_log();
    std::env::set_var("RUST_LOG", "warn");
    std::env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();
    warn!("Server INIT");

    let feed_actor_addr = FeedActor.start();
    tokio::spawn(stream_init_task(feed_actor_addr));

    let server_address = CONFIG.server_address.clone();
    let server_port = CONFIG.server_port.clone();

    HttpServer::new(|| {
        let logger = Logger::default();
        App::new().wrap(logger)
        // .route("/index", web::get().to(index))
        .route("/rates", web::get().to(ws_stream_rates))
        .service(fs::Files::new("/", PATH_SERVER).index_file("index.html"))
    })
    .bind((server_address.clone(), server_port))?
    .run()
    .await
}
