use crate::services::auth_service;

use actix_web::{ HttpRequest, HttpResponse, Result};
use futures::future::ready;

pub async fn signup() -> Result<HttpResponse>{  
   let token = auth_service::signup();
   return Ok(HttpResponse::Ok().json(token));
}


pub async fn verify(req: HttpRequest) -> Result<HttpResponse, auth_service::VerifyError> {
    let is_valid = ready(auth_service::verify(req)).await;

    match is_valid {
        Ok(is_valid) => {
            if is_valid {
                Ok(HttpResponse::Ok().finish())
            } else {
                Ok(HttpResponse::Unauthorized().finish())
            }
        },
        Err(e) => Err(e),
    }
}
