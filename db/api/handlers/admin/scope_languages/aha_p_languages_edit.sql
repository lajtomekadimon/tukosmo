
CREATE TYPE "ApiLanguagesEdit" AS (
    req "AdminRequest",
    csrf_token UUID,
    language_id BIGINT,
    lang_code TEXT,
    lang_ids BIGINT[],
    lang_names TEXT[]
);


CREATE OR REPLACE FUNCTION aha_p_languages_edit(
    r "ApiLanguagesEdit"
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    other_lang_id BIGINT;

    lang_name TEXT;
    lang_id BIGINT;

    lang_name_id BIGINT;

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

    -- Check language ID is correct
    IF NOT c_lang_by_id(r.language_id) THEN
        PERFORM err_wrong_lang_id();
    END IF;

    -- Check language code is correct
    IF NOT e_is_lang_code(r.lang_code) THEN
        PERFORM err_field_is_not_lang_code();
    END IF;

    -- Check language code is unique
    other_lang_id := s_language_id_by_code(r.lang_code);
    IF other_lang_id IS NOT NULL THEN
        IF other_lang_id <> r.language_id THEN
            PERFORM err_lang_code_already_exists();
        END IF;
    END IF;

    PERFORM u_language(
        r.language_id,
        r.lang_code
    );

    FOR i IN 1..ARRAY_LENGTH(r.lang_names, 1) LOOP
        lang_id := r.lang_ids[i];
        lang_name := r.lang_names[i];

        -- Check each language ID of each name is correct
        IF NOT c_lang_by_id(lang_id) THEN
            PERFORM err_some_wrong_lang_id_of_name();
        END IF;

        -- Check each name is correct
        IF NOT e_is_lang_name(lang_name) THEN
            PERFORM err_some_wrong_lang_name();
        END IF;

        lang_name_id = s_lname_id_by_lang_nlang(
            r.language_id,
            lang_id
        );

        IF lang_name_id IS NULL THEN

            PERFORM i_language_name(
                r.language_id,
                lang_name,
                lang_id
            );

        ELSE

            PERFORM u_language_name(
                lang_name_id,
                lang_name
            );

        END IF;
    END LOOP;

END;

$$;
