
CREATE TYPE "NewLanguagePostARequest" AS (
    req "AdminRequest",
    lang_code TEXT,
    lang_ids BIGINT[],
    lang_names TEXT[]
);


CREATE OR REPLACE FUNCTION awa_new_language_post(
    r "NewLanguagePostARequest"
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    language_id BIGINT;
    lang_name TEXT;
    lang_id BIGINT;

BEGIN

    PERFORM s_admin_handler_data(r.req);

    language_id := i_language(r.lang_code);

    FOR i IN 1..ARRAY_LENGTH(r.lang_names, 1) LOOP
        lang_id := r.lang_ids[i];
        lang_name := r.lang_names[i];

        PERFORM i_language_name(
            language_id,
            lang_name,
            lang_id
        );
        /* IDEA: Insert all of them at once using a SELECT */
    END LOOP;

END;

$$;

