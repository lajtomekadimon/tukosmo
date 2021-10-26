use crate::i18n::t::t;
use crate::database::error_codes as ec;


pub fn error_code_message(
    error_code: &str,
    lang_code: &str,
) -> String {

    let m = match error_code {
        ec::EMAIL_ALREADY_EXISTS => "ERROR:email_already_exists",
        ec::FIELD_IS_NOT_LANG_CODE => "ERROR:field_is_not_lang_code",
        ec::LANG_CODE_ALREADY_EXISTS => "ERROR:lang_code_already_exists",
        ec::PASSWORDS_DO_NOT_MATCH => "ERROR:passwords_do_not_match",
        ec::SOME_WRONG_I18N_USER_NAME => "ERROR:some_wrong_i18n_user_name",
        ec::SOME_WRONG_LANG_ID_OF_NAME => "ERROR:some_wrong_lang_id_of_name",
        ec::SOME_WRONG_LANG_NAME => "ERROR:some_wrong_lang_name",
        ec::SOME_WRONG_NAME_FOR_LANG => "ERROR:some_wrong_name_for_lang",
        ec::UNKNOWN_ERROR => "ERROR:unknown_error",
        ec::USER_NOT_LOGGED_IN => "ERROR:user_not_logged_in",
        ec::WRONG_BODY_TEXT => "ERROR:wrong_body_text",
        ec::WRONG_DESCRIPTION => "ERROR:wrong_description",
        ec::WRONG_EMAIL => "ERROR:wrong_email",
        ec::WRONG_LANG_CODE => "ERROR:wrong_lang_code",
        ec::WRONG_LANG_ID => "ERROR:wrong_lang_id",
        ec::WRONG_OWN_LANG_NAME => "ERROR:wrong_own_lang_name",
        ec::WRONG_PASSWORD => "ERROR:wrong_password",
        ec::WRONG_PERMALINK => "ERROR:wrong_permalink",
        ec::WRONG_POST_ID => "ERROR:wrong_post_id",
        ec::WRONG_POST_PERMALINK => "ERROR:wrong_post_permalink",
        ec::WRONG_TITLE => "ERROR:wrong_title",
        ec::WRONG_USER_EMAIL => "ERROR:wrong_user_email",
        ec::WRONG_USER_NAME => "ERROR:wrong_user_name",
        ec::WRONG_USER_PASSWORD => "ERROR:wrong_user_password",

        // ---- //

        _ => "ERROR:unknown_error",  // TODO: Known error without name
    };

    t(m, lang_code)

}
