use crate::api::*;
use actix_web::web;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/v1").service(
                web::scope("/auth")
                    .service(
                        web::resource("/signup").route(web::get().to(account_controller::signup)),
                    )
                    .service(
                        web::resource("/verify").route(web::get().to(account_controller::verify)),
                    ),
            ),
        ),
    );
}
