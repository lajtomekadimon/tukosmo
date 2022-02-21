use minify_html::{Cfg, minify};


pub fn render_html(
    html_input: &str,
) -> String {

    let mut cfg = Cfg {
        do_not_minify_doctype: true,
        ensure_spec_compliant_unquoted_attribute_values: true,
        keep_closing_tags: true,
        keep_html_and_head_opening_tags: true,
        keep_spaces_between_attributes: true,
        keep_comments: false,
        minify_css: false,
        minify_js: false,
        remove_bangs: false,
        remove_processing_instructions: false,
    };
    cfg.keep_comments = true;
    let minified = minify(&html_input.as_bytes(), &cfg);

    // Convert u8 vector to String
    match std::str::from_utf8(&minified) {
        Ok(v) => v.to_string(),
        Err(_e) => "ERROR".to_string(),
    }
}

