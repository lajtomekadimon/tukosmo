use crate::i18n::t::t;
use crate::database::error_codes as ec;


pub fn error_code_message(
    error_code: &str,
    lang_code: &str,
) -> &'static str {

    let t = &t(lang_code);

    match error_code {
        ec::CSRF_TOKEN_IS_NOT_A_VALID_UUID =>
            t.err_csrf_token_is_not_a_valid_uuid,
        ec::EMAIL_ALREADY_EXISTS => t.err_email_already_exists,
        ec::DEFAULT_LANGUAGE_CANT_BE_DELETED =>
            t.err_default_language_cant_be_deleted,
        ec::FEATURED_IMAGE_IS_NOT_IMAGE => t.err_featured_image_is_not_image,
        ec::FIELD_IS_NOT_LANG_CODE => t.err_field_is_not_lang_code,
        ec::FILE_TITLE_ALREADY_EXISTS => t.err_file_title_already_exists,
        ec::FILENAME_ALREADY_EXISTS => t.err_filename_already_exists,
        ec::LANG_CODE_ALREADY_EXISTS => t.err_lang_code_already_exists,
        ec::PASSWORDS_DO_NOT_MATCH => t.err_passwords_do_not_match,
        ec::SOME_WRONG_I18N_USER_NAME => t.err_some_wrong_i18n_user_name,
        ec::SOME_WRONG_LANG_ID_OF_NAME => t.err_some_wrong_lang_id_of_name,
        ec::SOME_WRONG_LANG_NAME => t.err_some_wrong_lang_name,
        ec::SOME_WRONG_NAME_FOR_LANG => t.err_some_wrong_name_for_lang,
        ec::UNKNOWN_ERROR => t.err_unknown_error,
        ec::USER_CANT_DELETE_ITSELF => t.err_user_cant_delete_itself,
        ec::USER_NOT_LOGGED_IN => t.err_user_not_logged_in,
        ec::UUID_IS_NOT_VALID => t.err_uuid_is_not_valid,
        ec::WRONG_BODY_TEXT => t.err_wrong_body_text,
        ec::WRONG_COPYRIGHT_OWNER => t.err_wrong_copyright_owner,
        ec::WRONG_CSRF_TOKEN => t.err_wrong_csrf_token,
        ec::WRONG_DESCRIPTION => t.err_wrong_description,
        ec::WRONG_DOMAIN => t.err_wrong_domain,
        ec::WRONG_EMAIL => t.err_wrong_email,
        ec::WRONG_FILE_ID => t.err_wrong_file_id,
        ec::WRONG_FILE_TITLE => t.err_wrong_file_title,
        ec::WRONG_FILENAME => t.err_wrong_filename,
        ec::WRONG_LANG_CODE => t.err_wrong_lang_code,
        ec::WRONG_LANG_ID => t.err_wrong_lang_id,
        ec::WRONG_OWN_LANG_NAME => t.err_wrong_own_lang_name,
        ec::WRONG_PAGE_NUMBER => t.err_wrong_page_number,
        ec::WRONG_PASSWORD => t.err_wrong_password,
        ec::WRONG_PERMALINK => t.err_wrong_permalink,
        ec::WRONG_POST_ID => t.err_wrong_post_id,
        ec::WRONG_POST_PERMALINK => t.err_wrong_post_permalink,
        ec::WRONG_RPP_NUMBER => t.err_wrong_rpp_number,
        ec::WRONG_TITLE => t.err_wrong_title,
        ec::WRONG_USER_EMAIL => t.err_wrong_user_email,
        ec::WRONG_USER_ID => t.err_wrong_user_id,
        ec::WRONG_USER_NAME => t.err_wrong_user_name,
        ec::WRONG_USER_PASSWORD => t.err_wrong_user_password,
        ec::WRONG_WEBSITE_SUBTITLE => t.err_wrong_website_subtitle,
        ec::WRONG_WEBSITE_TITLE => t.err_wrong_website_title,

        // ---- //

        _ => t.err_unknown_error,  // TODO: Known error without name
    }

}
