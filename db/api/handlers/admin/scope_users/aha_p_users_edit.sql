
CREATE TYPE "ApiUsersEdit" AS (
    req "AdminRequest",
    csrf_token UUID,
    id BIGINT,
    email TEXT,
    name TEXT,
    i18n_name_langs BIGINT[],
    i18n_names TEXT[]
);


CREATE OR REPLACE FUNCTION aha_p_users_edit(
    r "ApiUsersEdit"
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    language_of_user BIGINT;

    lang_id BIGINT;
    i18n_name TEXT;

BEGIN

    -- Check request
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Check CSRF token
    IF NOT c_csrf_token_by_token_session(
        r.csrf_token,
        (r.req).session
    ) THEN
        PERFORM err_wrong_csrf_token();
    END IF;

    -- Check user ID is correct
    IF s_user_by_id_lang(
        r.id,
        language_of_user
    ) IS NULL THEN
        PERFORM err_wrong_user_id();
    END IF;

    -- Check user name
    IF NOT e_is_user_name(r.name) THEN
        PERFORM err_wrong_user_name();
    END IF;

    -- Check email
    IF NOT e_is_email(r.email) THEN
        PERFORM err_wrong_email();
    END IF;

    -- Check email is unique
    IF c_user_by_email(r.email)
        AND r.id <> s_user_id_by_email(r.email)
    THEN
        PERFORM err_email_already_exists();
    END IF;

    PERFORM u_user_by_admin(
        r.id,
        r.email,
        r.name
    );

    FOR i IN 1..ARRAY_LENGTH(r.i18n_name_langs, 1) LOOP
        lang_id := r.i18n_name_langs[i];

        -- Check each language ID of each name is correct
        IF NOT c_lang_by_id(lang_id) THEN
            PERFORM err_some_wrong_lang_id_of_name();
        END IF;

        i18n_name := r.i18n_names[i];

        IF i18n_name <> '' THEN
            -- Check each name is correct
            IF NOT e_is_user_name(i18n_name) THEN
                PERFORM err_some_wrong_i18n_user_name();
            END IF;

            IF c_user_name_by_user_lang(r.id, lang_id) THEN
                PERFORM u_user_name(
                    r.id,
                    i18n_name,
                    lang_id
                );
            ELSE
                PERFORM i_user_name(
                    r.id,
                    i18n_name,
                    lang_id
                );
            END IF;
        ELSE
            IF c_user_name_by_user_lang(r.id, lang_id) THEN
                PERFORM d_user_name_by_user_lang(
                    r.id,
                    lang_id
                );
            END IF;
        END IF;
    END LOOP;

END;

$$;
