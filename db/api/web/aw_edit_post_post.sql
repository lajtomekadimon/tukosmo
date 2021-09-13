
CREATE OR REPLACE FUNCTION aw_edit_post_post(

    post_id BIGINT,

    lang_code TEXT,

    title_value TEXT,

    description_value TEXT,

    body_value TEXT,

    permalink_value TEXT,

    is_draft BOOL,

    is_deleted BOOL,

    translator_id BIGINT

)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    post_trans_id BIGINT;

    lang_id BIGINT;

BEGIN

    lang_id := s_language_id_by_code(lang_code);

    -- Update existing post
    IF c_post_has_trans(
        post_id,
        lang_id
    ) THEN

        post_trans_id := u_post_translation(
            post_id,
            lang_id,
            title_value,
            description_value,
            body_value,
            permalink_value,
            is_draft,
            is_deleted
        );

    -- Create new translation of the post
    ELSE

        post_trans_id := i_post_translation(
            post_id,
            lang_id,
            title_value,
            description_value,
            body_value,
            permalink_value,
            translator_id,
            is_draft
        );

    END IF;

    RETURN post_trans_id;

END;

$$;
