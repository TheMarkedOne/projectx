use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer, Responder};
use bytes::BytesMut;
use flate2::write::GzEncoder;
use flate2::Compression;
use futures::{StreamExt, TryStreamExt};
use mime::APPLICATION_OCTET_STREAM;
use std::io::Write;

async fn compress_files(mut payload: Multipart) -> Result<impl Responder, Error> {
    println!("Compressing File...");

    let mut compressed_data = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = item?;

        let mut data = BytesMut::new();
        while let Some(chunk) = field.next().await {
            data.extend_from_slice(&chunk?);
        }

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&data)?;

        let compressed_chunk = encoder.finish()?;

        compressed_data.extend_from_slice(&compressed_chunk);
    }

    println!("Finished Compressing.");

    Ok(HttpResponse::Ok()
        .content_type(APPLICATION_OCTET_STREAM.as_ref())
        .header(
            "Content-Disposition",
            format!("attachment; filename=\"compressed_files.gz\""),
        )
        .body(compressed_data))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/upload").route(web::post().to(compress_files)))
    })
    .bind(("myrust", 3001))?
    .run()
    .await
}
