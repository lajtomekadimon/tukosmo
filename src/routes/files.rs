use actix_files::Files;


pub fn routes() -> Files {
    Files::new(
        // Website route
        "/files",
        // System dir
        "files",
    ).show_files_listing()
}
