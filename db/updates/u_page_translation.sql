
CREATE OR REPLACE FUNCTION u_page_translation(
    page_id BIGINT,
    lang_id BIGINT,
    title_value TEXT,
    body_value TEXT,
    permalink_value TEXT,
    author_id BIGINT,
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

    page_trans_id BIGINT;

BEGIN

    UPDATE t_page_translations
    SET tpt_title = title_value,
        tpt_body = body_value,
        tpt_permalink = permalink_value,
        tpt_draft = is_draft,
        tpt_deleted = is_deleted
    WHERE tpt_page = page_id
        AND tpt_lang = lang_id
    RETURNING tpt_id INTO page_trans_id;

    RETURN page_trans_id;

END;

$$;
