use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::error::Error;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_getrest() -> impl Responder {
    let htrq = match migetrest().await {
        Err(e) => e.to_string(),
        Ok(text) => text
    };
    HttpResponse::Ok().body(htrq)
}

#[actix_web::main]
//#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
                App::new()
                        .service(hello)
                        .route("/rest", web::get().to(manual_getrest))
    })
    .bind(("0.0.0.0", 3001))?
    .run()
    .await
}

async fn migetrest() -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let resp = client.get("http://192.168.1.42:8080/fabsapp-0.0.1-SNAPSHOT/api/v1/employees/1")
        .send()
        .await?;
    let body = resp.text().await?;
    println!("{:#?}", body);
    Ok(body)
}