
CREATE TYPE "ApiUsersNew" AS (
    req "AdminRequest",
    csrf_token UUID,
    name TEXT,
    email TEXT,
    password TEXT,
    repeat_password TEXT,
    i18n_name_langs BIGINT[],
    i18n_names TEXT[]
);


CREATE OR REPLACE FUNCTION aha_p_users_new(
    r "ApiUsersNew"
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    user_id BIGINT;

    lang_id BIGINT;
    i18n_name TEXT;

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    -- Check CSRF token
    IF NOT c_csrf_token_by_token_session(
        r.csrf_token,
        (r.req).session
    ) THEN
        PERFORM err_wrong_csrf_token();
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
    IF c_user_by_email(r.email) THEN
        PERFORM err_email_already_exists();
    END IF;

    -- Check password
    IF NOT e_is_secure_password(r.password) THEN
        PERFORM err_wrong_password();
    END IF;

    -- Check password and repeat password are equal
    IF r.password <> r.repeat_password THEN
        PERFORM err_passwords_do_not_match();
    END IF;

    user_id := i_user(
        r.email,
        CRYPT(r.password, GEN_SALT('bf')),
        r.name
    );

    FOR i IN 1..ARRAY_LENGTH(r.i18n_name_langs, 1) LOOP
        lang_id := r.i18n_name_langs[i];
        i18n_name := r.i18n_names[i];

        IF i18n_name <> '' THEN
            -- Check each language ID of each name is correct
            IF NOT c_lang_by_id(lang_id) THEN
                PERFORM err_some_wrong_lang_id_of_name();
            END IF;

            -- Check each name is correct
            IF NOT e_is_user_name(i18n_name) THEN
                PERFORM err_some_wrong_i18n_user_name();
            END IF;

            PERFORM i_user_name(
                user_id,
                i18n_name,
                lang_id
            );

            /* IDEA: Insert all of them at once using a SELECT */
        END IF;
    END LOOP;

END;

$$;
