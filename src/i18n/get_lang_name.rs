use crate::i18n::t::t;


pub fn get_lang_name(
    language: &str,
    lang_code: &str,
) -> String {
    let string_to_translate = &"LANG:{code}"
        .replace("{code}", language);

    t(string_to_translate, lang_code)
}

