use actix_web::{
    http::header::{
        CACHE_CONTROL,
        CacheControl,
        CacheDirective,
    },
    HttpRequest,
    Responder,
};
use actix_files::NamedFile;
use std::path::PathBuf;


pub fn r_file(
    filename: &str,
) -> String {
    "/files/{filename}".replace("{filename}", filename)
}

pub async fn files_get(
    req: HttpRequest,
) -> impl Responder {
    let pathreq: PathBuf = req.match_info().query("filename").parse().unwrap();
    let mut path: PathBuf = PathBuf::from("files/");
    path.push(pathreq.as_path());

    let named_file = match NamedFile::open(path.clone()) {
        Ok(nf) => nf.customize().insert_header((
            CACHE_CONTROL,
            CacheControl(vec![
                CacheDirective::MaxAge(31622400),  // 1 year
                CacheDirective::Extension("immutable".into(), None),
            ])
        )),
        // TODO: Handle this properly
        Err(e) => {
            panic!(
                "ERROR IN STATIC FILES: {}; dir: {}",
                e,
                path.as_path().display(),
            );
        },
    };

    named_file
}

