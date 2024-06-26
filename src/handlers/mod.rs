mod login;
mod logout;
mod status;
mod channels;
mod episodes;
mod users;
mod options;
mod feed;


use actix_web::web;
use tracing::info;

use super::{
    middleware::Authentication,
    models::{
        CustomResponse,
        Credentials,
        TokenClaims,
        AppState,
    }
};
use feed::web_feed;


pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes...");
    cfg.service(
        web::scope("")
            .service(
                web::redirect("/", "/app/")
            )
            .configure(web_feed)
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/1.0")
                            .service(
                                web::scope("")
                                    .service(
                                        web::resource("/logout/")
                                            .route(web::get().to(logout::get_logout)))
                                    .service(
                                        web::resource("/status/")
                                            .route(web::get().to(status::get_status)))
                                    .service(
                                        web::resource("/login/")
                                            .route(web::post().to(login::post_login)))
                                    .service(channels::read)
                                    .service(channels::read_with_pagination)
                                    .service(episodes::read_with_pagination)
                            )
                            .service(
                                web::scope("")
                                    .wrap(Authentication)
                                    .service(channels::create)
                                    .service(channels::update)
                                    .service(channels::delete)
                            )
                )
            ).service(
                web::scope("/config")
                    .wrap(Authentication)
                    //.configure(config_users)
            )
    );
}
