use actix_web::HttpRequest;

pub async fn middleware(req: HttpRequest) -> actix_web::Result<()> {
    Ok(())
}

pub async fn verify_token(req: HttpRequest) -> actix_web::Result<()> {
    Ok(())
}
