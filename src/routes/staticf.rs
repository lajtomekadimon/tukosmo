use actix_files::Files;


pub fn routes() -> Files {
    Files::new(
        // Website route
        "/static",
        // System dir
        "static",
    ).show_files_listing()
}
