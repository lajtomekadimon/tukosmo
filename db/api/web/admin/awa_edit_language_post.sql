
CREATE TYPE "EditLanguagePostARequest" AS (
    req "AdminRequest",
    language_id BIGINT,
    lang_code TEXT,
    lang_ids BIGINT[],
    lang_names TEXT[]
);


CREATE OR REPLACE FUNCTION awa_edit_language_post(
    r "EditLanguagePostARequest"
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    lang_name TEXT;
    lang_id BIGINT;

    lang_name_id BIGINT;

BEGIN

    IF s_admin_handler_data(r.req) IS NULL THEN

        RAISE EXCEPTION 'TODO: Wrong request or user not logged in.';

    END IF;

    PERFORM u_language(
        r.language_id,
        r.lang_code
    );

    FOR i IN 1..ARRAY_LENGTH(r.lang_names, 1) LOOP
        lang_id := r.lang_ids[i];
        lang_name := r.lang_names[i];

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
