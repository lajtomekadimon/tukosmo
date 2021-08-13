
use crate::i18n::translate_en::translate_en;
use crate::i18n::translate_es::translate_es;


pub fn t(
    text_value: &str,
    lang_code: &str,
) -> String {

    match lang_code {
        "en" => translate_en(text_value),
        "es" => translate_es(text_value),

        //--------------//

        _ => "[unstranslated]"
    }.to_string()

}
