
CREATE TYPE "ApiTagsNew" AS (
    req "AdminRequest",
    csrf_token UUID,
    lang_ids BIGINT[],
    tag_names TEXT[],
    tag_permalinks TEXT[]
);


CREATE OR REPLACE FUNCTION aha_p_tags_new(
    r "ApiTagsNew"
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    tag_id BIGINT;
    lang_id BIGINT;
    tag_name TEXT;
    tag_permalink TEXT;

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

    tag_id := i_tag(
        (d.userd).id
    );

    FOR i IN 1..ARRAY_LENGTH(r.lang_ids, 1) LOOP
        lang_id := r.lang_ids[i];
        tag_name := r.tag_names[i];
        tag_permalink := r.tag_permalinks[i];

        -- Check each language ID of each tag translation is correct
        IF NOT c_lang_by_id(lang_id) THEN
            PERFORM err_some_wrong_lang_id_of_name();
        END IF;

        -- Check each name is correct
        IF NOT e_is_tag_name(tag_name) THEN
            PERFORM err_some_wrong_tag_name();
        END IF;

        -- Check each permalink is correct
        IF NOT e_is_permalink(tag_permalink) THEN
            PERFORM err_some_wrong_tag_permalink();
        END IF;

        PERFORM i_tag_translation(
            tag_id,
            lang_id,
            tag_name,
            tag_permalink,
            (d.userd).id
        );

        /* IDEA: Insert all of them at once using a SELECT */
    END LOOP;

END;

$$;

