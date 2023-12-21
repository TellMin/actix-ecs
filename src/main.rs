use actix_files as fs;
use actix_web::{get, App, HttpRequest, HttpServer, Result};
use std::path::PathBuf;

#[get("/")]
async fn index() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("wwwroot/index.html")?)
}

#[get("/{filename}")]
async fn pages(req: HttpRequest) -> Result<fs::NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let file_name = path.file_name().unwrap().to_str().unwrap();

    if file_name.contains(".") {
        let root_path = PathBuf::from("wwwroot");
        let file = fs::NamedFile::open(root_path.join(path));
        if let Ok(file) = file {
            return Ok(file);
        }
    }
    Ok(fs::NamedFile::open("wwwroot/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(pages)
            .service(fs::Files::new("/", "wwwroot"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
