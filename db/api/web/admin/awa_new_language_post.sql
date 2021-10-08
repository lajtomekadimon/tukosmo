
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

    other_lang_id BIGINT;

    language_id BIGINT;
    lang_name TEXT;
    lang_id BIGINT;

BEGIN

    -- Check request
    PERFORM s_admin_handler_data(r.req);

    IF NOT e_is_lang_code(r.lang_code) THEN
        PERFORM err_field_is_not_lang_code();
    END IF;

    other_lang_id := s_language_id_by_code(r.lang_code);
    IF other_lang_id IS NOT NULL THEN
        IF other_lang_id <> r.language_id THEN
            PERFORM err_lang_code_already_exists();
        END IF;
    END IF;

    -- TODO: Check lang_ids

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

