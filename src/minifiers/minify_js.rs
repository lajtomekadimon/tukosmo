use std::fs;
use std::sync::Arc;
use crossbeam::sync::WaitGroup;
use esbuild_rs::{TransformOptionsBuilder, transform_direct, TransformResult};


fn minify_js_file(
    input_file: &'static str,
    output_file: &'static str,
) {
    let website_js = fs::read_to_string(input_file)
        .expect("Something went wrong reading the JS file!");

    let src = Arc::new(website_js.as_bytes().to_vec());

    let mut options_builder = TransformOptionsBuilder::new();
    options_builder.minify_whitespace = true;
    options_builder.minify_identifiers = true;
    options_builder.minify_syntax = true;
    let options = options_builder.build();

    let wg = WaitGroup::new();
    let task = wg.clone();
    transform_direct(src, options, move |TransformResult { 
        code,
        map: _,
        errors: _,
        warnings: _
    }| {
        fs::write(output_file, code.as_str())
            .expect("Error writing to the JS file.");
        drop(task);
    });
    wg.wait();
}



pub fn minify_js() {
    minify_js_file(
        "static/js/website.js",
        "static/bundle.js",
    );
    minify_js_file(
        "static/js/admin.js",
        "static/bundle.admin.js",
    );
}
