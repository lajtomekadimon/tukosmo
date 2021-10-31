use crate::i18n::translate_i18n::TranslateI18N;

use crate::i18n::languages::msg_en::MSG_EN;
use crate::i18n::languages::msg_es::MSG_ES;


pub fn t(
    lang_code: &str,
) -> TranslateI18N {

    match lang_code {
        "en" => MSG_EN,
        "es" => MSG_ES,

        //--------------//

        _ => MSG_EN,
    }

}


/*
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

        //_ => "[untranslated]"
        _ => translate_en(text_value),
    }.to_string()

}
*/
