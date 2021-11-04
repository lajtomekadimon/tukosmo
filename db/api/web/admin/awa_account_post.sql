
CREATE TYPE "AccountPostARequest" AS (
    req "AdminRequest",
    csrf_token UUID,
    email TEXT,
    name TEXT,
    password TEXT,
    new_password TEXT,
    repeat_password TEXT,
    i18n_name_langs BIGINT[],
    i18n_names TEXT[]
);


CREATE OR REPLACE FUNCTION awa_account_post(
    r "AccountPostARequest"
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    lang_id BIGINT;
    i18n_name TEXT;

    real_user_password TEXT;

BEGIN

    -- Check request
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
    IF c_user_by_email(r.email)
        AND (d.userd).id <> s_user_id_by_email(r.email)
    THEN
        PERFORM err_email_already_exists();
    END IF;

    real_user_password := s_user_password_by_email((d.userd).email);

    -- Password is correct
    /* TODO: Currently, password check is done here, in the
     * database. It may be more secure to do this
     * in the web server, but the downside is that
     * instead of one query, we would need to do two
     * queries from there: one for the password check, and
     * another one for the new session.
     */
    IF real_user_password = CRYPT(r.password, real_user_password) THEN

        IF r.new_password <> '' THEN

            -- Check password
            IF NOT e_is_secure_password(r.new_password) THEN
                PERFORM err_wrong_password();
            END IF;

            -- Check password and repeat password are equal
            IF r.new_password <> r.repeat_password THEN
                PERFORM err_passwords_do_not_match();
            END IF;

            PERFORM u_user(
                (d.userd).id,
                r.email,
                CRYPT(r.new_password, GEN_SALT('bf')),
                r.name
            );

        ELSE

            PERFORM u_user_by_admin(
                (d.userd).id,
                r.email,
                r.name
            );

        END IF;

    ELSE

        PERFORM err_wrong_user_password();

    END IF;

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

            IF c_user_name_by_user_lang((d.userd).id, lang_id) THEN
                PERFORM u_user_name(
                    (d.userd).id,
                    i18n_name,
                    lang_id
                );
            ELSE
                PERFORM i_user_name(
                    (d.userd).id,
                    i18n_name,
                    lang_id
                );
            END IF;
        ELSE
            IF c_user_name_by_user_lang((d.userd).id, lang_id) THEN
                PERFORM d_user_name_by_user_lang(
                    (d.userd).id,
                    lang_id
                );
            END IF;
        END IF;
    END LOOP;

END;

$$;
