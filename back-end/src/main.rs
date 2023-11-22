use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Serialize;
use std::collections::HashMap;

// This struct represents state
struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> Result<impl Responder> {
    let mut res = HashMap::new();
    res.insert("name", "hello");

    Ok(web::Json(res))
}

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/a/{name}")]
async fn name(name: web::Path<String>) -> Result<impl Responder> {
    let obj = MyObj {
        name: name.to_string(),
    };
    Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin() // This allows any origin. For production, specify your frontend origin.
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)

            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(index)
            .service(name)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}