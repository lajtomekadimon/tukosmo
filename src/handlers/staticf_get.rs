use actix_web::{
    web,
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


pub async fn staticf_get(
    codename: web::Data<String>,
    req: HttpRequest,
) -> impl Responder {
    let codename_req: String = req.match_info()
        .get("codename").unwrap().parse().unwrap();

    if codename.to_string() != codename_req {
        panic!("That URL has expired!");
    }

    let pathreq: PathBuf = req.match_info().query("filename").parse().unwrap();
    let mut path: PathBuf = PathBuf::from("static/");
    path.push(pathreq.as_path());

    let named_file = match NamedFile::open(path.clone()) {
        Ok(nf) => nf.with_header(
            CACHE_CONTROL,
            CacheControl(vec![
                CacheDirective::MaxAge(31622400),  // 1 year
                CacheDirective::Extension("immutable".into(), None),
            ])
        ),
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

