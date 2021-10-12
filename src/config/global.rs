// TODO: Use better structure (struct)
/*
struct Config {
    db_auth: String,
    theme: String,
    theme_dir: String,
}

// TODO: Even better is using a custom config file on root dir.
*/

pub const DB_AUTH_STRING: &'static str =
    "host=localhost dbname=tukosmo user=tukosmouser password=1234";

//pub const CURRENT_THEME: &'static str = "simple";
pub const CURRENT_THEME_DIR: &'static str = "static/css/themes/simple.css";

