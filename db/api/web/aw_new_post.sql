
CREATE OR REPLACE FUNCTION aw_new_post(

    post_id BIGINT,

    lang_id BIGINT,

    title_value TEXT,

    description_value TEXT,

    body_value TEXT,

    permalink_value TEXT,

    author_id BIGINT,

    is_draft BOOL

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

    IF post_id = 0 THEN
        post_id := i_post_id(author_id);
    END IF;

    post_trans_id := i_post(
        post_id,
        lang_id,
        title_value,
        description_value,
        body_value,
        permalink_value,
        author_id,
        is_draft
    );

    RETURN post_trans_id;

END;

$$;
