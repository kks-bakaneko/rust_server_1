use actix_files as fs;
use actix_web::{error, web, App, Error, HttpResponse, HttpServer, Result};
use actix_web::web::Data;
use tera::Tera;
use std::env;

async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", "Bob");
    let s = tmpl
        .render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template Error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string()); //env에서 서버 어드레스 받아오는데, env가 없을 경우 127.0.0.1:8080으로 저장
    //println!("Listening on: {}, open browser and visit have a try!", addr);
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"),"/static/iter1/**/*")).unwrap();
        App::new()
            .app_data(Data::new(tera))
            .service(fs::Files::new("/static", "./static").show_files_listing()) 
            .service(web::resource("/").route(web::get().to(index)))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

