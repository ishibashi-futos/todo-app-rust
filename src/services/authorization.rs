use actix_web::dev::{Service, ServiceRequest};
use actix_web::HttpResponse;
use serde_json::json;

pub async fn check_authorization_header<S>(
    req: ServiceRequest,
    server: &S,
) -> S::Future
where
    S: Service<ServiceRequest>
{
    if req.path() == "/health" {
        server.call(req)
    }

    let error_message = json!({
        "error": "Authorization is failure"
    });
    let res = HttpResponse::Unauthorized().json(error_message);
    ServiceRe

    Err(actix_web::error::ErrorUnauthorized(format!("{:?}", res)))
}
