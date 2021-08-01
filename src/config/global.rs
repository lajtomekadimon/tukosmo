// TODO: Use better structure (struct)
/*
struct Config {
    db_auth: String,
    theme: String,
    theme_dir: String,
}

// TODO: Even better is using a custom config file on root dir.
*/

pub fn db_auth_string() -> &'static str {
    "host=localhost dbname=tukosmo user=tukosmouser password=1234"
}

/*pub fn current_theme() -> &'static str {
    "simple"
}*/

pub fn current_theme_dir() -> &'static str {
    "static/css/themes/simple.css"
}

