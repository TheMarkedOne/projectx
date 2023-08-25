use actix_multipart::form::{tempfile::{TempFile, TempFileConfig}, MultipartForm, };
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer, Responder};
use nanoid::nanoid;

const BASE_DIR: &str = "/home/ilia/Desktop/saas-compressor/uploads/";

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "files")]
    files: Vec<TempFile>
}

async fn save_files(MultipartForm(form): MultipartForm<UploadForm>) -> Result<impl Responder, Error> {
    
    for f in form.files {
        let path = format!("{}{}", BASE_DIR, nanoid!());
        f.file.persist(path).unwrap();
    }

    Ok(HttpResponse::Ok())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(TempFileConfig::default().directory(BASE_DIR))
            .service(
                web::resource("/upload")
                    .route(web::post().to(save_files))
            )
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
