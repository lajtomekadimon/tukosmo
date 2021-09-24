
CREATE OR REPLACE FUNCTION awa_new_post_post(

    post_id BIGINT,

    lang_code TEXT,

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

    lang_id BIGINT;

BEGIN

    lang_id := s_language_id_by_code(lang_code);

    IF post_id = 0 THEN
        post_id := i_post(author_id);
    END IF;

    post_trans_id := i_post_translation(
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
