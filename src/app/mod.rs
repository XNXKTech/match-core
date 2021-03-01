use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}

async fn match_request(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
pub async fn run(bind_address: String) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(routes)
    })
    .bind(&bind_address)
    .expect("Can not bind to {}")
    .run()
    .await
}

fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/").to(index))
        .service(web::resource("/match").to(match_request));
}
