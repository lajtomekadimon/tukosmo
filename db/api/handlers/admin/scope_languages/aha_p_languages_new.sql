
CREATE TYPE "ApiLanguagesNew" AS (
    req "AdminRequest",
    csrf_token UUID,
    lang_code TEXT,
    own_lang_name TEXT,
    lang_ids BIGINT[],
    lang_names TEXT[],
    names_for_langs TEXT[],
    website_title TEXT,
    website_subtitle TEXT,
    copyright_owner TEXT
);


CREATE OR REPLACE FUNCTION aha_p_languages_new(
    r "ApiLanguagesNew"
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    other_lang_id BIGINT;

    language_id BIGINT;
    lang_id BIGINT;
    lang_name TEXT;
    name_for_lang TEXT;

BEGIN

    -- Check request
    PERFORM s_admin_handler_data(r.req);

    -- Check CSRF token
    IF NOT c_csrf_token_by_token_session(
        r.csrf_token,
        (r.req).session
    ) THEN
        PERFORM err_wrong_csrf_token();
    END IF;

    -- Check language code is correct
    IF NOT e_is_lang_code(r.lang_code) THEN
        PERFORM err_field_is_not_lang_code();
    END IF;

    -- Check own language name is correct
    IF NOT e_is_lang_name(r.own_lang_name) THEN
        PERFORM err_wrong_own_lang_name();
    END IF;

    -- Check language code is unique
    other_lang_id := s_language_id_by_code(r.lang_code);
    IF other_lang_id IS NOT NULL THEN
        PERFORM err_lang_code_already_exists();
    END IF;

    -- Check website's title in the new language
    IF NOT e_is_website_title(r.website_title) THEN
        PERFORM err_wrong_website_title();
    END IF;

    -- Check website's subtitle in the new language
    IF NOT e_is_website_subtitle(r.website_subtitle) THEN
        PERFORM err_wrong_website_subtitle();
    END IF;

    -- Check copyright owner in the new language
    IF NOT e_is_copyright_owner(r.copyright_owner) THEN
        PERFORM err_wrong_copyright_owner();
    END IF;

    language_id := i_language(
        r.lang_code,
        r.website_title,
        r.website_subtitle,
        r.copyright_owner
    );

    PERFORM i_language_name(
        language_id,
        r.own_lang_name,
        language_id
    );

    FOR i IN 1..ARRAY_LENGTH(r.lang_ids, 1) LOOP
        lang_id := r.lang_ids[i];
        lang_name := r.lang_names[i];
        name_for_lang := r.names_for_langs[i];

        -- Check each language ID of each name is correct
        IF NOT c_lang_by_id(lang_id) THEN
            PERFORM err_some_wrong_lang_id_of_name();
        END IF;

        -- Check each name is correct
        IF NOT e_is_lang_name(lang_name) THEN
            PERFORM err_some_wrong_lang_name();
        END IF;

        -- Check each name (in the new language) for each language is correct
        IF NOT e_is_lang_name(name_for_lang) THEN
            PERFORM err_some_wrong_name_for_lang();
        END IF;

        PERFORM i_language_name(
            language_id,
            lang_name,
            lang_id
        );

        PERFORM i_language_name(
            lang_id,
            name_for_lang,
            language_id
        );

        /* IDEA: Insert all of them at once using a SELECT */
    END LOOP;

END;

$$;

