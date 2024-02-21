
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tera;

use actix_files as fs;
use actix_session::CookieSession;
use actix_web::middleware::{errhandlers::ErrorHandlers, Logger};
use actix_web::{http, web, App, HttpServer}; 
use api::user_api::{create_user, delete_user, get_all_users, get_user, update_user};
use repository::mongodb_repo::MongoRepo;

mod api;
mod models;
mod repository;
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let db = MongoRepo::init().await;
//     let db_data = Data::new(db);
//     HttpServer::new(move || {
//         App::new()
//             .app_data(db_data.clone())
//             .service(create_user)
//             .service(get_user)
//             .service(update_user)
//             .service(delete_user)
//             .service(get_all_users)
//     })
//     .bind(("0.0.0.0:8088", 8080))?
//     .run()
//     .await
// }

static SESSION_SIGNING_KEY: &[u8] = &[0; 32];

fn main() -> io::Result<()> {
    dotenv().ok();

    env::set_var("RUST_LOG", "actix_todo=debug,actix_web=info");
    env_logger::init();



    let app = move || {
        debug!("Constructing the App");

        let db = MongoRepo::init().await;
        let db_data = Data::new(db);

        let session_store = CookieSession::signed(SESSION_SIGNING_KEY).secure(false);

        // let error_handlers = ErrorHandlers::new()
        //     .handler(
        //         http::StatusCode::INTERNAL_SERVER_ERROR,
        //         api::internal_server_error,
        //     )
        //     .handler(http::StatusCode::BAD_REQUEST, api::bad_request)
        //     .handler(http::StatusCode::NOT_FOUND, api::not_found);

        App::new()
            .wrap(Logger::default())
            .wrap(session_store) 
            .service(web::resource("/users").route(web::post().to_async(api::get_all_users)))

            .service(fs::Files::new("/static", "static/"))
    };

    debug!("Starting server");
    HttpServer::new(app).bind("0.0.0.0:8088")?.run()
}