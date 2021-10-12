use postgres::Error;


use crate::i18n::t::t;
use crate::database::error_codes as ec;


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
                ec::FIELD_IS_NOT_LANG_CODE =>
                    "ERROR:field_is_not_lang_code",
                ec::LANG_CODE_ALREADY_EXISTS =>
                    "ERROR:lang_code_already_exists",
                ec::SOME_WRONG_LANG_ID_OF_NAME =>
                    "ERROR:some_wrong_lang_id_of_name",
                ec::SOME_WRONG_LANG_NAME => "ERROR:some_wrong_lang_name",
                ec::USER_NOT_LOGGED_IN => "ERROR:user_not_logged_in",
                ec::WRONG_BODY_TEXT => "ERROR:wrong_body_text",
                ec::WRONG_DESCRIPTION => "ERROR:wrong_description",
                ec::WRONG_LANG_CODE => "ERROR:wrong_lang_code",
                ec::WRONG_LANG_ID => "ERROR:wrong_lang_id",
                ec::WRONG_PERMALINK => "ERROR:wrong_permalink",
                ec::WRONG_POST_ID => "ERROR:wrong_post_id",
                ec::WRONG_POST_PERMALINK => "ERROR:wrong_post_permalink",
                ec::WRONG_TITLE => "ERROR:wrong_title",
                ec::WRONG_USER_EMAIL => "ERROR:wrong_user_email",
                ec::WRONG_USER_PASSWORD => "ERROR:wrong_user_password",

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
                code: ec::UNKNOWN_ERROR.to_string(),
                message: t("ERROR:unknown_error", lang_code),
            }

        }

    } else {

        // Debugging
        println!("debugging: {}", e);

        ErrorDB {
            code: ec::UNKNOWN_ERROR.to_string(),
            message: t("ERROR:unknown_error", lang_code),
        }

    }

}
