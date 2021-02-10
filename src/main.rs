#[macro_use]
extern crate log;

use actix_web::{middleware::Logger, web, App, HttpServer};
use aos_auth::{Config, auth, app::views::index};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_envs();
    let address = config.get_address();
    let password_hasher = config.get_password_hasher();
    let data_layer = config.get_datalayer();
    debug!("Open to http://{}", &address);

    HttpServer::new(move || {
        App::new()
            .data(config.clone())
            .data(data_layer.clone())
            .data(password_hasher.clone())
            .wrap(Logger::default())
            .service(
                web::scope("/auth")
                    .route("/login/", web::get().to(auth::views::login_get))
                    .route("/login/", web::post().to(auth::views::login_post))
                    .route("/logout/", web::get().to(auth::views::logout_get)),
            )
            .route("/", web::get().to(index))
    })
    .bind(address)?
    .run()
    .await
}
