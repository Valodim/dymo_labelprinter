use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use serde::Deserialize;

use dymo_label::picture;

#[macro_use]
extern crate log;

#[derive(Deserialize, Debug)]
struct TextFormData {
    label_text: String,
}

#[get("/preview/image/{image}")]
async fn preview_image(param: web::Path<String>) -> impl Responder {
    let result = handle_preview_text(param.to_string());
    match result {
        Ok(img) => HttpResponse::Ok().content_type("image/png").body(img),
        Err(err) => error_response(err),
    }
}

#[get("/preview/text/{label}")]
async fn preview_text(param: web::Path<String>) -> impl Responder {
    let result = handle_preview_text(param.to_string());
    match result {
        Ok(img) => HttpResponse::Ok().content_type("image/png").body(img),
        Err(err) => error_response(err),
    }
}

#[post("/print/text")]
async fn print_text(form: web::Form<TextFormData>) -> impl Responder {
    debug!("Form Data: {:?}!", form);
    let result = handle_print_text(&form.label_text);
    match result {
        Ok(_) => HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body("Go get your label!"),
        Err(err) => error_response(err),
    }
}

#[get("/text")]
async fn text() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/text.html"))
}

#[get("/image")]
async fn image() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/image.html"))
}

#[get("/static/site.css")]
async fn css() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(include_str!("../static/site.css"))
}

fn handle_print_text(label_text: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("label text: {}", label_text);
    let pic = picture::create_image(&label_text, "Ubuntu")?;
    let bw_pic = picture::convert_to_bw(&pic, 128)?;

    dymo_label::print_label(&bw_pic)
}

fn handle_preview_text(label_text: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    info!("label text: {}", label_text);
    let pic = picture::create_image(&label_text, "Ubuntu")?;
    let bw_pic = picture::convert_to_bw(&pic, 128)?;

    picture::encode_png(&bw_pic)
}

fn error_response(err: Box<dyn std::error::Error>) -> HttpResponse {
    error!("{}", err);
    HttpResponse::InternalServerError().body(err.to_string())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("debug")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(css)
            .service(text)
            .service(image)
            .service(preview_text)
            .service(print_text)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
