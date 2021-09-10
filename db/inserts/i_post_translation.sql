
CREATE OR REPLACE FUNCTION i_post_translation(

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

    INSERT INTO t_post_translations (
        --tpt_id,
        tpt_post,
        tpt_lang,
        tpt_title,
        tpt_description,
        tpt_body,
        tpt_permalink,
        tpt_translator,
        --tpt_date,
        tpt_draft,
        tpt_deleted
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
    ) RETURNING tpt_id INTO this_post_id;

    RETURN this_post_id;

END;

$$;

