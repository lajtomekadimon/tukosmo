use css_minify::optimizations::{Minifier, Level};
use std::fs;

use crate::config::global::current_theme_dir;


pub fn minify_css() {
    // WEBSITE
    ////////////
    let normalize_css = fs::read_to_string("static/css/normalize.css")
        .expect("Something went wrong reading normalize.css!");
    let theme_css = fs::read_to_string(current_theme_dir())
        .expect("Something went wrong reading CSS theme file!");

    let mut css_to_minify = String::new();
    css_to_minify.push_str(&normalize_css);
    css_to_minify.push_str(&theme_css);
    // TODO: Add license,etc comment like GNU Project proposes

    let minified_css = Minifier::default().minify(
        &css_to_minify,
        Level::One  // Note: Levels Two and Three are dangerous
    ).expect("CSS couldn't be minified!");

    fs::write("static/bundle.css", &minified_css)
        .expect("Error writing to bundle.css");

    // ADMIN
    //////////
    // TODO: Use bulma.css instead
    let bulma_css = fs::read_to_string("static/css/bulma.min.css")
        .expect("Something went wrong reading bulma.css!");
    let admin_css = fs::read_to_string("static/css/admin.css")
        .expect("Something went wrong reading CSS admin file!");

    let mut css_to_minify2 = String::new();
    css_to_minify2.push_str(&bulma_css);
    css_to_minify2.push_str(&admin_css);
    // TODO: Add license,etc comment like GNU Project proposes

    let minified_css2 = Minifier::default().minify(
        &css_to_minify2,
        Level::One  // Note: Levels Two and Three are dangerous
    ).expect("CSS couldn't be minified!");

    fs::write("static/bundle.admin.css", &minified_css2)
        .expect("Error writing to bundle.admin.css");
}
