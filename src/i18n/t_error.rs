use postgres::Error;
use crate::i18n::t::t;


pub struct ErrorDB {
    pub code: String,
    pub message: String,
}


pub fn t_error(
    e: Error,
    lang_code: &str,
) -> ErrorDB {

    if let Some(dberror) = e.as_db_error() {

        let error_message = dberror.message();

        if &error_message[..8] == "TUKOSMO:" {

            // Extract X from "TUKOSMO:X"
            let error_code = &error_message[8..];

            let m = match error_code {
                "C3BA3FC81BB9" => "ERROR:field_is_not_lang_code",
                "4C66AB9F871B" => "ERROR:lang_code_already_exists",
                "55F1A77CE041" => "ERROR:some_wrong_lang_id_of_name",
                "39464FAE6EEB" => "ERROR:some_wrong_lang_name",
                "D994FEF2356A" => "ERROR:user_not_logged_in",
                "F24F7F99E78D" => "ERROR:wrong_body_text",
                "4E576E6BB1EE" => "ERROR:wrong_description",
                "9F4297D0460F" => "ERROR:wrong_lang_code",
                "92165E44AFE4" => "ERROR:wrong_lang_id",
                "7D637808B8AA" => "ERROR:wrong_permalink",
                "E4E9108D630B" => "ERROR:wrong_post_id",
                "C90EAC6F8E2E" => "ERROR:wrong_post_permalink",
                "7B744BC3C936" => "ERROR:wrong_title",
                "9CF674E24CDC" => "ERROR:wrong_user_email",
                "C37F7C062377" => "ERROR:wrong_user_password",

                // ---- //

                _ => "ERROR:unknown_error",  // TODO: Known error without name
            };

            ErrorDB {
                code: error_code.to_string(),
                message: t(m, lang_code),
            }

        } else {

            // Debugging
            println!("debugging: {}", e);

            ErrorDB {
                code: "BCDD2EA230A5".to_string(),
                message: t("ERROR:unknown_error", lang_code),
            }

        }

    } else {

        // Debugging
        println!("debugging: {}", e);

        ErrorDB {
            code: "BCDD2EA230A5".to_string(),
            message: t("ERROR:unknown_error", lang_code),
        }

    }

}
