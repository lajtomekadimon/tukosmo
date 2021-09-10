
CREATE OR REPLACE FUNCTION u_post_translation(

    post_id BIGINT,

    lang_id BIGINT,

    title_value TEXT,

    description_value TEXT,

    body_value TEXT,

    permalink_value TEXT,

    is_draft BOOL,

    is_deleted BOOL

)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    post_trans_id BIGINT;

BEGIN

    UPDATE t_post_translations
    SET tpt_title = title_value,
        tpt_description = description_value,
        tpt_body = body_value,
        tpt_permalink = permalink_value,
        tpt_draft = is_draft,
        tpt_deleted = is_deleted
    WHERE tpt_post = post_id
        AND tpt_lang = lang_id
    RETURNING tpt_id INTO post_trans_id;


    RETURN post_trans_id;

END;

$$;
