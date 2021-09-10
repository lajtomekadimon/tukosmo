
CREATE OR REPLACE FUNCTION i_post(

    post_id BIGINT,

    lang_id BIGINT,

    title_value TEXT,

    description_value TEXT,

    body_value TEXT,

    permalink_value TEXT,

    translator_id BIGINT,

    is_draft BOOL

)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    this_post_id BIGINT;

BEGIN

    INSERT INTO t_posts (
        --tp_id,
        tp_post,
        tp_lang,
        tp_title,
        tp_description,
        tp_body,
        tp_permalink,
        tp_translator,
        --tp_date,
        tp_draft,
        tp_deleted
    ) VALUES (
        -- BIGSERIAL (autoincrement)
        post_id,
        lang_id,
        title_value,
        description_value,
        body_value,
        permalink_value,
        translator_id,
        --NOW(),
        is_draft,
        FALSE
    ) RETURNING tp_id INTO this_post_id;

    RETURN this_post_id;

END;

$$;

