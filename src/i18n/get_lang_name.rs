use crate::i18n::t::t;


pub fn get_lang_name(
    language: &str,
    lang_code: &str,
) -> &'static str {

    let t = &t(lang_code);

    match language {
        "en" => t.lang_en,
        "es" => t.lang_es,

        _ => t.lang_en,  // TODO!!!
    }

}

