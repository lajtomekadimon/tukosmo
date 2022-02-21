use std::fs;
use std::sync::Arc;
use crossbeam::sync::WaitGroup;
use esbuild_rs::{TransformOptionsBuilder, transform_direct, TransformResult};


fn minify_js_file(
    input_code: String,
    output_file: String,
) {
    let src = Arc::new(input_code.as_bytes().to_vec());

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
    let website_js = fs::read_to_string("static/js/website.js")
        .expect("Something went wrong reading the JS file!");

    minify_js_file(
        website_js,
        "static/bundles/bundle.js".to_string(),
    );

    /*---*/

    let mut admin_js = String::new();

    // TODO: Function that adds an entire directory to some string
    // Extra
    let extra_paths = fs::read_dir("static/js/admin/extra/").unwrap();
    for path in extra_paths {
        let file_path = path.unwrap().path();

        if let Some(extension) = file_path.extension() {
            if extension == "js" {
                let file_code = fs::read_to_string(file_path)
                    .expect("Something went wrong reading the JS file!");

                admin_js.push_str(&file_code);
            }
        }
    }

    let file_paths = fs::read_dir("static/js/admin/eventlisteners/").unwrap();
    for path in file_paths {
        let file_path = path.unwrap().path();

        if let Some(extension) = file_path.extension() {
            if extension == "js" {
                let file_code = fs::read_to_string(file_path)
                    .expect("Something went wrong reading the JS file!");

                admin_js.push_str(&file_code);
            }
        }
    }

    minify_js_file(
        admin_js,
        "static/bundles/bundle.admin.js".to_string(),
    );
}

