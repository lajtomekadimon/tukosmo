
CREATE TYPE "ApiTagsEdit" AS (
    req "AdminRequest",
    csrf_token UUID,
    tag_id BIGINT,
    lang_ids BIGINT[],
    tag_names TEXT[],
    tag_permalinks TEXT[]
);


CREATE OR REPLACE FUNCTION aha_p_tags_edit(
    r "ApiTagsEdit"
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
    tag_name TEXT;
    tag_permalink TEXT;

    tag_trans "TagDB";

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

    -- Check tag ID is correct
    IF NOT c_tag_by_id(r.tag_id) THEN
        PERFORM err_wrong_tag_id();
    END IF;

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

        tag_trans = s_tag_trans_by_id(
            lang_id,
            r.tag_id
        );

        IF tag_trans IS NULL THEN

            PERFORM i_tag_translation(
                r.tag_id,
                lang_id,
                tag_name,
                tag_permalink,
                (d.userd).id
            );

        ELSE

            PERFORM u_tag_translation(
                r.tag_id,
                lang_id,
                tag_name,
                tag_permalink
            );

        END IF;
    END LOOP;

END;

$$;
