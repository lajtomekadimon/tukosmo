use css_minify::optimizations::{Minifier, Level};
use std::fs;


fn theme_file_dir(
    current_theme: &str,
    file_dir: &str,
) -> String {
    let mut current_theme_dir = "static/css/themes/".to_string();
    current_theme_dir.push_str(current_theme);

    let mut current_theme_file_dir = current_theme_dir.clone();
    current_theme_file_dir.push_str(file_dir);

    fs::read_to_string(current_theme_file_dir)
        .expect("Something went wrong reading CSS theme file!")
}


pub fn minify_css(
    current_theme: &str,
) {
    // WEBSITE
    ////////////
    let normalize_css = fs::read_to_string("static/css/extra/normalize.css")
        .expect("Something went wrong reading normalize.css!");

    let mut css_to_minify = String::new();
    // TODO: Add license,etc comment like GNU Project proposes
    css_to_minify.push_str(&normalize_css);
    css_to_minify.push_str(&theme_file_dir(current_theme, "/html.css"));
    css_to_minify.push_str(&theme_file_dir(current_theme, "/site.css"));
    css_to_minify.push_str(&theme_file_dir(current_theme, "/widget.css"));
    css_to_minify.push_str(&theme_file_dir(current_theme, "/blog.css"));
    css_to_minify.push_str(&theme_file_dir(current_theme, "/post.css"));
    css_to_minify.push_str(&theme_file_dir(current_theme, "/error.css"));

    let minified_css = Minifier::default().minify(
        &css_to_minify,
        Level::One  // Note: Levels Two and Three are dangerous
    ).expect("CSS couldn't be minified!");

    fs::write("static/bundle.css", &minified_css)
        .expect("Error writing to bundle.css");

    // ADMIN
    //////////
    // TODO: Use bulma.css instead
    let bulma_css = fs::read_to_string("static/css/extra/bulma.min.css")
        .expect("Something went wrong reading bulma.css!");
    let admin_css = fs::read_to_string("static/css/admin.css")
        .expect("Something went wrong reading CSS admin file!");

    let mut css_to_minify2 = String::new();
    css_to_minify2.push_str(&bulma_css);
    css_to_minify2.push_str(&admin_css);

    let mut minified_css2 = Minifier::default().minify(
        &css_to_minify2,
        Level::One  // Note: Levels Two and Three are dangerous
    ).expect("CSS couldn't be minified!");

    let toastui_editor_css = fs::read_to_string(
        "static/css/extra/toastui-editor.min.css",
    ).expect("Something went wrong reading toastui-editor.min.css!");
    minified_css2.push_str(&toastui_editor_css);
    // TODO: Toast UI Editor can't be minified!?

    // TODO: Add license, etc. of everything in a comment like GNU proposes

    fs::write("static/bundle.admin.css", &minified_css2)
        .expect("Error writing to bundle.admin.css");
}
