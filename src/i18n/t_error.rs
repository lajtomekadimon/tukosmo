use postgres::Error;


use crate::i18n::t::t;
use crate::database::error_codes as ec;
use crate::i18n::error_code_message::error_code_message;


pub struct ErrorDB {
    pub code: String,
    pub message: String,
}


pub fn t_error(
    e: Error,
    lang_code: &str,
) -> ErrorDB {

    let t = &t(lang_code);

    if let Some(dberror) = e.as_db_error() {

        let error_message = dberror.message();

        if &error_message[..8] == "TUKOSMO:" {

            // Extract X from "TUKOSMO:X"
            let error_code = &error_message[8..];

            ErrorDB {
                code: error_code.to_string(),
                message: error_code_message(
                    error_code,
                    lang_code,
                ).to_string(),
            }

        } else {

            // Debugging
            println!("debugging: {}", e);

            ErrorDB {
                code: ec::UNKNOWN_ERROR.to_string(),
                message: t.err_unknown_error.to_string(),
            }

        }

    } else {

        // Debugging
        println!("debugging: {}", e);

        ErrorDB {
            code: ec::UNKNOWN_ERROR.to_string(),
            message: t.err_unknown_error.to_string(),
        }

    }

}
