
CREATE OR REPLACE FUNCTION aw_edit_post(

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

    post_trans_id := u_post(
        post_id,
        lang_id,
        title_value,
        description_value,
        body_value,
        permalink_value,
        is_draft,
        is_deleted
    );

    RETURN post_trans_id;

END;

$$;
